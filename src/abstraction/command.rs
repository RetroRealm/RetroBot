use crate::built_info;
use lazy_static::lazy_static;
use poise::CreateReply;
use poise::serenity_prelude::CreateActionRow;
use reqwest::header::HeaderMap;
use serenity::all::RoleId;
use serenity::builder::{
	CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use std::env;
use std::sync::Arc;
use std::time::Duration;

pub type CommandError = anyhow::Error;
pub type CommandContext<'a> = poise::Context<'a, CommandData, CommandError>;
pub type CommandResult = Result<(), CommandError>;
pub type CheckResult = Result<bool, CommandError>;

pub struct CommandData {
	pub client: reqwest::Client,
	pub playmatch_client: Arc<playmatch_client::Client>,
}

impl Default for CommandData {
	fn default() -> Self {
		let mut headers = HeaderMap::new();
		headers.insert(
			"Authorization",
			format!(
				"Bearer {}",
				std::env::var("PLAYMATCH_API_AUTH").expect("missing PLAYMATCH_API_AUTH")
			)
			.parse()
			.expect("Invalid Authorization header"),
		);

		let client = reqwest::ClientBuilder::new()
			.user_agent(format!(
				"{}/{} ({})",
				built_info::PKG_NAME,
				built_info::PKG_VERSION,
				built_info::PKG_REPOSITORY
			))
			.default_headers(headers)
			.build()
			.unwrap();

		Self {
			client: client.clone(),
			playmatch_client: Arc::new(playmatch_client::Client::new_with_client(
				&env::var("PLAYMATCH_API_URL")
					.unwrap_or("https://playmatch.retrorealm.dev".to_string()),
				client,
			)),
		}
	}
}

lazy_static! {
	pub static ref STAFF_ROLE_ID: u64 = env::var("DISCORD_RETROREALM_STAFF_ROLE_ID")
		.unwrap_or_default()
		.parse()
		.unwrap();
	pub static ref TRUSTED_ROLE_IDS: Vec<u64> = env::var("DISCORD_TRUSTED_ROLE_IDS")
		.unwrap_or_default()
		.split(",")
		.map(|id| id.trim().parse().unwrap())
		.collect();
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
		role_id == &RoleId::new(*STAFF_ROLE_ID) || TRUSTED_ROLE_IDS.contains(&role_id.get())
	}) {
		return Ok(true);
	}

	ctx.say("You do not have permission to use this command.")
		.await?;

	Ok(false)
}

pub async fn paginate<U, E>(
	ctx: poise::Context<'_, U, E>,
	pages: &[&str],
) -> Result<(), serenity::Error> {
	// Define some unique identifiers for the navigation buttons
	let ctx_id = ctx.id();
	let author_id = ctx.author().id;
	let prev_button_id = format!("{}prev", ctx_id);
	let next_button_id = format!("{}next", ctx_id);

	// Send the embed with the first page as content
	let reply = {
		let components = CreateActionRow::Buttons(vec![
			CreateButton::new(&prev_button_id).emoji('◀'),
			CreateButton::new(&next_button_id).emoji('▶'),
		]);

		CreateReply::default()
			.embed(CreateEmbed::default().description(pages[0]))
			.components(vec![components])
	};

	ctx.send(reply).await?;

	// Loop through incoming interactions with the navigation buttons
	let mut current_page = 0;
	while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
		// We defined our button IDs to start with `ctx_id`. If they don't, some other command's
		// button was pressed
		.filter(move |press| {
			press.data.custom_id.starts_with(&ctx_id.to_string()) && press.user.id == author_id
		})
		// Timeout when no navigation button has been pressed for 24 hours
		.timeout(Duration::from_secs(3600 * 24))
		.await
	{
		// Depending on which button was pressed, go to next or previous page
		if press.data.custom_id == next_button_id {
			current_page += 1;
			if current_page >= pages.len() {
				current_page = 0;
			}
		} else if press.data.custom_id == prev_button_id {
			current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
		} else {
			// This is an unrelated button interaction
			continue;
		}

		// Update the message with the new page contents
		press
			.create_response(
				ctx.serenity_context(),
				CreateInteractionResponse::UpdateMessage(
					CreateInteractionResponseMessage::new()
						.embed(CreateEmbed::new().description(pages[current_page])),
				),
			)
			.await?;
	}

	Ok(())
}
