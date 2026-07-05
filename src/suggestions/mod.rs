//! Suggestion cards: playmatch's pending metadata suggestions, mirrored into a Discord
//! channel as staff-actionable Approve/Decline cards.
//!
//! - [`store`]: which suggestions have a card, keyed by UUID, in Redis.
//! - [`card`]: turning a suggestion into a Discord card.
//! - [`interaction`]: the Approve/Decline collector and resolution.
//! - [`dispatch`]: the rate-limited send/edit/delete workers.
//! - [`sync`]: the periodic reconcile that decides what to send/edit/delete.
//!
//! [`run`] wires them together; [`post_immediately`] is the fast path for `/suggest`.

pub mod card;
pub mod dispatch;
mod interaction;
pub mod store;
mod sync;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use log::{debug, info, warn};
use playmatch_client::types::Suggestion;
use serenity::all::{Context, Http, TeamMemberRole, UserId};
use tokio::time::interval;

use crate::abstraction::command::CommandData;
use crate::suggestions::card::SuggestionSubmitter;
use crate::suggestions::dispatch::Dispatcher;

fn poll_interval_secs() -> u64 {
	env::var("SUGGESTION_POLL_INTERVAL_SECS")
		.ok()
		.and_then(|v| v.parse().ok())
		.unwrap_or(300)
}

/// Start the suggestion system: resolve the bot owners (who may Approve/Decline), start the
/// rate-limited dispatcher, then reconcile playmatch against Discord on a loop. Meant to be
/// spawned once, from the `Ready` handler.
pub async fn run(ctx: Context, data: Arc<CommandData>) {
	let owners = match fetch_owners(ctx.http.as_ref()).await {
		Ok(o) => o,
		Err(e) => {
			warn!("suggestions: cannot resolve bot owners, not starting: {e}");
			return;
		}
	};

	let dispatcher = Dispatcher::start(ctx.clone(), data.clone(), owners);
	if data.dispatcher.set(dispatcher).is_err() {
		warn!("suggestions: already started, ignoring duplicate run()");
		return;
	}
	let dispatcher = data.dispatcher.get().expect("dispatcher just set");

	let interval_secs = poll_interval_secs();
	info!("suggestions: reconciling every {interval_secs}s");
	let mut tick = interval(Duration::from_secs(interval_secs));
	// The first successful pass is logged at info (the startup summary of what's pending /
	// to migrate); later passes drop to debug so steady-state runs don't spam.
	let mut first = true;
	loop {
		tick.tick().await;
		match sync::sync_once(&ctx, &data, dispatcher).await {
			Ok(summary) if first => {
				info!("suggestions: {summary}");
				first = false;
			}
			Ok(summary) => debug!("suggestions: {summary}"),
			Err(e) => warn!("suggestions: sync failed: {e}"),
		}
	}
}

/// Post a card for a just-created suggestion straight away, bypassing the rate-limited
/// queue. Used by `/suggest` so the submitter sees their card immediately. Goes through the
/// dispatcher so the post reserves the suggestion against the sync (no duplicate) and arms a
/// collector carrying the Discord author, so the resolution DM reaches them. Errors are
/// logged, not returned: the card is a side effect of the command, not its result.
pub async fn post_immediately(
	ctx: &Context,
	data: &Arc<CommandData>,
	suggestion: Suggestion,
	submitter: SuggestionSubmitter,
) {
	match data.dispatcher.get() {
		Some(dispatcher) => dispatcher.post_now(ctx, data, suggestion, submitter).await,
		// Before `run` sets the dispatcher (should not happen once Ready has fired). The next
		// sync will post this card, attributed as external.
		None => warn!(
			"suggestion {}: dispatcher not started, deferring to sync",
			suggestion.id
		),
	}
}

/// The users allowed to Approve/Decline: the app owner plus team admins and developers.
async fn fetch_owners(http: &Http) -> serenity::Result<HashSet<UserId>> {
	let app_info = http.get_current_application_info().await?;
	let mut owners = HashSet::new();
	if let Some(owner) = app_info.owner.as_ref() {
		owners.insert(owner.id);
	}
	if let Some(team) = app_info.team.as_ref() {
		for member in team.members.iter() {
			if matches!(member.role, TeamMemberRole::Admin | TeamMemberRole::Developer) {
				owners.insert(member.user.id);
			}
		}
	}
	Ok(owners)
}
