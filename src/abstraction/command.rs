use crate::built_info;
use lazy_static::lazy_static;
use serenity::all::RoleId;

pub type CommandError = anyhow::Error;
pub type CommandContext<'a> = poise::Context<'a, CommandData, CommandError>;
pub type CommandResult = Result<(), CommandError>;
pub type CheckResult = Result<bool, CommandError>;

pub struct CommandData {
	pub client: reqwest::Client,
	pub playmatch_client: playmatch_client::Client,
}

impl Default for CommandData {
	fn default() -> Self {
		let client = reqwest::ClientBuilder::new()
			.user_agent(format!(
				"{}/{} ({})",
				built_info::PKG_NAME,
				built_info::PKG_VERSION,
				built_info::PKG_REPOSITORY
			))
			.build()
			.unwrap();

		Self {
			client: client.clone(),
			playmatch_client: playmatch_client::Client::new_with_client(
				"https://playmatch.retrorealm.dev",
				client,
			),
		}
	}
}

lazy_static! {
	pub static ref STAFF_ROLE_ID: u64 = std::env::var("DISCORD_RETROREALM_STAFF_ROLE_ID")
		.unwrap_or_default()
		.parse()
		.unwrap();
	pub static ref TRUSTED_ROLE_ID: u64 = std::env::var("DISCORD_RETROREALM_TRUSTED_ROLE_ID")
		.unwrap_or_default()
		.parse()
		.unwrap();
}

pub async fn is_user_trusted_or_above(ctx: CommandContext<'_>) -> CheckResult {
	let user = ctx.author();
	let member = ctx.author_member().await;

	// Check if user is owner
	if ctx.framework().options().owners.contains(&user.id) {
		return Ok(true);
	}

	if member.is_none() {
		return Ok(false);
	}

	let member = member.unwrap();

	// Check if user has the Staff or Trusted role
	if member.roles.iter().any(|role_id| {
		role_id == &RoleId::new(*STAFF_ROLE_ID) || role_id == &RoleId::new(*TRUSTED_ROLE_ID)
	}) {
		return Ok(true);
	}

	Ok(false)
}
