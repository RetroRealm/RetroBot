use crate::abstraction::command::{CommandContext, CommandResult};
use crate::command::UPDATE_ROLE_ID;
use log::warn;
use serenity::all::RoleId;
use std::str::FromStr;

/// Applies or removes the updates role for this user
#[poise::command(
	slash_command,
	category = "Role",
	rename = "toggle_update",
	required_permissions = "SEND_MESSAGES | MANAGE_ROLES"
)]
pub async fn toggle_update_role(ctx: CommandContext<'_>) -> CommandResult {
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
