use crate::abstraction::command::{CommandContext, CommandResult};
use crate::abstraction::components_v2::{self, Card, Status};

/// Show this menu
#[poise::command(
	slash_command,
	category = "Util",
	required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn help(
	ctx: CommandContext<'_>,
	#[description = "Specific command to show help about"] command: Option<String>,
) -> CommandResult {
	let commands = &ctx.framework().options().commands;

	if let Some(query) = command.as_deref() {
		let query_lc = query.to_lowercase();
		let found = commands.iter().find(|c| {
			c.name.eq_ignore_ascii_case(&query_lc)
				|| c.qualified_name.eq_ignore_ascii_case(&query_lc)
		});

		if let Some(cmd) = found {
			let parameters = if cmd.parameters.is_empty() {
				"None".to_string()
			} else {
				cmd.parameters
					.iter()
					.map(|p| {
						let req = if p.required { "" } else { "?" };
						match p.description.as_deref() {
							Some(desc) => format!("`{}{}` {}", p.name, req, desc),
							None => format!("`{}{}`", p.name, req),
						}
					})
					.collect::<Vec<_>>()
					.join("\n")
			};

			let category = cmd
				.category
				.as_deref()
				.map(|s| s.to_string())
				.unwrap_or_else(|| "Uncategorized".to_string());
			let description = cmd
				.description
				.as_deref()
				.map(|s| s.to_string())
				.unwrap_or_else(|| "No description".to_string());

			ctx.send(
				Card::new(Status::Info, format!("/{}", cmd.qualified_name))
					.row("Category", category)
					.row("Description", description)
					.row("Parameters", parameters)
					.into_reply(),
			)
			.await?;
		} else {
			ctx.send(components_v2::status_reply(
				Status::Error,
				format!("No command named `{query}` found."),
			))
			.await?;
		}

		return Ok(());
	}

	let mut by_category: std::collections::BTreeMap<String, Vec<String>> =
		std::collections::BTreeMap::new();
	for cmd in commands {
		if cmd.hide_in_help {
			continue;
		}
		let category = cmd
			.category
			.as_deref()
			.map(|s| s.to_string())
			.unwrap_or_else(|| "Uncategorized".to_string());
		let line = match cmd.description.as_deref() {
			Some(desc) => format!("`/{}` {desc}", cmd.qualified_name),
			None => format!("`/{}`", cmd.qualified_name),
		};
		by_category.entry(category).or_default().push(line);
	}

	let mut card = Card::new(Status::Info, "RetroBot commands");
	for (category, lines) in by_category {
		card = card.section(category).text(lines.join("\n"));
	}
	card = card.footer("Type `/help <command>` for details on a specific command.");

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Responds with pong!
#[poise::command(
	slash_command,
	category = "Util",
	required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn ping(ctx: CommandContext<'_>) -> CommandResult {
	let handle = ctx
		.send(components_v2::status_reply(
			Status::Info,
			"Pinging Discord...",
		))
		.await?;

	let handle_message = handle.message().await?;

	let sent_timestamp = handle_message.timestamp.timestamp_millis();
	let original_timestamp = ctx.created_at().timestamp_millis();

	let rest_latency = sent_timestamp - original_timestamp;
	let gateway_latency = ctx.ping().await.map(|d| d.as_millis()).unwrap_or(0);

	handle
		.edit(
			ctx,
			components_v2::status_reply(
				Status::Success,
				format!(
					"Pong! Rest latency: {rest_latency}ms, Gateway latency: {gateway_latency}ms"
				),
			),
		)
		.await?;

	Ok(())
}
