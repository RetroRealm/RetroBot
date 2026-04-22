use crate::abstraction::components_v2::{self, Status};
use crate::built_info;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use serenity::all::{
	ButtonStyle, ComponentInteractionCollector, CreateButton, CreateComponent,
	CreateInteractionResponse, CreateInteractionResponseMessage, MessageFlags, RoleId,
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
				env::var("PLAYMATCH_API_AUTH").expect("missing PLAYMATCH_API_AUTH")
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

	if ctx.framework().options().owners.contains(&user.id) {
		return Ok(true);
	}

	if member.is_none() {
		return Ok(false);
	}

	let member = member.unwrap();

	if member.roles.iter().any(|role_id| {
		role_id == &RoleId::new(*STAFF_ROLE_ID) || TRUSTED_ROLE_IDS.contains(&role_id.get())
	}) {
		return Ok(true);
	}

	ctx.send(components_v2::status_reply(
		Status::Error,
		"You do not have permission to use this command.",
	))
	.await?;

	Ok(false)
}

pub async fn paginate<U: Send + Sync + 'static, E>(
	ctx: poise::Context<'_, U, E>,
	pages: &[&str],
) -> Result<(), serenity::Error> {
	let ctx_id = ctx.id();
	let author_id = ctx.author().id;
	let prev_button_id = format!("{ctx_id}prev");
	let next_button_id = format!("{ctx_id}next");
	let ctx_id_str = ctx_id.to_string();
	let total = pages.len().max(1);

	let build_buttons = |prev_id: &str, next_id: &str| -> Vec<CreateButton<'static>> {
		vec![
			CreateButton::new(prev_id.to_owned())
				.emoji('◀')
				.style(ButtonStyle::Secondary),
			CreateButton::new(next_id.to_owned())
				.emoji('▶')
				.style(ButtonStyle::Secondary),
		]
	};

	let container = components_v2::paginate_container(
		pages[0].to_owned(),
		0,
		total,
		build_buttons(&prev_button_id, &next_button_id),
	);
	ctx.send(components_v2::reply_from_container(container))
		.await?;

	let mut current_page = 0usize;
	loop {
		let prefix = ctx_id_str.clone();
		let Some(press) = ComponentInteractionCollector::new(ctx.serenity_context())
			.timeout(Duration::from_secs(3600 * 24))
			.author_id(author_id)
			.filter(move |press| press.data.custom_id.starts_with(&prefix))
			.await
		else {
			break;
		};

		if press.data.custom_id == next_button_id {
			current_page = (current_page + 1) % total;
		} else if press.data.custom_id == prev_button_id {
			current_page = current_page.checked_sub(1).unwrap_or(total - 1);
		} else {
			continue;
		}

		let container = components_v2::paginate_container(
			pages[current_page].to_owned(),
			current_page,
			total,
			build_buttons(&prev_button_id, &next_button_id),
		);

		press
			.create_response(
				ctx.serenity_context().http.as_ref(),
				CreateInteractionResponse::UpdateMessage(
					CreateInteractionResponseMessage::new()
						.flags(MessageFlags::IS_COMPONENTS_V2)
						.components(vec![CreateComponent::Container(container)]),
				),
			)
			.await?;
	}

	Ok(())
}
