use crate::abstraction::command::{CommandContext, CommandResult};
use poise::CreateReply;

/// Show this menu
#[poise::command(
	slash_command,
	category = "Util",
	required_permissions = "SEND_MESSAGES"
)]
pub async fn help(
	ctx: CommandContext<'_>,
	#[description = "Specific command to show help about"] command: Option<String>,
) -> CommandResult {
	let config = poise::builtins::HelpConfiguration {
		extra_text_at_bottom: "\
Type /help command for more info on a command.",
		..Default::default()
	};
	poise::builtins::help(ctx, command.as_deref(), config).await?;
	Ok(())
}

/// Responds with pong!
#[poise::command(
	slash_command,
	category = "Util",
	required_permissions = "SEND_MESSAGES"
)]
pub async fn ping(ctx: CommandContext<'_>) -> CommandResult {
	let handle = ctx.reply("Calculating....").await?;

	let handle_message = handle.message().await?;

	let sent_timestamp = handle_message.timestamp.timestamp_millis();
	let original_timestamp = ctx.created_at().timestamp_millis();

	let rest_latency = sent_timestamp - original_timestamp;
	let gateway_latency = ctx.ping().await.as_millis();

	handle
		.edit(
			ctx,
			CreateReply::default().content(format!(
				"Pong! Rest latency: {}ms, Gateway latency: {}ms",
				rest_latency, gateway_latency
			)),
		)
		.await?;

	Ok(())
}
