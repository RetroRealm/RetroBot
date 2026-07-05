//! Approve/Decline handling for a posted suggestion card.
//!
//! `arm` attaches an owners-only collector to a card's message and does nothing else until
//! a button is pressed. On a press it acknowledges immediately (to beat Discord's 3s
//! interaction deadline), then fetches card data and resolves on playmatch. Because the ack
//! happens before any read, resolution always edits the card in place rather than answering
//! the interaction a second time.

use std::collections::HashSet;
use std::sync::Arc;

use chrono::Utc;
use log::{error, warn};
use playmatch_client::types::Suggestion;
use serenity::all::{
	ChannelId, ComponentInteraction, ComponentInteractionCollector, Context,
	CreateInteractionResponse, MessageId, UserId,
};

use crate::abstraction::command::CommandData;
use crate::abstraction::components_v2::{Card, Status};
use crate::abstraction::playmatch::ConciseError;
use crate::command::SUGGESTION_CHANNEL_ID;
use crate::suggestions::card::{
	CardData, RenderContext, Resolution, SuggestionSubmitter, SuggestionType, build_card_data,
};

/// Attach an owners-only Approve/Decline collector to an existing card and wait. Issues no
/// playmatch reads and no Discord writes until a button is pressed; then acks, builds card
/// data, and resolves. `submitter` is carried from the post so the resolution can DM the
/// original `/suggest` author. Meant to be spawned: it lives as long as the card is
/// unresolved.
pub async fn arm(
	ctx: Context,
	data: Arc<CommandData>,
	owners: HashSet<UserId>,
	suggestion: Suggestion,
	message_id: MessageId,
	submitter: SuggestionSubmitter,
) {
	let suggestion_id = suggestion.id;
	let filter_owners = owners.clone();
	let Some(interaction) = ComponentInteractionCollector::new(&ctx)
		.message_id(message_id)
		.filter(move |i| filter_owners.contains(&i.user.id))
		.await
	else {
		return;
	};

	// Acknowledge before any read: build_card_data + enrichment can take several requests,
	// well past Discord's 3s response window.
	if let Err(e) = interaction
		.create_response(ctx.http.as_ref(), CreateInteractionResponse::Acknowledge)
		.await
	{
		warn!("suggestion {suggestion_id}: failed to ack interaction: {e}");
		return;
	}

	let card_data = match build_card_data(&data.playmatch_client, &suggestion, submitter).await {
		Ok(d) => d,
		Err(e) => {
			warn!("suggestion {suggestion_id}: cannot build card data on resolve: {e}");
			return;
		}
	};
	let cx = match RenderContext::build(ctx.http.as_ref(), &data.playmatch_client, &card_data).await
	{
		Ok(cx) => cx,
		Err(e) => {
			warn!("suggestion {suggestion_id}: cannot build render context on resolve: {e}");
			return;
		}
	};

	resolve(&ctx, &data, &card_data, &cx, &interaction, message_id).await;
}

/// Run the approve/decline the pressed button asked for. The interaction is already acked.
async fn resolve(
	ctx: &Context,
	data: &CommandData,
	card: &CardData,
	cx: &RenderContext,
	interaction: &ComponentInteraction,
	message_id: MessageId,
) {
	let staff_id = interaction.user.id;
	let approved = match interaction.data.custom_id.split(':').next() {
		Some("approve") => true,
		Some("decline") => false,
		_ => {
			warn!("unexpected suggestion button custom_id: {}", interaction.data.custom_id);
			return;
		}
	};

	let roms_updated = if approved {
		match data.playmatch_client.approve_suggestion(card.suggestion_id).await.concise() {
			Ok(updated) => {
				matches!(card.kind, SuggestionType::Game).then(|| updated.updated.max(0) as u64)
			}
			Err(e) => {
				warn!("suggestion {}: approve failed: {e}", card.suggestion_id);
				return;
			}
		}
	} else {
		if let Err(e) = data.playmatch_client.delete_suggestion(card.suggestion_id).await.concise() {
			warn!("suggestion {}: decline failed: {e}", card.suggestion_id);
			return;
		}
		None
	};

	let (status, heading) = if approved {
		(Status::Success, "Suggestion Approved")
	} else {
		(Status::Error, "Suggestion Declined")
	};
	let mut resolution = cx.card(
		card,
		status,
		heading.to_string(),
		Some(&Resolution {
			staff_id,
			roms_updated,
			resolved_at: Utc::now(),
		}),
	);
	if let Some(url) = cx.page_url() {
		resolution = resolution.link_external(url, format!("View on {}", cx.provider_label()));
	}

	finalize(ctx, data, card.suggestion_id, message_id, resolution).await;

	if let Some(dm_id) = cx.dm_target {
		let dm = cx.resolution_dm(card, approved, roms_updated);
		if let Err(e) = dm_id.dm(ctx.http.as_ref(), dm.into_message()).await {
			error!("suggestion {}: failed to DM submitter: {e}", card.suggestion_id);
		}
	}
}

/// Edit the staff card into its resolved form, then drop the store entry. The entry is
/// removed only after the edit succeeds: on failure the suggestion is already resolved on
/// playmatch, and keeping the entry lets the next sync delete the now-stale card instead of
/// orphaning one with dead buttons.
async fn finalize(
	ctx: &Context,
	data: &CommandData,
	suggestion_id: uuid::Uuid,
	message_id: MessageId,
	resolution: Card<'static>,
) {
	let channel_id = ChannelId::new(*SUGGESTION_CHANNEL_ID);
	match channel_id
		.widen()
		.edit_message(ctx.http.as_ref(), message_id, resolution.into_edit())
		.await
	{
		Ok(_) => {
			if let Err(e) = data.suggestion_store.remove(suggestion_id).await {
				warn!("suggestion {suggestion_id}: cannot remove from store after resolve: {e}");
			}
		}
		Err(e) => warn!(
			"suggestion {suggestion_id}: cannot update resolved card (keeping store entry): {e}"
		),
	}
}
