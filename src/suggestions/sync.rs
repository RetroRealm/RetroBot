//! One reconciliation pass: read playmatch's pending queue and Redis, then hand every
//! needed Discord operation to the rate-limited `Dispatcher`. This module only reads and
//! decides; it never writes to Discord directly, so it can run as often as we like without
//! risking the rate limit.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use log::{info, warn};
use playmatch_client::types::{ExternalMetadata, MetadataMatchType, MetadataProvider, Suggestion};
use serenity::all::Context;
use uuid::Uuid;

use crate::abstraction::command::CommandData;
use crate::abstraction::playmatch::{ConciseError, describe};
use crate::suggestions::card::{LAYOUT_VERSION, SuggestionSubmitter};
use crate::suggestions::dispatch::Dispatcher;
use crate::suggestions::store::TrackedCard;

pub async fn sync_once(
	ctx: &Context,
	data: &Arc<CommandData>,
	dispatcher: &Dispatcher,
) -> anyhow::Result<()> {
	// Snapshot Redis before fetching pending. A card the /suggest path posts mid-sync would
	// otherwise appear in neither snapshot consistently; reading `posted` first keeps a
	// freshly-posted card off the delete list until it also shows up in `pending`.
	let posted = data.suggestion_store.list_all().await?;
	let mut pending = data.playmatch_client.get_all_suggestions().await.concise()?;

	// Tear down suggestions whose target already has an active provider match.
	let swept = sweep_redundant(data, dispatcher, &pending, &posted).await;

	// Oldest first, so the backlog posts in chronological order.
	pending.sort_by_key(|s| s.created_at);
	let pending_ids: HashSet<Uuid> = pending.iter().map(|s| s.id).collect();

	for suggestion in pending {
		if swept.contains(&suggestion.id) {
			continue;
		}
		match posted.get(&suggestion.id) {
			None => dispatcher.queue_send(suggestion),
			Some(card) if card.version == LAYOUT_VERSION => {
				let submitter = SuggestionSubmitter::from_source(suggestion.source.clone());
				dispatcher.arm(ctx, data, suggestion, card.message_id, submitter);
			}
			Some(card) => {
				// Outdated layout: arm now so the buttons work immediately, and queue the
				// (heavily rate-limited) refresh to catch the card up to the new layout.
				let submitter = SuggestionSubmitter::from_source(suggestion.source.clone());
				let message_id = card.message_id;
				dispatcher.arm(ctx, data, suggestion.clone(), message_id, submitter);
				dispatcher.queue_refresh(suggestion, message_id);
			}
		}
	}

	// Cards whose suggestion is no longer pending were resolved outside Discord: remove them.
	for (uuid, card) in &posted {
		if !pending_ids.contains(uuid) && !swept.contains(uuid) {
			dispatcher.queue_delete(*uuid, card.message_id);
		}
	}

	Ok(())
}

/// Dismiss game suggestions whose target already has an `Automatic`/`Manual` mapping for the
/// suggested provider: delete them on playmatch and queue their card for deletion. Returns
/// the set dismissed this pass so the caller skips them. All game ids are resolved in one
/// bulk lookup; individual failures are logged and skipped.
async fn sweep_redundant(
	data: &CommandData,
	dispatcher: &Dispatcher,
	pending: &[Suggestion],
	posted: &HashMap<Uuid, TrackedCard>,
) -> HashSet<Uuid> {
	let mut swept = HashSet::new();

	let game_suggestions: Vec<&Suggestion> =
		pending.iter().filter(|s| s.game_id.is_some()).collect();
	let distinct_ids: Vec<Uuid> = game_suggestions
		.iter()
		.filter_map(|s| s.game_id)
		.collect::<HashSet<_>>()
		.into_iter()
		.collect();
	if distinct_ids.is_empty() {
		return swept;
	}

	let metadata_by_game: HashMap<Uuid, Vec<ExternalMetadata>> =
		match data.playmatch_client.get_games_bulk(&distinct_ids).await {
			Ok(results) => results
				.into_iter()
				.filter_map(|r| r.data.map(|g| (r.id, g.external_metadata)))
				.collect(),
			Err(e) => {
				warn!("suggestion sweep: bulk game lookup failed: {}", describe(&e));
				return swept;
			}
		};

	for suggestion in game_suggestions {
		let Some(game_id) = suggestion.game_id else {
			continue;
		};
		let Some(metas) = metadata_by_game.get(&game_id) else {
			continue;
		};
		if !has_active_provider_match(metas, suggestion.provider) {
			continue;
		}

		// Playmatch is the source of truth; tear it down there first.
		if let Err(e) = data.playmatch_client.delete_suggestion(suggestion.id).await {
			warn!("suggestion sweep: delete_suggestion({}) failed: {}", suggestion.id, describe(&e));
			continue;
		}
		if let Some(card) = posted.get(&suggestion.id) {
			dispatcher.queue_delete(suggestion.id, card.message_id);
		}
		info!(
			"suggestion sweep: dismissed {} (game {game_id}, provider {})",
			suggestion.id, suggestion.provider
		);
		swept.insert(suggestion.id);
	}

	swept
}

fn has_active_provider_match(metas: &[ExternalMetadata], provider: MetadataProvider) -> bool {
	metas.iter().any(|m| {
		m.provider_name == provider
			&& matches!(m.match_type, MetadataMatchType::Automatic | MetadataMatchType::Manual)
	})
}
