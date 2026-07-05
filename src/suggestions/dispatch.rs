//! Rate-limited delivery of card operations to Discord.
//!
//! Every card the sync wants sent, refreshed or deleted is queued here and drained by a
//! background worker at a steady per-hour rate (governor with burst 1, so the budget is
//! spread evenly across the hour rather than bursting up front). Sends and edits have
//! separate queues and rates. A shared `queued` set makes enqueuing idempotent so repeated
//! sync ticks never double-post; a shared `armed` map makes collector arming idempotent (a
//! card never gets two collectors) and holds each collector's task handle so it can be
//! aborted when the card goes away and re-armed if it exits without resolving.

use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use log::{info, warn};
use playmatch_client::types::Suggestion;
use reqwest::StatusCode;
use serenity::all::{ChannelId, Context, MessageId, UserId};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::abstraction::command::CommandData;
use crate::command::SUGGESTION_CHANNEL_ID;
use crate::suggestions::card::{
	CardData, LAYOUT_VERSION, RenderContext, SuggestionSubmitter, build_card_data,
};
use crate::suggestions::interaction;

type DirectLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

fn per_hour(var: &str, default: u32) -> u32 {
	env::var(var)
		.ok()
		.and_then(|v| v.parse().ok())
		.unwrap_or(default)
		.max(1)
}

/// A steady limiter that hands out `per_hour` permits an hour, evenly spaced (burst 1).
fn hourly_limiter(per_hour: u32) -> DirectLimiter {
	let period = Duration::from_secs_f64(3600.0 / per_hour as f64);
	RateLimiter::direct(Quota::with_period(period).expect("period is non-zero"))
}

/// An operation on an existing card, drained by the edit worker.
enum EditJob {
	Refresh {
		suggestion: Suggestion,
		message_id: MessageId,
	},
	Delete {
		uuid: Uuid,
		message_id: MessageId,
	},
}

impl EditJob {
	fn uuid(&self) -> Uuid {
		match self {
			EditJob::Refresh { suggestion, .. } => suggestion.id,
			EditJob::Delete { uuid, .. } => *uuid,
		}
	}
}

#[derive(Default)]
struct Managed {
	/// UUIDs with a queued-but-not-yet-executed send/edit/delete job.
	queued: Mutex<HashSet<Uuid>>,
	/// UUIDs with a live Approve/Decline collector, mapped to its task handle so the
	/// collector can be aborted when the card is deleted.
	armed: Mutex<HashMap<Uuid, JoinHandle<()>>>,
}

/// Owns the queues and the shared dedup state. Held in `CommandData` so both the sync loop
/// and the immediate `/suggest` path enqueue through the same limiters and share one
/// `queued`/`armed` set. Deliberately holds no `Context`/`Arc<CommandData>` back-reference
/// (that would be a cycle); callers pass those to `arm` per call, and the workers own their
/// own clones.
pub struct Dispatcher {
	owners: HashSet<UserId>,
	send_tx: mpsc::UnboundedSender<Suggestion>,
	edit_tx: mpsc::UnboundedSender<EditJob>,
	managed: Arc<Managed>,
}

impl Dispatcher {
	pub fn start(ctx: Context, data: Arc<CommandData>, owners: HashSet<UserId>) -> Self {
		let send_rate = per_hour("SUGGESTION_SEND_PER_HOUR", 250);
		let edit_rate = per_hour("SUGGESTION_EDIT_PER_HOUR", 50);
		info!("suggestion dispatcher: sends {send_rate}/h, edits {edit_rate}/h");

		let (send_tx, send_rx) = mpsc::unbounded_channel();
		let (edit_tx, edit_rx) = mpsc::unbounded_channel();
		let managed = Arc::new(Managed::default());

		tokio::spawn(send_worker(
			ctx.clone(),
			data.clone(),
			owners.clone(),
			managed.clone(),
			send_rx,
			hourly_limiter(send_rate),
		));
		tokio::spawn(edit_worker(
			ctx.clone(),
			data.clone(),
			managed.clone(),
			edit_rx,
			hourly_limiter(edit_rate),
		));

		Self {
			owners,
			send_tx,
			edit_tx,
			managed,
		}
	}

	/// Queue a new card. No-op if this suggestion already has a job in flight.
	pub fn queue_send(&self, suggestion: Suggestion) {
		if self.managed.queued.lock().unwrap().insert(suggestion.id) {
			let _ = self.send_tx.send(suggestion);
		}
	}

	/// Queue a layout refresh of an existing card. No-op if a job is already in flight.
	pub fn queue_refresh(&self, suggestion: Suggestion, message_id: MessageId) {
		if self.managed.queued.lock().unwrap().insert(suggestion.id) {
			let _ = self.edit_tx.send(EditJob::Refresh {
				suggestion,
				message_id,
			});
		}
	}

	/// Queue deletion of a card whose suggestion is gone. No-op if a job is already in flight.
	pub fn queue_delete(&self, uuid: Uuid, message_id: MessageId) {
		if self.managed.queued.lock().unwrap().insert(uuid) {
			let _ = self.edit_tx.send(EditJob::Delete { uuid, message_id });
		}
	}

	/// Attach a collector to an already-posted card. No-op if one is already armed.
	pub fn arm(
		&self,
		ctx: &Context,
		data: &Arc<CommandData>,
		suggestion: Suggestion,
		message_id: MessageId,
		submitter: SuggestionSubmitter,
	) {
		arm(
			&self.managed,
			ctx,
			data,
			&self.owners,
			suggestion,
			message_id,
			submitter,
		);
	}

	/// Post a card for a just-created suggestion immediately, bypassing the send limiter, and
	/// arm it. Reserves the UUID in `queued` for the duration so a concurrent sync tick can't
	/// queue a duplicate send. Used by `/suggest`, which carries the Discord author as the
	/// submitter so the resolution DM reaches them.
	pub async fn post_now(
		&self,
		ctx: &Context,
		data: &Arc<CommandData>,
		suggestion: Suggestion,
		submitter: SuggestionSubmitter,
	) {
		let id = suggestion.id;
		if !self.managed.queued.lock().unwrap().insert(id) {
			return; // already being handled by the sync path
		}

		let card_data =
			match build_card_data(&data.playmatch_client, &suggestion, submitter.clone()).await {
				Ok(d) => Some(d),
				Err(e) => {
					warn!("suggestion {id}: cannot build card data: {e}");
					None
				}
			};
		if let Some(card_data) = card_data
			&& let Some(message_id) = send_staff_card(ctx, data, &card_data).await
		{
			arm(
				&self.managed,
				ctx,
				data,
				&self.owners,
				suggestion,
				message_id,
				submitter,
			);
		}

		self.managed.queued.lock().unwrap().remove(&id);
	}
}

/// Spawn a collector for `suggestion`'s card, tracked in `armed` so it is armed at most once
/// and can be aborted later. The task frees its own `armed` slot when it ends, so a collector
/// that exits without resolving (a transient failure) is re-armed by the next sync.
fn arm(
	managed: &Arc<Managed>,
	ctx: &Context,
	data: &Arc<CommandData>,
	owners: &HashSet<UserId>,
	suggestion: Suggestion,
	message_id: MessageId,
	submitter: SuggestionSubmitter,
) {
	let id = suggestion.id;
	let mut armed = managed.armed.lock().unwrap();
	if armed.contains_key(&id) {
		return;
	}
	let managed_for_task = managed.clone();
	let ctx = ctx.clone();
	let data = data.clone();
	let owners = owners.clone();
	let handle = tokio::spawn(async move {
		interaction::arm(ctx, data, owners, suggestion, message_id, submitter).await;
		managed_for_task.armed.lock().unwrap().remove(&id);
	});
	armed.insert(id, handle);
}

/// Abort and forget the collector for `uuid`, if any. Called when its card is deleted so the
/// parked collector does not linger forever waiting on a message that is gone.
fn disarm(managed: &Managed, uuid: Uuid) {
	if let Some(handle) = managed.armed.lock().unwrap().remove(&uuid) {
		handle.abort();
	}
}

async fn send_worker(
	ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	managed: Arc<Managed>,
	mut rx: mpsc::UnboundedReceiver<Suggestion>,
	limiter: DirectLimiter,
) {
	while let Some(suggestion) = rx.recv().await {
		limiter.until_ready().await;
		data.discord_cooldown.wait().await;
		post_new_card(&ctx, &data, &owners, &managed, suggestion.clone()).await;
		managed.queued.lock().unwrap().remove(&suggestion.id);
	}
}

async fn edit_worker(
	ctx: Context,
	data: Arc<CommandData>,
	managed: Arc<Managed>,
	mut rx: mpsc::UnboundedReceiver<EditJob>,
	limiter: DirectLimiter,
) {
	while let Some(job) = rx.recv().await {
		limiter.until_ready().await;
		data.discord_cooldown.wait().await;
		let uuid = job.uuid();
		match job {
			EditJob::Refresh {
				suggestion,
				message_id,
			} => refresh_card(&ctx, &data, &managed, &suggestion, message_id).await,
			EditJob::Delete { uuid, message_id } => {
				delete_card(&ctx, &data, &managed, uuid, message_id).await
			}
		}
		managed.queued.lock().unwrap().remove(&uuid);
	}
}

async fn post_new_card(
	ctx: &Context,
	data: &Arc<CommandData>,
	owners: &HashSet<UserId>,
	managed: &Arc<Managed>,
	suggestion: Suggestion,
) {
	let id = suggestion.id;
	// Close the race with the immediate /suggest path: if a card already exists (posted by
	// the command, or by an earlier worker run), don't post a second one.
	match data.suggestion_store.get(id).await {
		Ok(Some(_)) => return,
		Ok(None) => {}
		Err(e) => {
			warn!("suggestion {id}: cannot check store before posting: {e}");
			return;
		}
	}

	let submitter = SuggestionSubmitter::from_source(suggestion.source.clone());
	let card_data =
		match build_card_data(&data.playmatch_client, &suggestion, submitter.clone()).await {
			Ok(d) => d,
			Err(e) => {
				warn!("suggestion {id}: cannot build card data to post: {e}");
				return;
			}
		};

	if let Some(message_id) = send_staff_card(ctx, data, &card_data).await {
		arm(
			managed, ctx, data, owners, suggestion, message_id, submitter,
		);
	}
}

/// Render `card_data` into a staff card, post it, and record it in the store. Returns the
/// new message id, or `None` if any step failed (already logged). Shared by the send worker
/// and the immediate `/suggest` path (`post_now`).
async fn send_staff_card(
	ctx: &Context,
	data: &Arc<CommandData>,
	card_data: &CardData,
) -> Option<MessageId> {
	let id = card_data.suggestion_id;
	let cx = match RenderContext::build(ctx.http.as_ref(), &data.playmatch_client, card_data).await
	{
		Ok(cx) => cx,
		Err(e) => {
			warn!("suggestion {id}: cannot build render context to post: {e}");
			return None;
		}
	};

	let message = match ChannelId::new(*SUGGESTION_CHANNEL_ID)
		.widen()
		.send_message(ctx.http.as_ref(), cx.staff_card(card_data).into_message())
		.await
	{
		Ok(m) => m,
		Err(e) => {
			warn!("suggestion {id}: posting card failed: {e}");
			return None;
		}
	};

	if let Err(e) = data
		.suggestion_store
		.mark_posted(id, message.id, LAYOUT_VERSION)
		.await
	{
		warn!("suggestion {id}: posted card but failed to record it: {e}");
	}
	Some(message.id)
}

async fn refresh_card(
	ctx: &Context,
	data: &Arc<CommandData>,
	managed: &Managed,
	suggestion: &Suggestion,
	message_id: MessageId,
) {
	let id = suggestion.id;
	let submitter = SuggestionSubmitter::from_source(suggestion.source.clone());
	let card_data = match build_card_data(&data.playmatch_client, suggestion, submitter).await {
		Ok(d) => d,
		Err(e) => {
			warn!("suggestion {id}: cannot build card data to refresh: {e}");
			return;
		}
	};
	let cx = match RenderContext::build(ctx.http.as_ref(), &data.playmatch_client, &card_data).await
	{
		Ok(cx) => cx,
		Err(e) => {
			warn!("suggestion {id}: cannot build render context to refresh: {e}");
			return;
		}
	};

	match ChannelId::new(*SUGGESTION_CHANNEL_ID)
		.widen()
		.edit_message(
			ctx.http.as_ref(),
			message_id,
			cx.staff_card(&card_data).into_edit(),
		)
		.await
	{
		Ok(_) => {
			if let Err(e) = data
				.suggestion_store
				.mark_posted(id, message_id, LAYOUT_VERSION)
				.await
			{
				warn!("suggestion {id}: refreshed card but failed to record version: {e}");
			}
		}
		// The card was deleted out from under us: drop the entry and its collector; the sync
		// reposts if the suggestion is still pending.
		Err(serenity::Error::Http(e)) if e.status_code() == Some(StatusCode::NOT_FOUND) => {
			disarm(managed, id);
			if let Err(e) = data.suggestion_store.remove(id).await {
				warn!("suggestion {id}: cannot drop entry after 404 on refresh: {e}");
			}
		}
		Err(e) => warn!("suggestion {id}: refreshing card failed: {e}"),
	}
}

async fn delete_card(
	ctx: &Context,
	data: &Arc<CommandData>,
	managed: &Managed,
	uuid: Uuid,
	message_id: MessageId,
) {
	// A 404 means a human already removed it, which is the outcome we wanted anyway.
	let result = match ChannelId::new(*SUGGESTION_CHANNEL_ID)
		.widen()
		.delete_message(ctx.http.as_ref(), message_id, None)
		.await
	{
		Err(serenity::Error::Http(e)) if e.status_code() == Some(StatusCode::NOT_FOUND) => Ok(()),
		other => other,
	};

	match result {
		Ok(()) => {
			// The card is gone; stop its collector waiting on a message that will never fire.
			disarm(managed, uuid);
			if let Err(e) = data.suggestion_store.remove(uuid).await {
				warn!("suggestion {uuid}: deleted card but cannot drop entry: {e}");
			}
		}
		// Keep the entry so the next sync retries the delete.
		Err(e) => warn!("suggestion {uuid}: deleting card failed: {e}"),
	}
}
