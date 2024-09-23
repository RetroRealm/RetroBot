use crate::abstraction::command::{CommandContext, CommandData, CommandError, CommandResult};
use lazy_static::lazy_static;
use log::warn;
use poise::Command;
use serenity::all::RoleId;
use std::str::FromStr;

lazy_static! {
	static ref UPDATE_ROLE_ID: String =
		std::env::var("DISCORD_RETROREALM_UPDATE_ROLE_ID").unwrap_or_default();
}

pub fn get_commands() -> Vec<Command<CommandData, CommandError>> {
	vec![ping(), toggle_update_role(), help()]
}

/// Show this menu
#[poise::command(slash_command)]
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
#[poise::command(slash_command, required_permissions = "SEND_MESSAGES")]
async fn ping(ctx: CommandContext<'_>) -> CommandResult {
	ctx.say("pong!").await?;
	Ok(())
}

/// Applies or removes the updates role for this user
#[poise::command(
	slash_command,
	rename = "toggle_update",
	required_permissions = "SEND_MESSAGES | MANAGE_ROLES"
)]
async fn toggle_update_role(ctx: CommandContext<'_>) -> CommandResult {
	if UPDATE_ROLE_ID.is_empty() {
		ctx.say("This command is not configured").await?;
		warn!("UPDATE_ROLE_ID is not set, toggle_update_role command cannot be used!");
		return Ok(());
	}

	let member = ctx.author_member().await;

	if member.is_none() {
		ctx.say("This command can only be run inside of RetroRealm's Server")
			.await?;
		return Ok(());
	}

	let member = member.unwrap_or_default(); // safe to unwrap because we checked above

	let role_id = &RoleId::from_str(&UPDATE_ROLE_ID)?;

	if member.roles.contains(role_id) {
		member.remove_role(ctx, role_id).await?;

		ctx.say("Update Role Removed!").await?;
		return Ok(());
	}

	member.add_role(ctx, role_id).await?;
	ctx.say("Update Role Added!").await?;

	Ok(())
}
