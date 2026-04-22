use crate::abstraction::command::{CommandContext, CommandResult};
use crate::abstraction::components_v2::{self, Status};

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
			let params = if cmd.parameters.is_empty() {
				"—".to_string()
			} else {
				cmd.parameters
					.iter()
					.map(|p| {
						let req = if p.required { "" } else { "?" };
						format!("`{}{}`", p.name, req)
					})
					.collect::<Vec<_>>()
					.join(" ")
			};

			let category = cmd
				.category
				.as_deref()
				.map(|s| s.to_string())
				.unwrap_or_else(|| "—".to_string());
			let description = cmd
				.description
				.as_deref()
				.map(|s| s.to_string())
				.unwrap_or_else(|| "—".to_string());

			let rows = vec![
				("Category".to_string(), category),
				("Description".to_string(), description),
				("Parameters".to_string(), params),
			];

			ctx.send(components_v2::card_reply(
				Status::Info,
				format!("/{}", cmd.qualified_name),
				rows,
			))
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
			Some(desc) => format!("`/{}` — {desc}", cmd.qualified_name),
			None => format!("`/{}`", cmd.qualified_name),
		};
		by_category.entry(category).or_default().push(line);
	}

	let rows: Vec<(String, String)> = by_category
		.into_iter()
		.map(|(cat, lines)| (cat, lines.join("\n")))
		.collect();

	ctx.send(components_v2::card_reply(
		Status::Info,
		"RetroBot commands",
		rows,
	))
	.await?;

	ctx.send(components_v2::status_reply(
		Status::Info,
		"Type `/help <command>` for details on a specific command.",
	))
	.await?;

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
		.send(components_v2::status_reply(Status::Info, "Calculating..."))
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
