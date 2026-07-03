use crate::abstraction::command::CommandData;
use crate::abstraction::playmatch::{ConciseError, describe};
use crate::abstraction::suggestion_store::TrackedCard;
use crate::command::SUGGESTION_CHANNEL_ID;
use crate::command::playmatch::{
	ExistingCard, SUGGESTION_CARD_LAYOUT_VERSION, SuggestionMessageHandleData, SuggestionSubmitter,
	SuggestionType, handle_suggestion_message,
};
use governor::{Quota, RateLimiter};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use playmatch_client::types::{ExternalMetadata, MetadataMatchType, MetadataProvider, Suggestion};
use reqwest::StatusCode;
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
	/// Paces the poller's bulk Discord writes to one per second. Suggestion cards all
	/// live in one channel, so every write shares one per-route bucket; 1/s keeps
	/// serenity's learned per-route limiter far from `remaining == 0` and produces zero
	/// 429s, so nothing feeds Discord's invalid-request ban counter. Interactive paths
	/// (approve/decline, DMs, command responses) never go through here and stay instant.
	static ref BULK_WRITE_PACER: RateLimiter<
		governor::state::NotKeyed,
		governor::state::InMemoryState,
		governor::clock::DefaultClock,
	> = RateLimiter::direct(bulk_write_quota());
}

/// The quota behind `BULK_WRITE_PACER`, split out so the constants are unit-testable.
fn bulk_write_quota() -> Quota {
	Quota::per_second(std::num::NonZeroU32::new(1).unwrap())
		.allow_burst(std::num::NonZeroU32::new(1).unwrap())
}

/// Blocks until the shared 1/s bucket has a permit. Maintenance-path Discord writes
/// only (re-arm refresh, reconcile posts/deletes, sweep deletes); never interactive
/// responses.
async fn pace_bulk_write() {
	BULK_WRITE_PACER.until_ready().await;
}

const APPROVE_PREFIX: &str = "approve:";
const DECLINE_PREFIX: &str = "decline:";
const CHANNEL_SEED_SCAN_LIMIT: u8 = 100;

/// A tracked card needs a re-render when its stored layout version is unknown (legacy or
/// seeded) or differs from the current layout.
fn needs_layout_refresh(stored: Option<u32>) -> bool {
	stored != Some(SUGGESTION_CARD_LAYOUT_VERSION)
}

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

	// Reconcile once at startup: post anything new, delete anything resolved externally.
	let pending = match reconcile(&ctx, &data, &owners).await {
		Ok(p) => p,
		Err(e) => {
			warn!("suggestion poller: initial reconcile failed: {e}");
			Vec::new()
		}
	};
	let mut by_id: HashMap<Uuid, Suggestion> = pending.into_iter().map(|s| (s.id, s)).collect();

	// Re-arm collectors for everything currently tracked in Redis.
	let posted = match data.suggestion_store.list_all().await {
		Ok(p) => p,
		Err(e) => {
			warn!("suggestion poller: cannot list posted suggestions for collector re-arm: {e}");
			HashMap::new()
		}
	};
	let mut armed = 0usize;
	let mut refreshing = 0usize;
	for (uuid, tracked) in posted {
		match by_id.remove(&uuid) {
			Some(s) => {
				if needs_layout_refresh(tracked.layout_version) {
					refreshing += 1;
				}
				spawn_collector(ctx.clone(), data.clone(), owners.clone(), tracked, s);
				armed += 1;
			}
			None => debug!("suggestion poller: {uuid} tracked but not pending, skipping re-arm"),
		}
	}
	info!(
		"suggestion poller: re-arming {armed} collectors ({refreshing} scheduled for layout v{SUGGESTION_CARD_LAYOUT_VERSION} refresh)"
	);

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
			// Seeded cards were posted by an older build, layout unknown ⇒ None ⇒ exactly
			// one paced refresh on the next boot.
			if let Err(e) = data.suggestion_store.mark_posted(uuid, msg.id, None).await {
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

/// Removes a resolved suggestion card from the channel. Discord 404 means a human
/// already deleted it, which is the outcome we wanted anyway.
async fn delete_suggestion_card(http: &Http, message_id: MessageId) -> serenity::Result<()> {
	let channel_id = ChannelId::new(*SUGGESTION_CHANNEL_ID);
	match channel_id
		.widen()
		.delete_message(http, message_id, None)
		.await
	{
		Err(serenity::Error::Http(e)) if e.status_code() == Some(StatusCode::NOT_FOUND) => Ok(()),
		other => other,
	}
}

/// Compare playmatch's pending queue against Redis. First tears down any suggestions
/// whose target game already has an active mapping for the suggested provider, then
/// posts anything new and deletes the card of anything resolved outside Discord. Does
/// not spawn collectors for new posts: the `handle_suggestion_message` call wraps
/// both the post and the collector wait. Returns the full fetched pending list so the
/// caller can re-arm collectors without a second round trip.
async fn reconcile(
	ctx: &Context,
	data: &Arc<CommandData>,
	owners: &HashSet<UserId>,
) -> anyhow::Result<Vec<Suggestion>> {
	let http = ctx.http.clone();

	// Freeze the delete-candidate set before fetching pending and running the
	// rate-limited sweep below. A user-submitted (source=None) suggestion created and
	// mark_posted'd during this same tick would otherwise show up in `posted` but not in
	// the pending snapshot, get its fresh card deleted, and never be reposted. Reading
	// `posted` first keeps it off the delete list this tick; by the next tick it is in
	// `pending`, so `pending_uuids` guards it.
	let posted = data.suggestion_store.list_all().await?;

	let pending = data
		.playmatch_client
		.get_all_suggestions()
		.await
		.concise()?;

	let cleanup = sweep_redundant_suggestions(&http, data, &pending).await;
	if !cleanup.cleaned.is_empty() {
		info!(
			"suggestion poller: cleaned up {} redundant suggestion(s) during reconcile",
			cleanup.cleaned.len()
		);
	}
	let cleaned: HashSet<Uuid> = cleanup.cleaned.iter().copied().collect();

	// Any suggestion still pending, whatever its source, must never be classified as
	// resolved externally, otherwise its live card would be deleted.
	let pending_uuids: HashSet<Uuid> = pending.iter().map(|s| s.id).collect();

	for s in &pending {
		if s.source.is_none() || cleaned.contains(&s.id) || posted.contains_key(&s.id) {
			continue;
		}
		spawn_new_post(ctx.clone(), data.clone(), owners.clone(), s.clone());
	}

	for (uuid, tracked) in &posted {
		if pending_uuids.contains(uuid) || cleaned.contains(uuid) {
			continue;
		}
		let msg_id = tracked.message_id;
		pace_bulk_write().await;
		match delete_suggestion_card(&http, msg_id).await {
			Ok(()) => {
				if let Err(e) = data.suggestion_store.remove(*uuid).await {
					warn!("suggestion poller: cannot remove {uuid} from store: {e}");
				}
			}
			// Keep the store entry so the next tick retries the delete.
			Err(e) => warn!("suggestion poller: cannot delete message {msg_id} for {uuid}: {e}"),
		}
	}

	Ok(pending)
}

/// Outcome of one sweep pass: how many game suggestions were inspected and which
/// ones got torn down because the target game already had an active mapping for
/// the suggested provider.
pub struct CleanupReport {
	pub checked: usize,
	pub cleaned: Vec<Uuid>,
}

/// Tear down any game-mapping suggestions in `pending` whose target game already
/// has an `Automatic` or `Manual` mapping for the suggested provider. For each
/// redundant suggestion: delete it on playmatch, delete the Discord card, and
/// remove the Redis entry.
///
/// All distinct game ids are resolved in a single bulk lookup (up to 100 ids per
/// request, chunked by the wrapper). Individual failures are logged and skipped so
/// one bad suggestion does not abort the rest.
pub async fn sweep_redundant_suggestions(
	http: &Http,
	data: &CommandData,
	pending: &[Suggestion],
) -> CleanupReport {
	let game_suggestions: Vec<&Suggestion> =
		pending.iter().filter(|s| s.game_id.is_some()).collect();
	let checked = game_suggestions.len();
	let mut cleaned: Vec<Uuid> = Vec::new();

	let distinct_ids: Vec<Uuid> = game_suggestions
		.iter()
		.filter_map(|s| s.game_id)
		.collect::<HashSet<_>>()
		.into_iter()
		.collect();
	if distinct_ids.is_empty() {
		return CleanupReport { checked, cleaned };
	}

	let metadata_by_game: HashMap<Uuid, Vec<ExternalMetadata>> =
		match data.playmatch_client.get_games_bulk(&distinct_ids).await {
			Ok(results) => results
				.into_iter()
				.filter_map(|r| r.data.map(|g| (r.id, g.external_metadata)))
				.collect(),
			Err(e) => {
				warn!(
					"suggestion cleanup: bulk game lookup failed: {}",
					describe(&e)
				);
				return CleanupReport { checked, cleaned };
			}
		};

	for suggestion in game_suggestions {
		let Some(game_id) = suggestion.game_id else {
			continue;
		};
		// A game deleted server-side is absent from the map; skip it like a failed fetch.
		let Some(metas) = metadata_by_game.get(&game_id) else {
			continue;
		};
		if !has_active_provider_match(metas, suggestion.provider) {
			continue;
		}

		// Playmatch is the source of truth, tear that down first.
		if let Err(e) = data.playmatch_client.delete_suggestion(suggestion.id).await {
			warn!(
				"suggestion cleanup: delete_suggestion({}) failed: {}",
				suggestion.id,
				describe(&e)
			);
			continue;
		}

		match data.suggestion_store.get(suggestion.id).await {
			Ok(Some(tracked)) => {
				let msg_id = tracked.message_id;
				pace_bulk_write().await;
				match delete_suggestion_card(http, msg_id).await {
					Ok(()) => {
						if let Err(e) = data.suggestion_store.remove(suggestion.id).await {
							warn!(
								"suggestion cleanup: redis remove {} failed: {e}",
								suggestion.id
							);
						}
					}
					// The next reconcile's externally-resolved pass retries this delete.
					Err(e) => warn!(
						"suggestion cleanup: delete message {msg_id} for {} failed: {e}",
						suggestion.id
					),
				}
			}
			Ok(None) => {}
			Err(e) => warn!(
				"suggestion cleanup: redis lookup {} failed: {e}",
				suggestion.id
			),
		}

		info!(
			"suggestion cleanup: dismissed {} (game {game_id}, provider {})",
			suggestion.id, suggestion.provider
		);
		cleaned.push(suggestion.id);
	}

	CleanupReport { checked, cleaned }
}

fn has_active_provider_match(metas: &[ExternalMetadata], provider: MetadataProvider) -> bool {
	metas.iter().any(|m| {
		m.provider_name == provider
			&& matches!(
				m.match_type,
				MetadataMatchType::Automatic | MetadataMatchType::Manual
			)
	})
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
		// Permit taken after the playmatch reads, immediately before the send inside
		// handle_suggestion_message.
		pace_bulk_write().await;
		if let Err(e) = handle_suggestion_message(payload, None).await {
			warn!("suggestion poller: posting {suggestion_id} ended with error: {e}");
		}
	});
}

fn spawn_collector(
	ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	tracked: TrackedCard,
	suggestion: Suggestion,
) {
	tokio::spawn(async move {
		let suggestion_id = suggestion.id;
		let payload = match build_handle_data(ctx.clone(), data.clone(), owners, suggestion).await {
			Ok(p) => p,
			Err(e) => {
				warn!("suggestion poller: cannot build re-arm payload for {suggestion_id}: {e}");
				return;
			}
		};
		let refresh = needs_layout_refresh(tracked.layout_version);
		// Only a refresh issues a Discord write, so only a refresh takes a permit. The
		// permit is acquired after build_handle_data's playmatch reads, immediately
		// before the edit inside handle_suggestion_message.
		if refresh {
			pace_bulk_write().await;
		}
		let existing = ExistingCard {
			message_id: tracked.message_id,
			refresh,
		};
		if let Err(e) = handle_suggestion_message(payload, Some(existing)).await {
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

/// The suggestion's target entity resolved for the card, plus the existing
/// provider matches already fetched alongside it.
struct SuggestionRelations {
	kind: SuggestionType,
	name: String,
	platform: Option<String>,
	company: Option<String>,
	existing_matches: Vec<crate::abstraction::playmatch::MatchSummary>,
}

async fn build_handle_data(
	serenity_ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	suggestion: Suggestion,
) -> anyhow::Result<SuggestionMessageHandleData> {
	let playmatch_client = data.playmatch_client.clone();
	let suggestion_store = data.suggestion_store.clone();

	let relations = if let Some(game_id) = suggestion.game_id {
		let game = playmatch_client
			.get_game_with_relations_by_id(game_id)
			.await
			.concise()?;
		SuggestionRelations {
			existing_matches: game.external_metadata.iter().map(Into::into).collect(),
			kind: SuggestionType::Game,
			name: game.game.name.clone(),
			platform: Some(game.platform.name.clone()),
			company: game.company.map(|c| c.name),
		}
	} else if let Some(company_id) = suggestion.company_id {
		let c = playmatch_client
			.get_company_by_id(company_id)
			.await
			.concise()?;
		SuggestionRelations {
			existing_matches: c.external_metadata.iter().map(Into::into).collect(),
			kind: SuggestionType::Company,
			name: c.name.clone(),
			platform: None,
			company: None,
		}
	} else if let Some(platform_id) = suggestion.platform_id {
		let p = playmatch_client
			.get_platform_by_id(platform_id)
			.await
			.concise()?;
		SuggestionRelations {
			existing_matches: p.external_metadata.iter().map(Into::into).collect(),
			kind: SuggestionType::Platform,
			name: p.name.clone(),
			platform: None,
			company: p.company_name.clone(),
		}
	} else {
		anyhow::bail!(
			"suggestion {} has no game/company/platform id",
			suggestion.id
		);
	};
	let SuggestionRelations {
		kind,
		name,
		platform,
		company,
		existing_matches,
	} = relations;

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
		provider_id: suggestion.provider_id,
		name,
		platform,
		company,
		comment: suggestion.comment,
		created_at: suggestion.created_at,
		existing_matches,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn layout_refresh_decision() {
		assert!(needs_layout_refresh(None));
		assert!(needs_layout_refresh(Some(
			SUGGESTION_CARD_LAYOUT_VERSION - 1
		)));
		assert!(!needs_layout_refresh(Some(SUGGESTION_CARD_LAYOUT_VERSION)));
	}

	#[test]
	fn bulk_write_quota_is_one_per_second() {
		let quota = bulk_write_quota();
		assert_eq!(quota.replenish_interval(), Duration::from_secs(1));
		assert_eq!(quota.burst_size().get(), 1);
	}
}
