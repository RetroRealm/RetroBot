//! Rendering a playmatch suggestion into a Discord card.
//!
//! `CardData` is the flat set of facts a card needs, resolved from a playmatch
//! `Suggestion` by `build_card_data`. `RenderContext` adds the one-time provider reads
//! (enrichment, submitter identity) and turns it into the pending staff card or a resolved
//! card. Splitting the two keeps the provider reads in exactly one place.

use chrono::{DateTime, Utc};
use playmatch_client::types::{MetadataProvider, Suggestion};
use serenity::all::{ButtonStyle, CreateButton, Http, UserId};

use crate::abstraction::components_v2::{Card, Status, long_date, relative_timestamp};
use crate::abstraction::playmatch::{ConciseError, MatchSummary, format_match_summaries};
use crate::abstraction::playmatch_client::PlaymatchClient;
use crate::abstraction::providers::{self, display_name};

/// Version of the rendered card layout. Bump when `RenderContext::card` / `staff_card` or
/// the `Card` builder changes: tracked cards on an older version are re-rendered once
/// (rate-limited) by the sync, matching cards are left untouched.
pub const LAYOUT_VERSION: u32 = 1;

pub const APPROVE_PREFIX: &str = "approve:";
pub const DECLINE_PREFIX: &str = "decline:";

#[derive(Clone, Copy)]
pub enum SuggestionType {
	Game,
	Company,
	Platform,
}

impl SuggestionType {
	fn label(self) -> &'static str {
		match self {
			SuggestionType::Game => "Game",
			SuggestionType::Company => "Company",
			SuggestionType::Platform => "Platform",
		}
	}
}

#[derive(Clone)]
pub enum SuggestionSubmitter {
	DiscordUser(UserId),
	External { source: String },
}

impl SuggestionSubmitter {
	/// External submitters carry a source string (a truncated User-Agent); a `None` source
	/// on playmatch is a user-submitted suggestion whose Discord author we no longer know.
	pub fn from_source(source: Option<String>) -> Self {
		SuggestionSubmitter::External {
			source: source.unwrap_or_else(|| "unknown".to_string()),
		}
	}
}

/// The flat facts a suggestion card renders from, independent of where the suggestion came
/// from (the sync or a `/suggest` command).
pub struct CardData {
	pub suggestion_id: uuid::Uuid,
	pub kind: SuggestionType,
	pub provider: MetadataProvider,
	pub provider_id: String,
	pub name: String,
	pub platform: Option<String>,
	pub company: Option<String>,
	pub comment: Option<String>,
	pub created_at: DateTime<Utc>,
	pub existing_matches: Vec<MatchSummary>,
	pub submitter: SuggestionSubmitter,
}

/// Resolve a suggestion's target entity (game/company/platform) from playmatch and flatten
/// it into `CardData`. This is the only playmatch read `build_card_data` does; enrichment
/// happens later in `RenderContext::build`.
pub async fn build_card_data(
	client: &PlaymatchClient,
	suggestion: &Suggestion,
	submitter: SuggestionSubmitter,
) -> anyhow::Result<CardData> {
	let (kind, name, platform, company, existing_matches) =
		if let Some(game_id) = suggestion.game_id {
			let game = client
				.get_game_with_relations_by_id(game_id)
				.await
				.concise()?;
			(
				SuggestionType::Game,
				game.game.name.clone(),
				Some(game.platform.name.clone()),
				game.company.map(|c| c.name),
				game.external_metadata.iter().map(Into::into).collect(),
			)
		} else if let Some(company_id) = suggestion.company_id {
			let c = client.get_company_by_id(company_id).await.concise()?;
			(
				SuggestionType::Company,
				c.name.clone(),
				None,
				None,
				c.external_metadata.iter().map(Into::into).collect(),
			)
		} else if let Some(platform_id) = suggestion.platform_id {
			let p = client.get_platform_by_id(platform_id).await.concise()?;
			(
				SuggestionType::Platform,
				p.name.clone(),
				None,
				p.company_name.clone(),
				p.external_metadata.iter().map(Into::into).collect(),
			)
		} else {
			anyhow::bail!(
				"suggestion {} has no game/company/platform id",
				suggestion.id
			);
		};

	Ok(CardData {
		suggestion_id: suggestion.id,
		kind,
		provider: suggestion.provider,
		provider_id: suggestion.provider_id.clone(),
		name,
		platform,
		company,
		comment: suggestion.comment.clone(),
		created_at: suggestion.created_at,
		existing_matches,
		submitter,
	})
}

/// Provider enrichment shared by the staff card, the resolved card and the DM.
struct Enrichment {
	page_url: Option<String>,
	thumbnail_url: Option<String>,
	entry_name: String,
	/// Game summary / company description / platform summary.
	summary: Option<String>,
	/// Games only (IGDB today).
	released: Option<DateTime<Utc>>,
}

/// Set when a staff card is being rewritten into an Approved/Declined card.
pub struct Resolution {
	pub staff_id: UserId,
	/// `Some` for approved game suggestions.
	pub roms_updated: Option<u64>,
	pub resolved_at: DateTime<Utc>,
}

/// The reads a card needs beyond `CardData`: the submitter's display identity and the
/// provider enrichment. Built once, reused for the staff card and (later) the resolved card.
pub struct RenderContext {
	provider_label: &'static str,
	author_label: String,
	pub dm_target: Option<UserId>,
	enrichment: Option<Enrichment>,
	matches_line: String,
}

impl RenderContext {
	pub async fn build(
		http: &Http,
		client: &PlaymatchClient,
		data: &CardData,
	) -> anyhow::Result<Self> {
		let (author_label, dm_target) = match &data.submitter {
			SuggestionSubmitter::DiscordUser(id) => {
				let user = http.get_user(*id).await?;
				(format!("<@{}> ({})", user.id, user.name), Some(user.id))
			}
			SuggestionSubmitter::External { source } => (format!("External ({source})"), None),
		};

		let enrichment = match data.kind {
			SuggestionType::Game => providers::fetch_game(client, data.provider, &data.provider_id)
				.await
				.map(|i| Enrichment {
					page_url: i.page_url,
					thumbnail_url: i.cover_url,
					entry_name: i.name,
					summary: i.summary,
					released: i.first_release_date,
				}),
			SuggestionType::Company => {
				providers::fetch_company(client, data.provider, &data.provider_id)
					.await
					.map(|i| Enrichment {
						page_url: i.page_url,
						thumbnail_url: i.logo_url,
						entry_name: i.name,
						summary: i.description,
						released: None,
					})
			}
			SuggestionType::Platform => {
				providers::fetch_platform(client, data.provider, &data.provider_id)
					.await
					.map(|i| Enrichment {
						page_url: i.page_url,
						thumbnail_url: i.logo_url,
						entry_name: i.name,
						summary: i.summary,
						released: None,
					})
			}
		};

		Ok(Self {
			provider_label: display_name(data.provider),
			author_label,
			dm_target,
			enrichment,
			matches_line: format_match_summaries(&data.existing_matches),
		})
	}

	/// The shared card body (claim rows, provenance, provider entry, matches). `resolution`
	/// switches between the pending staff card and a resolved card.
	pub fn card(
		&self,
		data: &CardData,
		status: Status,
		heading: String,
		resolution: Option<&Resolution>,
	) -> Card<'static> {
		let provider_label = self.provider_label;
		let subheading = match resolution {
			Some(res) => format!(
				"Suggested {} · resolved {}",
				relative_timestamp(data.created_at),
				relative_timestamp(res.resolved_at),
			),
			None => format!("Suggested {}", relative_timestamp(data.created_at)),
		};

		let mut card = Card::new(status, heading).subheading(subheading);
		if let Some(thumb) = self
			.enrichment
			.as_ref()
			.and_then(|e| e.thumbnail_url.clone())
		{
			card = card.thumbnail(thumb);
		}

		card = card.row(data.kind.label(), data.name.clone());
		if let Some(platform) = data.platform.clone() {
			card = card.row("Platform", platform);
		}
		if let Some(company) = data.company.clone() {
			card = card.row("Company", company);
		}
		card = card.row(
			format!("{provider_label} ID"),
			format!("`{}`", data.provider_id),
		);

		card = card.row("Suggested by", self.author_label.clone());
		if let Some(comment) = data.comment.clone() {
			card = card.row("Comment", comment);
		}
		if let Some(res) = resolution {
			card = card.row("Handled by", format!("<@{}>", res.staff_id));
			if let Some(roms) = res.roms_updated {
				card = card.row("ROMs updated", roms.to_string());
			}
		}

		if let Some(e) = self.enrichment.as_ref() {
			card = card.section(format!("{provider_label} Entry"));
			card = card.row("Name", e.entry_name.clone());
			if let Some(released) = e.released {
				card = card.row("Released", long_date(released));
			}
			if let Some(summary) = e.summary.clone() {
				card = card.quote(summary);
			}
		}

		card = card
			.section("Existing Matches")
			.text(self.matches_line.clone());

		match resolution {
			Some(_) => card.footer(format!("Suggestion `{}`", data.suggestion_id)),
			None => card,
		}
	}

	/// The pending staff card: the shared body plus the provider link, Approve/Decline
	/// buttons and owners footer. Changing this layout means bumping `LAYOUT_VERSION`.
	pub fn staff_card(&self, data: &CardData) -> Card<'static> {
		let provider_label = self.provider_label;
		let mut card = self.card(
			data,
			Status::Info,
			format!("New {} Suggestion", data.kind.label()),
			None,
		);
		if let Some(url) = self.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
			card = card.link_external(url, format!("View on {provider_label}"));
		}
		card.link(
			CreateButton::new(format!("{APPROVE_PREFIX}{}", data.suggestion_id))
				.label("Approve")
				.style(ButtonStyle::Success),
		)
		.link(
			CreateButton::new(format!("{DECLINE_PREFIX}{}", data.suggestion_id))
				.label("Decline")
				.style(ButtonStyle::Danger),
		)
		.footer("Only bot owners can approve or decline.")
	}

	/// The submitter DM sent on resolution. `approved` picks the wording; games may report
	/// how many ROMs were updated.
	pub fn resolution_dm(
		&self,
		data: &CardData,
		approved: bool,
		roms_updated: Option<u64>,
	) -> Card<'static> {
		let (status, heading, closing) = if approved {
			(
				Status::Success,
				"Suggestion Approved",
				"Thanks for contributing.",
			)
		} else {
			(
				Status::Error,
				"Suggestion Declined",
				"If you'd like context, reach out to the Playmatch team.",
			)
		};

		let mut dm = Card::new(status, heading);
		if let Some(thumb) = self
			.enrichment
			.as_ref()
			.and_then(|e| e.thumbnail_url.clone())
		{
			dm = dm.thumbnail(thumb);
		}
		dm = dm.row(data.kind.label(), data.name.clone()).row(
			format!("{} ID", self.provider_label),
			format!("`{}`", data.provider_id),
		);
		if let Some(roms) = roms_updated {
			dm = dm.row("ROMs updated", roms.to_string());
		}
		dm = dm.text(closing);
		if let Some(url) = self.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
			dm = dm.link_external(url, format!("View on {}", self.provider_label));
		}
		dm
	}

	pub fn provider_label(&self) -> &'static str {
		self.provider_label
	}

	pub fn page_url(&self) -> Option<String> {
		self.enrichment.as_ref().and_then(|e| e.page_url.clone())
	}
}
