use crate::abstraction::command::CommandData;
use crate::command::SUGGESTION_CHANNEL_ID;
use crate::command::playmatch::{
	SuggestionMessageHandleData, SuggestionSubmitter, SuggestionType, handle_suggestion_message,
	mark_external_resolved,
};
use lazy_static::lazy_static;
use log::{info, warn};
use playmatch_client::types::Suggestion;
use serenity::all::{
	ActionRowComponent, ButtonKind, ChannelId, Component, ContainerComponent, Context, GetMessages,
	Http, Message, MessageId, TeamMemberRole, UserId,
};
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
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
const CHANNEL_SEED_SCAN_LIMIT: u8 = 100;

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

	seed_from_channel_if_needed(&http, channel_id, &data).await;

	// Reconcile once at startup: post anything new, edit anything resolved externally.
	if let Err(e) = reconcile(&ctx, &data, &owners).await {
		warn!("suggestion poller: initial reconcile failed: {e}");
	}

	// Re-arm collectors for everything currently tracked in Redis.
	let posted = match data.suggestion_store.list_all().await {
		Ok(p) => p,
		Err(e) => {
			warn!("suggestion poller: cannot list posted suggestions for collector re-arm: {e}");
			HashMap::new()
		}
	};
	for (uuid, msg_id) in posted {
		spawn_collector(ctx.clone(), data.clone(), owners.clone(), msg_id, uuid);
	}

	let mut tick = interval(Duration::from_secs(*POLL_INTERVAL_SECS));
	tick.tick().await; // initial reconcile already ran
	loop {
		tick.tick().await;
		if let Err(e) = reconcile(&ctx, &data, &owners).await {
			warn!("suggestion poller: reconcile failed: {e}");
		}
	}
}

/// One-time migration: if Redis has no entries but the channel does, scan the channel and
/// seed Redis with every suggestion UUID we can find. Avoids re-posting on first deploy.
async fn seed_from_channel_if_needed(http: &Http, channel_id: ChannelId, data: &Arc<CommandData>) {
	let empty = match data.suggestion_store.is_empty().await {
		Ok(v) => v,
		Err(e) => {
			warn!("suggestion poller: cannot check store emptiness, skipping seed: {e}");
			return;
		}
	};
	if !empty {
		return;
	}

	let messages = match channel_id
		.widen()
		.messages(http, GetMessages::new().limit(CHANNEL_SEED_SCAN_LIMIT))
		.await
	{
		Ok(m) => m,
		Err(e) => {
			warn!("suggestion poller: channel history scan for seeding failed: {e}");
			return;
		}
	};

	let mut seeded = 0usize;
	for msg in &messages {
		if let Some(uuid) = extract_suggestion_uuid(msg) {
			if let Err(e) = data.suggestion_store.mark_posted(uuid, msg.id).await {
				warn!("suggestion poller: seed mark_posted({uuid}) failed: {e}");
				continue;
			}
			seeded += 1;
		}
	}
	info!(
		"suggestion poller: seeded {} entries from {} channel messages",
		seeded,
		messages.len()
	);
}

/// Compare playmatch's pending queue against Redis. Post anything new, edit anything
/// that's been resolved outside Discord. Does not spawn collectors for new posts: the
/// `handle_suggestion_message` call wraps both the post and the collector wait.
async fn reconcile(
	ctx: &Context,
	data: &Arc<CommandData>,
	owners: &HashSet<UserId>,
) -> anyhow::Result<()> {
	let pending = data.playmatch_client.get_all_suggestions().await?;
	let pending_external: Vec<Suggestion> =
		pending.into_iter().filter(|s| s.source.is_some()).collect();
	let pending_uuids: HashSet<Uuid> = pending_external.iter().map(|s| s.id).collect();

	let posted = data.suggestion_store.list_all().await?;

	for s in pending_external {
		if posted.contains_key(&s.id) {
			continue;
		}
		spawn_new_post(ctx.clone(), data.clone(), owners.clone(), s);
	}

	let http = ctx.http.clone();
	for (uuid, msg_id) in &posted {
		if pending_uuids.contains(uuid) {
			continue;
		}
		if let Err(e) = mark_external_resolved(&http, *msg_id).await {
			warn!("suggestion poller: cannot edit message {msg_id} for {uuid}: {e}");
		}
		if let Err(e) = data.suggestion_store.remove(*uuid).await {
			warn!("suggestion poller: cannot remove {uuid} from store: {e}");
		}
	}

	Ok(())
}

fn spawn_new_post(
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
				warn!("suggestion poller: cannot build payload for {suggestion_id}: {e}");
				return;
			}
		};
		if let Err(e) = handle_suggestion_message(payload, None).await {
			warn!("suggestion poller: posting {suggestion_id} ended with error: {e}");
		}
	});
}

fn spawn_collector(
	ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	message_id: MessageId,
	suggestion_id: Uuid,
) {
	tokio::spawn(async move {
		let suggestion = match data
			.playmatch_client
			.get_suggestion_by_id(suggestion_id)
			.await
		{
			Ok(s) => s,
			Err(e) => {
				warn!(
					"suggestion poller: cannot fetch suggestion {suggestion_id} for collector re-arm: {e}"
				);
				return;
			}
		};
		let payload = match build_handle_data(ctx.clone(), data.clone(), owners, suggestion).await {
			Ok(p) => p,
			Err(e) => {
				warn!("suggestion poller: cannot build re-arm payload for {suggestion_id}: {e}");
				return;
			}
		};
		if let Err(e) = handle_suggestion_message(payload, Some(message_id)).await {
			warn!("suggestion poller: collector for {suggestion_id} ended with error: {e}");
		}
	});
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

async fn build_handle_data(
	serenity_ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	suggestion: Suggestion,
) -> anyhow::Result<SuggestionMessageHandleData> {
	let playmatch_client = data.playmatch_client.clone();
	let suggestion_store = data.suggestion_store.clone();

	let (kind, name, platform, company): (SuggestionType, String, Option<String>, Option<String>) =
		if let Some(game_id) = suggestion.game_id {
			let game = playmatch_client
				.get_playmatch_game_with_relations_by_id(game_id)
				.await?;
			(
				SuggestionType::Game,
				game.game.name.clone(),
				Some(game.platform.name.clone()),
				game.company.map(|c| c.name),
			)
		} else if let Some(company_id) = suggestion.company_id {
			let c = playmatch_client.get_company_by_id(company_id).await?;
			(SuggestionType::Company, c.name.clone(), None, None)
		} else if let Some(platform_id) = suggestion.platform_id {
			let p = playmatch_client.get_platform_by_id(platform_id).await?;
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
		suggestion_store,
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
