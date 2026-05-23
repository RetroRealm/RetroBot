use crate::abstraction::command::CommandData;
use crate::command::SUGGESTION_CHANNEL_ID;
use crate::command::playmatch::{
	SuggestionMessageHandleData, SuggestionSubmitter, SuggestionType, handle_suggestion_message,
};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use playmatch_client::types::Suggestion;
use serenity::all::{
	ActionRowComponent, ButtonKind, ChannelId, Component, ContainerComponent, Context, GetMessages,
	Http, Message, MessageId, TeamMemberRole, UserId,
};
use std::collections::HashSet;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;
use uuid::Uuid;

lazy_static! {
	static ref POLL_INTERVAL_SECS: u64 = env::var("SUGGESTION_POLL_INTERVAL_SECS")
		.ok()
		.and_then(|v| v.parse().ok())
		.unwrap_or(300);
}

const APPROVE_PREFIX: &str = "approve:";
const DECLINE_PREFIX: &str = "decline:";
const CHANNEL_SCAN_LIMIT: u8 = 100;

pub async fn run(ctx: Context, data: Arc<CommandData>) {
	info!(
		"Suggestion poller starting (interval = {}s)",
		*POLL_INTERVAL_SECS
	);

	let http = ctx.http.clone();

	let owners = match fetch_owners(&http).await {
		Ok(o) => o,
		Err(e) => {
			warn!("suggestion poller: failed to resolve bot owners, aborting: {e}");
			return;
		}
	};

	let channel_id = ChannelId::new(*SUGGESTION_CHANNEL_ID);
	let seen: Arc<Mutex<HashSet<Uuid>>> = Arc::new(Mutex::new(HashSet::new()));

	match channel_id
		.widen()
		.messages(&http, GetMessages::new().limit(CHANNEL_SCAN_LIMIT))
		.await
	{
		Ok(messages) => {
			let mut recovery_count = 0usize;
			for msg in &messages {
				if let Some(uuid) = extract_suggestion_uuid(msg) {
					seen.lock().unwrap().insert(uuid);
					spawn_recovery(ctx.clone(), data.clone(), owners.clone(), msg.id, uuid);
					recovery_count += 1;
				}
			}
			info!(
				"suggestion poller: scanned {} channel messages, found {} suggestion cards",
				messages.len(),
				recovery_count
			);
		}
		Err(e) => {
			warn!("suggestion poller: channel history scan failed: {e}");
		}
	}

	// The first tick fires immediately so we catch up on suggestions submitted while the
	// bot was offline. Subsequent ticks wait the full interval.
	let mut tick = interval(Duration::from_secs(*POLL_INTERVAL_SECS));
	loop {
		tick.tick().await;

		let pending = match data.playmatch_client.get_all_suggestions().send().await {
			Ok(r) => r.into_inner(),
			Err(e) => {
				warn!("suggestion poller: get_all_suggestions failed: {e}");
				continue;
			}
		};

		for s in pending {
			if s.source.is_none() {
				continue;
			}
			{
				let mut guard = seen.lock().unwrap();
				if guard.contains(&s.id) {
					continue;
				}
				guard.insert(s.id);
			}
			spawn_external_post(ctx.clone(), data.clone(), owners.clone(), s);
		}
	}
}

async fn fetch_owners(http: &Http) -> serenity::Result<HashSet<UserId>> {
	let app_info = http.get_current_application_info().await?;
	let mut owners = HashSet::new();
	if let Some(owner) = app_info.owner.as_ref() {
		owners.insert(owner.id);
	}
	if let Some(team) = app_info.team.as_ref() {
		for member in team.members.iter() {
			if matches!(
				member.role,
				TeamMemberRole::Admin | TeamMemberRole::Developer
			) {
				owners.insert(member.user.id);
			}
		}
	}
	Ok(owners)
}

fn extract_suggestion_uuid(msg: &Message) -> Option<Uuid> {
	msg.components.iter().find_map(walk_component)
}

fn walk_component(c: &Component) -> Option<Uuid> {
	match c {
		Component::Container(container) => container.components.iter().find_map(|cc| match cc {
			ContainerComponent::ActionRow(row) => find_button_uuid_in_row(row),
			_ => None,
		}),
		Component::ActionRow(row) => find_button_uuid_in_row(row),
		_ => None,
	}
}

fn find_button_uuid_in_row(row: &serenity::all::ActionRow) -> Option<Uuid> {
	for arc in row.components.iter() {
		if let ActionRowComponent::Button(btn) = arc
			&& let ButtonKind::NonLink { custom_id, .. } = &btn.data
		{
			let rest = custom_id
				.strip_prefix(APPROVE_PREFIX)
				.or_else(|| custom_id.strip_prefix(DECLINE_PREFIX));
			if let Some(uuid_str) = rest
				&& let Ok(uuid) = Uuid::parse_str(uuid_str)
			{
				return Some(uuid);
			}
		}
	}
	None
}

fn spawn_recovery(
	ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	message_id: MessageId,
	suggestion_id: Uuid,
) {
	tokio::spawn(async move {
		let suggestion = match data
			.playmatch_client
			.get_suggestion_by_id()
			.id(suggestion_id)
			.send()
			.await
		{
			Ok(r) => r.into_inner(),
			Err(e) => {
				debug!(
					"suggestion poller: recovery skipped, suggestion {suggestion_id} no longer pending: {e}"
				);
				return;
			}
		};

		let payload = match build_handle_data(ctx.clone(), data.clone(), owners, suggestion).await {
			Ok(p) => p,
			Err(e) => {
				warn!("suggestion poller: cannot build recovery payload for {suggestion_id}: {e}");
				return;
			}
		};

		if let Err(e) = handle_suggestion_message(payload, Some(message_id)).await {
			warn!(
				"suggestion poller: recovery collector for {suggestion_id} ended with error: {e}"
			);
		}
	});
}

fn spawn_external_post(
	ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	suggestion: Suggestion,
) {
	tokio::spawn(async move {
		let suggestion_id = suggestion.id;
		let payload = match build_handle_data(ctx.clone(), data.clone(), owners, suggestion).await {
			Ok(p) => p,
			Err(e) => {
				warn!("suggestion poller: cannot post external suggestion {suggestion_id}: {e}");
				return;
			}
		};
		if let Err(e) = handle_suggestion_message(payload, None).await {
			warn!("suggestion poller: external suggestion {suggestion_id} ended with error: {e}");
		}
	});
}

async fn build_handle_data(
	serenity_ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	suggestion: Suggestion,
) -> anyhow::Result<SuggestionMessageHandleData> {
	let playmatch_client = data.playmatch_client.clone();

	let (kind, name, platform, company): (SuggestionType, String, Option<String>, Option<String>) =
		if let Some(game_id) = suggestion.game_id {
			let resp = playmatch_client
				.get_playmatch_game_with_relations_by_id()
				.id(game_id)
				.send()
				.await?;
			let game = resp.into_inner();
			(
				SuggestionType::Game,
				game.game.name.clone(),
				Some(game.platform.name.clone()),
				game.company.map(|c| c.name),
			)
		} else if let Some(company_id) = suggestion.company_id {
			let resp = playmatch_client
				.get_company_by_id()
				.id(company_id)
				.send()
				.await?;
			let c = resp.into_inner();
			(SuggestionType::Company, c.name.clone(), None, None)
		} else if let Some(platform_id) = suggestion.platform_id {
			let resp = playmatch_client
				.get_platform_by_id()
				.id(platform_id)
				.send()
				.await?;
			let p = resp.into_inner();
			(
				SuggestionType::Platform,
				p.name.clone(),
				None,
				p.company_name.clone(),
			)
		} else {
			anyhow::bail!(
				"suggestion {} has no game/company/platform id",
				suggestion.id
			);
		};

	let submitter = match suggestion.source.clone() {
		Some(source) => SuggestionSubmitter::External { source },
		None => SuggestionSubmitter::External {
			source: "unknown".to_string(),
		},
	};

	Ok(SuggestionMessageHandleData {
		playmatch_client,
		serenity_ctx,
		suggestion_id: suggestion.id,
		owners,
		submitter,
		r#type: kind,
		provider: suggestion.provider,
		name,
		platform,
		company,
		comment: suggestion.comment,
	})
}
