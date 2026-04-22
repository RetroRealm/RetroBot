use crate::abstraction::command::{CommandContext, CommandResult};
use crate::abstraction::components_v2::{self, Status};
use crate::command::{RETROREALM_SERVER_ID, UPDATE_ROLE_ID};
use log::warn;
use serenity::all::RoleId;
use std::str::FromStr;

/// Applies or removes the updates role for this user
#[poise::command(
	slash_command,
	category = "Role",
	rename = "toggle_update",
	required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES"
)]
pub async fn toggle_update_role(ctx: CommandContext<'_>) -> CommandResult {
	if UPDATE_ROLE_ID.is_empty() {
		ctx.send(components_v2::status_reply(
			Status::Error,
			"This command is not configured.",
		))
		.await?;
		warn!("UPDATE_ROLE_ID is not set, toggle_update_role command cannot be used!");
		return Ok(());
	}

	let member = ctx.author_member().await;

	if member.is_none() || ctx.guild_id().map(|g| g.get()) != Some(*RETROREALM_SERVER_ID) {
		ctx.send(components_v2::status_reply(
			Status::Error,
			"This command can only be run inside RetroRealm's Discord Server.",
		))
		.await?;
		return Ok(());
	}

	let member = member.unwrap_or_default();
	let role_id = RoleId::from_str(&UPDATE_ROLE_ID)?;

	if member.roles.contains(&role_id) {
		member.remove_role(ctx.http(), role_id, None).await?;
		ctx.send(components_v2::status_reply(
			Status::Success,
			"Updates role removed.",
		))
		.await?;
		return Ok(());
	}

	member.add_role(ctx.http(), role_id, None).await?;
	ctx.send(components_v2::status_reply(
		Status::Success,
		"Updates role added.",
	))
	.await?;

	Ok(())
}
