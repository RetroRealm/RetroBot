use crate::abstraction::command::{CommandContext, CommandResult};

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
	ctx.say("pong!").await?;
	Ok(())
}
