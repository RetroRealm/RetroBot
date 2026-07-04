use crate::abstraction::command::{
	CommandContext, CommandResult, TRUSTED_ROLE_IDS, is_user_trusted_or_above,
};
use crate::abstraction::components_v2::{self, Card, Status, long_date, relative_timestamp};
use crate::abstraction::playmatch::{
	ApiErrorAction, ConciseError, describe, format_match_summaries, paginate_playmatch_response,
	send_playmatch_api_error,
};
use crate::abstraction::providers::{self, ENRICHMENT_PRIORITY, ProviderChoice, display_name};
use crate::command::SUGGESTION_CHANNEL_ID;
use anyhow::anyhow;
use chrono::Utc;
use log::{debug, error, warn};
use playmatch_client::types::ManualMatchMode::{Admin, Trusted};
use playmatch_client::types::{
	CompanyOrPlatformMatchRequest, CompanyOrPlatformSuggestionRequest, CreateOrGetUserRequestV2,
	GameMatchRequest, GameMatchType, GameSuggestionRequest, ManualMatchMode, MetadataMatchType,
	MetadataProvider, UpdateUserPermissionsRequestV2, UserPermissions,
};
use reqwest::StatusCode;
use serenity::all::{
	ButtonStyle, Cache, ChannelId, ComponentInteraction, ComponentInteractionCollector, Context,
	CreateButton, CreateInteractionResponse, MessageId, UserId,
};
use serenity::http::Http;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

/// Shows a list of playmatch entities with its metadata matches
#[poise::command(
	slash_command,
	category = "Playmatch",
	required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
	subcommands("list_companies", "list_platforms")
)]
pub async fn list(_: CommandContext<'_>) -> CommandResult {
	Ok(())
}

/// Subcommands related to manual metadata matching in playmatch
#[poise::command(
	slash_command,
	category = "Playmatch",
	required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
	rename = "match",
	subcommands("manual_match_game", "manual_match_platform", "manual_match_company")
)]
pub async fn r#match(_: CommandContext<'_>) -> CommandResult {
	Ok(())
}

/// Subcommands related to suggesting a metadata match in playmatch
#[poise::command(
	slash_command,
	category = "Playmatch",
	required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
	subcommands(
		"create_platform_suggestion",
		"create_company_suggestion",
		"create_game_suggestion",
		"cleanup_suggestions"
	)
)]
pub async fn suggest(_: CommandContext<'_>) -> CommandResult {
	Ok(())
}

/// Shows a list of companies with its metadata matches
#[poise::command(slash_command, category = "Playmatch", rename = "companies")]
pub async fn list_companies(ctx: CommandContext<'_>) -> CommandResult {
	let companies = ctx
		.data()
		.playmatch_client
		.get_all_companies()
		.await
		.concise()?;
	paginate_playmatch_response(ctx, companies).await
}

/// Shows a list of platforms with its metadata matches
#[poise::command(slash_command, category = "Playmatch", rename = "platforms")]
pub async fn list_platforms(ctx: CommandContext<'_>) -> CommandResult {
	let platforms = ctx
		.data()
		.playmatch_client
		.get_all_platforms()
		.await
		.concise()?;
	paginate_playmatch_response(ctx, platforms).await
}

/// Gets metadata for a game on Playmatch by hashes or file name and size.
#[poise::command(slash_command, category = "Playmatch", rename = "game")]
pub async fn get_game_metadata(
	ctx: CommandContext<'_>,
	md5_hash: Option<String>,
	sha1_hash: Option<String>,
	sha256_hash: Option<String>,
	file_name: String,
	file_size: i64,
) -> CommandResult {
	let inner = ctx
		.data()
		.playmatch_client
		.identify_game_and_relations(file_name, file_size, md5_hash, sha1_hash, sha256_hash)
		.await
		.concise()?;

	if inner.game_match_type == GameMatchType::NoMatch {
		ctx.send(components_v2::status_reply(
			Status::Error,
			"No matching game found for the provided hashes or file name and size.",
		))
		.await?;
		return Ok(());
	}

	let game = inner.game.ok_or(anyhow!(
		"No game found for the provided hashes or file name"
	))?;
	let game_files = inner.game_files;
	let platform = inner.platform.ok_or(anyhow!(
		"No platform found for the provided hashes or file name"
	))?;
	let metadata_mappings = inner.external_metadata;
	let company = inner.company;
	let dat_file = inner.dat_file.ok_or(anyhow!(
		"No DAT file found for the provided hashes or file name"
	))?;
	let signature_group = inner.signature_group.ok_or(anyhow!(
		"No signature group found for the provided hashes or file name"
	))?;

	let mut enriched_info = None;
	for &priority_provider in ENRICHMENT_PRIORITY {
		let Some(provider_id) = metadata_mappings.iter().find_map(|m| {
			if m.provider_name == priority_provider
				&& matches!(
					m.match_type,
					MetadataMatchType::Automatic | MetadataMatchType::Manual
				) {
				m.provider_id.clone()
			} else {
				None
			}
		}) else {
			continue;
		};
		if let Some(info) = providers::fetch_game(
			&ctx.data().playmatch_client,
			priority_provider,
			&provider_id,
		)
		.await
		{
			enriched_info = Some(info);
			break;
		}
	}

	let total_files = game_files.len();
	let mut files_info = game_files
		.iter()
		.enumerate()
		.take(5)
		.map(|(i, file)| {
			let mut out = format!("**{}. {}**", i + 1, file.file_name);
			if let Some(size) = file.file_size_in_bytes {
				out.push_str(&format!(
					"\nSize: `{:.2} MB`",
					size as f64 / 1024.0 / 1024.0
				));
			}
			if let Some(serial) = &file.serial {
				out.push_str(&format!("\nSerial: `{serial}`"));
			}
			if let Some(crc) = &file.crc {
				out.push_str(&format!("\nCRC32: `{crc}`"));
			}
			if let Some(md5) = &file.md5 {
				out.push_str(&format!("\nMD5: `{md5}`"));
			}
			if let Some(sha1) = &file.sha1 {
				out.push_str(&format!("\nSHA1: `{sha1}`"));
			}
			if let Some(sha256) = &file.sha256 {
				out.push_str(&format!("\nSHA256: `{sha256}`"));
			}
			out
		})
		.collect::<Vec<_>>()
		.join("\n\n");

	if total_files > 5 {
		let hidden = total_files - 5;
		files_info.push_str(&format!("\n\n-# …and {hidden} more ROM files not shown"));
	}

	let escape_dat_file_name = dat_file.name.replace("_", "\\_");
	let dat_file_value = match dat_file.tags {
		None => format!(
			"**{}**\nSignature Group: {}\nCurrent Version: `{}`",
			escape_dat_file_name, signature_group.name, dat_file.current_version
		),
		Some(tags) => format!(
			"**{}**\nSignature Group: {}\nCurrent Version: `{}`\nTags: `{}`",
			escape_dat_file_name,
			signature_group.name,
			dat_file.current_version,
			tags.join(", ")
		),
	};

	let mut card = Card::new(Status::Success, game.name.clone());

	if let Some(info) = enriched_info.as_ref() {
		if let Some(cover_url) = info.cover_url.clone() {
			card = card.thumbnail(cover_url);
		}
		if let Some(date) = info.first_release_date {
			card = card.subheading(format!("Released {}", date.format("%b %-d, %Y")));
		}
		if let Some(summary) = info.summary.clone() {
			card = card.intro(summary);
		}
	}

	card = card
		.row("Match Type", format!("`{}`", inner.game_match_type))
		.row("Platform", platform.name);

	if let Some(c) = company {
		card = card.row("Company", c.name);
	}

	card = card.section("ROM Files").text(files_info);
	card = card.section("DAT File").text(dat_file_value);

	if !metadata_mappings.is_empty() {
		card = card.section("Metadata Mappings");
		for metadata_mapping in metadata_mappings {
			let provider_info = match metadata_mapping.match_type {
				MetadataMatchType::Automatic => {
					let reason = metadata_mapping
						.automatic_match_reason
						.ok_or_else(|| anyhow!("Automatic match is missing reason"))?;
					let provider_id = metadata_mapping
						.provider_id
						.ok_or_else(|| anyhow!("Automatic match is missing provider id"))?;
					format!("\nReason: `{reason}`\nProvider ID: `{provider_id}`")
				}
				MetadataMatchType::Manual => {
					let manual_match_type = metadata_mapping
						.manual_match_type
						.ok_or_else(|| anyhow!("Manual match is missing match type"))?;
					let provider_id = metadata_mapping
						.provider_id
						.ok_or_else(|| anyhow!("Manual match is missing provider id"))?;
					format!("\nMatched By: `{manual_match_type}`\nProvider ID: `{provider_id}`")
				}
				MetadataMatchType::Failed => {
					let failed_reason = metadata_mapping
						.failed_match_reason
						.ok_or_else(|| anyhow!("Failed match is missing reason"))?;
					format!("\nReason: `{failed_reason}`")
				}
				MetadataMatchType::None => String::new(),
			};
			card = card.text(format!(
				"**{}** Status: `{}`{}",
				metadata_mapping.provider_name, metadata_mapping.match_type, provider_info
			));
		}
	}

	if let Some(info) = enriched_info.as_ref()
		&& !info.screenshot_urls.is_empty()
	{
		card = card.media(info.screenshot_urls.clone());
	}

	if let Some(info) = enriched_info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}
	if let Some(link) = signature_group.website_link.clone() {
		card = card.link_external(link, "Signature Group");
	}

	card = card.footer(format!("Playmatch Game ID: `{}`", game.id));

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Creates a metadata match suggestion for a Game by hashes or name.
#[poise::command(slash_command, category = "Playmatch", rename = "game")]
#[allow(clippy::too_many_arguments)]
pub async fn create_game_suggestion(
	ctx: CommandContext<'_>,
	provider: ProviderChoice,
	provider_id: String,
	md5_hash: Option<String>,
	sha1_hash: Option<String>,
	sha256_hash: Option<String>,
	name: Option<String>,
	comment: Option<String>,
) -> CommandResult {
	if md5_hash.is_none() && sha1_hash.is_none() && sha256_hash.is_none() && name.is_none() {
		ctx.send(components_v2::status_reply(
			Status::Error,
			"You must provide at least one of the following: MD5, SHA1, SHA256 or name.",
		))
		.await?;
		return Ok(());
	}

	let provider_meta = provider.to_metadata_provider();
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_game_suggestion(GameSuggestionRequest {
			provider_id: provider_id.clone(),
			sha1: sha1_hash,
			provider: provider_meta,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
			md5: md5_hash,
			sha256: sha256_hash,
		})
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value,
		Err(e) => {
			send_playmatch_api_error(ctx, &e, "Game", "hashes or name", ApiErrorAction::Suggest)
				.await?;
			return Ok(());
		}
	};

	let game_id = match suggestion.game_id {
		Some(id) => id,
		None => {
			error!("Game ID is missing in the suggestion response! This should not happen.");
			ctx.send(components_v2::status_reply(
				Status::Error,
				"Internal error: game ID missing from suggestion response.",
			))
			.await?;
			return Ok(());
		}
	};

	let game_response = ctx
		.data()
		.playmatch_client
		.get_game_with_relations_by_id(game_id)
		.await
		.concise()?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let suggestion_store = ctx.data().suggestion_store.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let suggestion_provider_id = provider_id.clone();
		let game_name = game_response.game.name.clone();
		let platform = game_response.platform.name.clone();
		let company = game_response.company.clone().map(|c| c.name);
		let created_at = suggestion.created_at;
		let existing_matches = game_response
			.external_metadata
			.iter()
			.map(Into::into)
			.collect();
		async move {
			handle_suggestion_message(
				SuggestionMessageHandleData {
					playmatch_client,
					suggestion_store,
					serenity_ctx,
					suggestion_id: suggestion.id,
					owners,
					submitter: SuggestionSubmitter::DiscordUser(author_id),
					r#type: SuggestionType::Game,
					provider: provider_meta,
					provider_id: suggestion_provider_id,
					name: game_name,
					platform: Some(platform),
					company,
					comment: suggestion.comment,
					created_at,
					existing_matches,
				},
				CardPresentation::New,
			)
			.await
		}
	});

	let info =
		providers::fetch_game(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Suggestion Submitted");
	if let Some(info) = info.as_ref() {
		if let Some(cover) = info.cover_url.clone() {
			card = card.thumbnail(cover);
		}
		card = card.subheading(info.name.clone());
	}
	card = card
		.row("Game", game_response.game.name.clone())
		.row("Platform", game_response.platform.name.clone())
		.row(
			format!("{} ID", display_name(provider_meta)),
			format!("`{provider_id}`"),
		)
		.text("We'll DM you when a maintainer approves or declines this.");
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Creates a metadata match suggestion for a Company by name.
#[poise::command(slash_command, category = "Playmatch", rename = "company")]
pub async fn create_company_suggestion(
	ctx: CommandContext<'_>,
	provider: ProviderChoice,
	provider_id: String,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let provider_meta = provider.to_metadata_provider();
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_company_suggestion(CompanyOrPlatformSuggestionRequest {
			provider_id: provider_id.clone(),
			provider: provider_meta,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value,
		Err(e) => {
			send_playmatch_api_error(ctx, &e, "Company", "name", ApiErrorAction::Suggest).await?;
			return Ok(());
		}
	};

	// Existing matches mirror the poller's company card. The card must still post if
	// this lookup fails, so a failure logs and falls back to an empty list.
	let existing_matches: Vec<crate::abstraction::playmatch::MatchSummary> =
		match suggestion.company_id {
			Some(company_id) => match ctx
				.data()
				.playmatch_client
				.get_company_by_id(company_id)
				.await
			{
				Ok(company) => company.external_metadata.iter().map(Into::into).collect(),
				Err(e) => {
					warn!(
						"failed to fetch company {company_id} for existing matches: {}",
						describe(&e)
					);
					Vec::new()
				}
			},
			None => Vec::new(),
		};

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let suggestion_store = ctx.data().suggestion_store.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let suggestion_provider_id = provider_id.clone();
		let company_name = name.clone();
		let created_at = suggestion.created_at;
		async move {
			handle_suggestion_message(
				SuggestionMessageHandleData {
					playmatch_client,
					suggestion_store,
					serenity_ctx,
					suggestion_id: suggestion.id,
					owners,
					submitter: SuggestionSubmitter::DiscordUser(author_id),
					r#type: SuggestionType::Company,
					provider: provider_meta,
					provider_id: suggestion_provider_id,
					name: company_name,
					platform: None,
					company: None,
					comment: suggestion.comment,
					created_at,
					existing_matches,
				},
				CardPresentation::New,
			)
			.await
		}
	});

	let info =
		providers::fetch_company(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Suggestion Submitted");
	if let Some(info) = info.as_ref() {
		if let Some(logo) = info.logo_url.clone() {
			card = card.thumbnail(logo);
		}
		card = card.subheading(info.name.clone());
	}
	card = card
		.row("Company", name.clone())
		.row(
			format!("{} ID", display_name(provider_meta)),
			format!("`{provider_id}`"),
		)
		.text("We'll DM you when a maintainer approves or declines this.");
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Creates a metadata match suggestion for a Platform by name.
#[poise::command(slash_command, category = "Playmatch", rename = "platform")]
pub async fn create_platform_suggestion(
	ctx: CommandContext<'_>,
	provider: ProviderChoice,
	provider_id: String,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let provider_meta = provider.to_metadata_provider();
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_platform_suggestion(CompanyOrPlatformSuggestionRequest {
			provider_id: provider_id.clone(),
			provider: provider_meta,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value,
		Err(e) => {
			send_playmatch_api_error(ctx, &e, "Platform", "name", ApiErrorAction::Suggest).await?;
			return Ok(());
		}
	};

	let platform_id = match suggestion.platform_id {
		Some(id) => id,
		None => {
			error!("Platform ID is missing in the suggestion response! This should not happen.");
			ctx.send(components_v2::status_reply(
				Status::Error,
				"Internal error: platform ID missing from suggestion response.",
			))
			.await?;
			return Ok(());
		}
	};

	let platform = ctx
		.data()
		.playmatch_client
		.get_platform_by_id(platform_id)
		.await
		.concise()?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let suggestion_store = ctx.data().suggestion_store.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let suggestion_provider_id = provider_id.clone();
		let platform_name = name.clone();
		let company = platform.company_name.clone();
		let created_at = suggestion.created_at;
		let existing_matches = platform.external_metadata.iter().map(Into::into).collect();
		async move {
			handle_suggestion_message(
				SuggestionMessageHandleData {
					playmatch_client,
					suggestion_store,
					serenity_ctx,
					suggestion_id: suggestion.id,
					owners,
					submitter: SuggestionSubmitter::DiscordUser(author_id),
					r#type: SuggestionType::Platform,
					provider: provider_meta,
					provider_id: suggestion_provider_id,
					name: platform_name,
					platform: None,
					company,
					comment: suggestion.comment,
					created_at,
					existing_matches,
				},
				CardPresentation::New,
			)
			.await
		}
	});

	let info =
		providers::fetch_platform(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Suggestion Submitted");
	if let Some(info) = info.as_ref() {
		if let Some(logo) = info.logo_url.clone() {
			card = card.thumbnail(logo);
		}
		card = card.subheading(info.name.clone());
	}
	card = card
		.row("Platform", name.clone())
		.row(
			format!("{} ID", display_name(provider_meta)),
			format!("`{provider_id}`"),
		)
		.text("We'll DM you when a maintainer approves or declines this.");
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Dismisses pending game suggestions whose game is already mapped to that provider.
// Handles the race where an external tool like RomM matches a game after the
// suggestion card has already been posted to Discord.
#[poise::command(
	slash_command,
	category = "Playmatch",
	rename = "cleanup",
	check = "is_user_trusted_or_above"
)]
pub async fn cleanup_suggestions(ctx: CommandContext<'_>) -> CommandResult {
	ctx.defer().await?;

	let data = ctx.data();
	let http = ctx.serenity_context().http.clone();
	let pending = data
		.playmatch_client
		.get_all_suggestions()
		.await
		.concise()?;
	let report = crate::events::suggestion_poller::sweep_redundant_suggestions(
		http.as_ref(),
		&data,
		&pending,
	)
	.await;

	let mut card = Card::new(Status::Success, "Suggestion Cleanup".to_string())
		.row("Checked", format!("`{}`", report.checked))
		.row("Cleaned", format!("`{}`", report.cleaned.len()));

	if !report.cleaned.is_empty() {
		const PREVIEW_MAX: usize = 10;
		let preview = report
			.cleaned
			.iter()
			.take(PREVIEW_MAX)
			.map(|u| format!("`{u}`"))
			.collect::<Vec<_>>()
			.join("\n");
		card = card.section("Cleaned Suggestions").text(preview);
		if report.cleaned.len() > PREVIEW_MAX {
			let extra = report.cleaned.len() - PREVIEW_MAX;
			card = card.text(format!("-# …and {extra} more not shown"));
		}
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Manually matches a Platform by name.
#[poise::command(slash_command, category = "Playmatch", rename = "platform", check = is_user_trusted_or_above)]
pub async fn manual_match_platform(
	ctx: CommandContext<'_>,
	provider: ProviderChoice,
	provider_id: String,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let provider_meta = provider.to_metadata_provider();
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_platform(CompanyOrPlatformMatchRequest {
			manual_match_type: playmatch_user_ctx.manual_match_mode(),
			provider_id: provider_id.clone(),
			provider: provider_meta,
			name,
			matched_name: None,
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	if let Err(e) = &result {
		send_playmatch_api_error(ctx, e, "Platform", "name", ApiErrorAction::Match).await?;
		return Ok(());
	}

	let info =
		providers::fetch_platform(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Matched Platform");
	if let Some(info) = info.as_ref() {
		if let Some(logo) = info.logo_url.clone() {
			card = card.thumbnail(logo);
		}
		card = card.subheading(info.name.clone());
	}
	card = card.row(
		format!("{} ID", display_name(provider_meta)),
		format!("`{provider_id}`"),
	);
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Manually matches a Company by name.
#[poise::command(slash_command, category = "Playmatch", rename = "company", check = is_user_trusted_or_above)]
pub async fn manual_match_company(
	ctx: CommandContext<'_>,
	provider: ProviderChoice,
	provider_id: String,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let provider_meta = provider.to_metadata_provider();
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_company(CompanyOrPlatformMatchRequest {
			manual_match_type: playmatch_user_ctx.manual_match_mode(),
			provider_id: provider_id.clone(),
			provider: provider_meta,
			name,
			matched_name: None,
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	if let Err(e) = &result {
		send_playmatch_api_error(ctx, e, "Company", "name", ApiErrorAction::Match).await?;
		return Ok(());
	}

	let info =
		providers::fetch_company(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Matched Company");
	if let Some(info) = info.as_ref() {
		if let Some(logo) = info.logo_url.clone() {
			card = card.thumbnail(logo);
		}
		card = card.subheading(info.name.clone());
	}
	card = card.row(
		format!("{} ID", display_name(provider_meta)),
		format!("`{provider_id}`"),
	);
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Manually matches a game with provided hashes or name.
#[poise::command(slash_command, category = "Playmatch", rename = "game", check = is_user_trusted_or_above)]
#[allow(clippy::too_many_arguments)]
pub async fn manual_match_game(
	ctx: CommandContext<'_>,
	provider: ProviderChoice,
	provider_id: String,
	md5_hash: Option<String>,
	sha1_hash: Option<String>,
	sha256_hash: Option<String>,
	name: Option<String>,
	comment: Option<String>,
) -> CommandResult {
	if md5_hash.is_none() && sha1_hash.is_none() && sha256_hash.is_none() && name.is_none() {
		ctx.send(components_v2::status_reply(
			Status::Error,
			"You must provide at least one of the following: MD5, SHA1, SHA256 or name.",
		))
		.await?;
		return Ok(());
	}

	let provider_meta = provider.to_metadata_provider();
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_game(GameMatchRequest {
			manual_match_type: playmatch_user_ctx.manual_match_mode(),
			provider_id: provider_id.clone(),
			provider: provider_meta,
			md5: md5_hash,
			sha1: sha1_hash,
			sha256: sha256_hash,
			name,
			matched_name: None,
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	let matched = match result {
		Err(e) => {
			send_playmatch_api_error(ctx, &e, "Game", "hashes or name", ApiErrorAction::Match)
				.await?;
			return Ok(());
		}
		Ok(value) => value.len(),
	};

	let info =
		providers::fetch_game(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Matched Game");
	if let Some(info) = info.as_ref() {
		if let Some(cover) = info.cover_url.clone() {
			card = card.thumbnail(cover);
		}
		card = card.subheading(info.name.clone());
	}
	card = card
		.row(
			format!("{} ID", display_name(provider_meta)),
			format!("`{provider_id}`"),
		)
		.row("ROMs updated", matched.to_string());
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link_external(page_url, format!("View on {}", display_name(info.provider)));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

struct PlaymatchUserCtx {
	is_admin: bool,
	playmatch_user: playmatch_client::types::User,
}

impl PlaymatchUserCtx {
	fn manual_match_mode(&self) -> ManualMatchMode {
		if self.is_admin { Admin } else { Trusted }
	}
}

pub(crate) enum SuggestionType {
	Platform,
	Company,
	Game,
}

pub(crate) enum SuggestionSubmitter {
	DiscordUser(UserId),
	External { source: String },
}

/// Version of the rendered suggestion card layout. Bump this when the rendered
/// suggestion card changes: `build_card`/`build_staff_card` below, or the `Card` builder
/// in `components_v2`. Tracked cards whose stored version differs are re-rendered once
/// (paced) on the next boot; matching cards are left untouched. Suggestion cards are the
/// only persistent cards the bot owns, so the version is scoped to them, not to `Card`.
pub(crate) const SUGGESTION_CARD_LAYOUT_VERSION: u32 = 1;

/// How `handle_suggestion_message` should present the staff card before it arms the
/// collector. A steady-state re-arm renders nothing and never lands here: it goes through
/// the poller's zero-read `rearm_existing_collector`, which fetches card data only if a
/// button is actually pressed.
pub(crate) enum CardPresentation {
	/// Post a brand-new staff card into the suggestion channel.
	New,
	/// Rewrite an existing card that is on a stale layout, keeping its message id.
	Refresh(serenity::all::MessageId),
}

pub(crate) struct SuggestionMessageHandleData {
	pub playmatch_client: Arc<crate::abstraction::playmatch_client::PlaymatchClient>,
	pub suggestion_store: Arc<crate::abstraction::suggestion_store::SuggestionStore>,
	pub serenity_ctx: Context,
	pub suggestion_id: Uuid,
	pub owners: HashSet<UserId>,
	pub submitter: SuggestionSubmitter,
	pub r#type: SuggestionType,
	pub provider: MetadataProvider,
	pub provider_id: String,
	pub name: String,
	pub platform: Option<String>,
	pub company: Option<String>,
	pub comment: Option<String>,
	pub created_at: chrono::DateTime<chrono::Utc>,
	pub existing_matches: Vec<crate::abstraction::playmatch::MatchSummary>,
}

/// Provider enrichment shared by the staff card, resolution cards and DMs.
/// Built from whichever of `ProviderGameInfo` / `ProviderCompanyInfo` /
/// `ProviderPlatformInfo` the suggestion type fetched. One fetch, as before.
struct Enrichment {
	page_url: Option<String>,
	thumbnail_url: Option<String>,
	entry_name: String,
	/// Game summary / company description / platform summary.
	summary: Option<String>,
	/// Games only (IGDB today).
	released: Option<chrono::DateTime<chrono::Utc>>,
}

/// Set when the staff card is being rewritten into an Approved/Declined card.
struct Resolution {
	staff_id: UserId,
	/// `Some` for approved game suggestions.
	roms_updated: Option<u64>,
	resolved_at: chrono::DateTime<chrono::Utc>,
}

/// Card data fetched once and shared between the staff card and the resolution card: the
/// submitter label, the provider enrichment, and the pre-rendered match list. `build` is
/// the only place that issues the provider reads, so a caller that never renders a card
/// never pays for them.
pub(crate) struct RenderContext {
	display_type: &'static str,
	provider_label: &'static str,
	author_label: String,
	dm_target: Option<UserId>,
	enrichment: Option<Enrichment>,
	matches_line: String,
}

impl RenderContext {
	/// Resolve the submitter and fetch provider enrichment. This is where the card's
	/// playmatch reads happen, so it runs only when a card is about to be rendered (a
	/// fresh post, a layout refresh, or a resolution), never on a steady-state re-arm.
	pub(crate) async fn build(data: &SuggestionMessageHandleData) -> anyhow::Result<Self> {
		let http: &Http = data.serenity_ctx.http.as_ref();

		let (author_label, dm_target): (String, Option<UserId>) = match &data.submitter {
			SuggestionSubmitter::DiscordUser(id) => {
				let user = http.get_user(*id).await?;
				(format!("<@{}> ({})", user.id, user.name), Some(user.id))
			}
			SuggestionSubmitter::External { source } => (format!("External ({source})"), None),
		};

		let display_type = match data.r#type {
			SuggestionType::Platform => "Platform",
			SuggestionType::Company => "Company",
			SuggestionType::Game => "Game",
		};

		let enrichment: Option<Enrichment> = match data.r#type {
			SuggestionType::Game => {
				providers::fetch_game(&data.playmatch_client, data.provider, &data.provider_id)
					.await
					.map(|i| Enrichment {
						page_url: i.page_url,
						thumbnail_url: i.cover_url,
						entry_name: i.name,
						summary: i.summary,
						released: i.first_release_date,
					})
			}
			SuggestionType::Company => {
				providers::fetch_company(&data.playmatch_client, data.provider, &data.provider_id)
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
				providers::fetch_platform(&data.playmatch_client, data.provider, &data.provider_id)
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
			display_type,
			provider_label: display_name(data.provider),
			author_label,
			dm_target,
			enrichment,
			matches_line: format_match_summaries(&data.existing_matches),
		})
	}

	/// The shared card body (claim rows, provenance, provider entry, matches).
	/// `resolution` switches it between the pending staff card and a resolution card.
	fn card(
		&self,
		data: &SuggestionMessageHandleData,
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
		if let Some(thumb) = self.enrichment.as_ref().and_then(|e| e.thumbnail_url.clone()) {
			card = card.thumbnail(thumb);
		}

		// Claim rows.
		card = card.row(self.display_type.to_string(), data.name.clone());
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

		// Provenance rows.
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

		// Provider entry section.
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

		// Existing matches section.
		card = card.section("Existing Matches").text(self.matches_line.clone());

		match resolution {
			Some(_) => card.footer(format!("Suggestion `{}`", data.suggestion_id)),
			None => card,
		}
	}

	/// The full pending staff card: the shared body plus the provider link, Approve/Decline
	/// buttons and owners footer. Any change to this layout must bump
	/// SUGGESTION_CARD_LAYOUT_VERSION.
	fn staff_card(&self, data: &SuggestionMessageHandleData) -> Card<'static> {
		let provider_label = self.provider_label;
		let mut staff_card = self.card(
			data,
			Status::Info,
			format!("New {} Suggestion", self.display_type),
			None,
		);
		if let Some(url) = self.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
			staff_card = staff_card.link_external(url, format!("View on {provider_label}"));
		}
		staff_card
			.link(
				CreateButton::new(format!("approve:{}", data.suggestion_id))
					.label("Approve")
					.style(ButtonStyle::Success),
			)
			.link(
				CreateButton::new(format!("decline:{}", data.suggestion_id))
					.label("Decline")
					.style(ButtonStyle::Danger),
			)
			.footer("Only bot owners can approve or decline.")
	}
}

/// Posts or refreshes the staff card, then waits on the Approve/Decline buttons. This path
/// always renders a card, so it builds the `RenderContext` (and its provider reads) up
/// front. A steady-state re-arm renders nothing and goes through the poller's
/// `rearm_existing_collector`, which builds the context only after a button is pressed.
pub(crate) async fn handle_suggestion_message(
	data: SuggestionMessageHandleData,
	presentation: CardPresentation,
) -> CommandResult {
	let http: &Http = data.serenity_ctx.http.as_ref();
	let cache: Arc<Cache> = data.serenity_ctx.cache.clone();

	let channel_id = ChannelId::new(*SUGGESTION_CHANNEL_ID);
	let channel_exists = cache.guilds().iter().any(|guild_id| {
		cache
			.guild(*guild_id)
			.is_some_and(|g| g.channels.contains_key(&channel_id))
	});

	if !channel_exists {
		return Err(anyhow!("Suggestion channel not found"));
	}

	let cx = RenderContext::build(&data).await?;

	let message_id = match presentation {
		// The stored layout is stale (or unknown), so rewrite the card before arming the
		// collector, then record the current version. Version is written only after the
		// edit succeeds, so a crash in between skews toward one redundant (paced) refresh
		// next boot, never toward a silently-stale card.
		CardPresentation::Refresh(id) => {
			match channel_id
				.widen()
				.edit_message(http, id, cx.staff_card(&data).into_edit())
				.await
			{
				Ok(_) => {
					if let Err(e) = data
						.suggestion_store
						.mark_posted(data.suggestion_id, id, Some(SUGGESTION_CARD_LAYOUT_VERSION))
						.await
					{
						warn!(
							"failed to record layout version for suggestion {} after re-arm edit: {e}",
							data.suggestion_id
						);
					}
					id
				}
				// A 404 means the card was deleted out from under us. Treat it like a dead
				// entry, exactly as delete_suggestion_card does: drop the store entry and
				// skip arming a collector (a card that does not exist cannot be approved).
				// The next reconcile reposts the suggestion fresh from its source if it is
				// still pending, so the end state stays coherent.
				Err(serenity::Error::Http(e)) if e.status_code() == Some(StatusCode::NOT_FOUND) => {
					if let Err(e) = data.suggestion_store.remove(data.suggestion_id).await {
						warn!(
							"failed to remove suggestion {} from store after 404 on re-arm edit: {e}",
							data.suggestion_id
						);
					}
					return Ok(());
				}
				// Transient Discord blips must not kill the re-arm; arm the collector on
				// the existing (still stale) card without a version write so the next boot
				// retries the refresh.
				Err(e) => {
					warn!(
						"failed to refresh suggestion card {} on re-arm, arming collector anyway: {e}",
						data.suggestion_id
					);
					id
				}
			}
		}
		CardPresentation::New => {
			let message = channel_id
				.widen()
				.send_message(http, cx.staff_card(&data).into_message())
				.await?;
			data.suggestion_store
				.mark_posted(
					data.suggestion_id,
					message.id,
					Some(SUGGESTION_CARD_LAYOUT_VERSION),
				)
				.await?;
			message.id
		}
	};

	let owners = data.owners.clone();
	let Some(interaction) = ComponentInteractionCollector::new(&data.serenity_ctx)
		.message_id(message_id)
		.filter(move |i| owners.contains(&i.user.id))
		.await
	else {
		return Ok(());
	};

	// The context is already in hand, so answer the interaction in one shot.
	resolve_from_interaction(&data, &cx, interaction, message_id, false).await
}

/// Handle an Approve/Decline click: resolve on playmatch, swap the staff card for a
/// resolution card, then drop the store entry (plus a submitter DM). The interaction is
/// acknowledged first, before the playmatch call, so a slow write (e.g. blocked behind a
/// 429 cooldown) cannot outrun Discord's 3 s response window and fail the interaction.
/// `acked` is true when the caller already sent that acknowledgement (the lazy re-arm
/// path acks before fetching card data); this path then skips the redundant ack.
pub(crate) async fn resolve_from_interaction(
	data: &SuggestionMessageHandleData,
	cx: &RenderContext,
	interaction: ComponentInteraction,
	message_id: MessageId,
	acked: bool,
) -> CommandResult {
	let http: &Http = data.serenity_ctx.http.as_ref();
	let channel_id = ChannelId::new(*SUGGESTION_CHANNEL_ID);
	let staff_id = interaction.user.id;
	let action = interaction.data.custom_id.split(':').next().unwrap_or("");

	// Acknowledge up front (unless the caller already did) so the card edit and any paced
	// playmatch write happen after the interaction is answered, never against its 3 s clock.
	if !acked {
		interaction
			.create_response(http, CreateInteractionResponse::Acknowledge)
			.await?;
	}

	match action {
		"approve" => {
			let updated = data
				.playmatch_client
				.approve_suggestion(data.suggestion_id)
				.await
				.concise()?;

			let roms_updated =
				matches!(data.r#type, SuggestionType::Game).then(|| updated.updated.max(0) as u64);
			let mut resolution = cx.card(
				data,
				Status::Success,
				"Suggestion Approved".to_string(),
				Some(&Resolution {
					staff_id,
					roms_updated,
					resolved_at: Utc::now(),
				}),
			);
			if let Some(url) = cx.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
				resolution = resolution.link_external(url, format!("View on {}", cx.provider_label));
			}

			finalize_resolution(data, channel_id, message_id, resolution).await;

			if let Some(dm_id) = cx.dm_target {
				let mut dm = Card::new(Status::Success, "Suggestion Approved");
				if let Some(thumb) = cx.enrichment.as_ref().and_then(|e| e.thumbnail_url.clone()) {
					dm = dm.thumbnail(thumb);
				}
				dm = dm.row(cx.display_type.to_string(), data.name.clone()).row(
					format!("{} ID", cx.provider_label),
					format!("`{}`", data.provider_id),
				);
				if let Some(roms) = roms_updated {
					dm = dm.row("ROMs updated", roms.to_string());
				}
				dm = dm.text("Thanks for contributing.");
				if let Some(url) = cx.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
					dm = dm.link_external(url, format!("View on {}", cx.provider_label));
				}
				if let Err(e) = dm_id.dm(http, dm.into_message()).await {
					error!("failed to DM submitter on approval: {e}");
				}
			}
		}
		"decline" => {
			data.playmatch_client
				.delete_suggestion(data.suggestion_id)
				.await
				.concise()?;

			let mut resolution = cx.card(
				data,
				Status::Error,
				"Suggestion Declined".to_string(),
				Some(&Resolution {
					staff_id,
					roms_updated: None,
					resolved_at: Utc::now(),
				}),
			);
			if let Some(url) = cx.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
				resolution = resolution.link_external(url, format!("View on {}", cx.provider_label));
			}

			finalize_resolution(data, channel_id, message_id, resolution).await;

			if let Some(dm_id) = cx.dm_target {
				let mut dm = Card::new(Status::Error, "Suggestion Declined");
				if let Some(thumb) = cx.enrichment.as_ref().and_then(|e| e.thumbnail_url.clone()) {
					dm = dm.thumbnail(thumb);
				}
				dm = dm
					.row(cx.display_type.to_string(), data.name.clone())
					.row(
						format!("{} ID", cx.provider_label),
						format!("`{}`", data.provider_id),
					)
					.text("If you'd like context, reach out to the Playmatch team.");
				if let Some(url) = cx.enrichment.as_ref().and_then(|e| e.page_url.clone()) {
					dm = dm.link_external(url, format!("View on {}", cx.provider_label));
				}
				if let Err(e) = dm_id.dm(http, dm.into_message()).await {
					error!("failed to DM submitter on decline: {e}");
				}
			}
		}
		_ => {
			warn!(
				"Unexpected button interaction custom_id: {}",
				interaction.data.custom_id
			);
		}
	}

	Ok(())
}

/// Edit the staff card into its resolution card, then drop the store entry. The store
/// entry is removed only after the edit succeeds: if the edit fails the suggestion is
/// already resolved on playmatch but keeping the entry lets the next reconcile pass delete
/// the now-stale card, instead of orphaning a card with dead Approve/Decline buttons.
async fn finalize_resolution(
	data: &SuggestionMessageHandleData,
	channel_id: ChannelId,
	message_id: MessageId,
	resolution: Card<'static>,
) {
	let http: &Http = data.serenity_ctx.http.as_ref();
	match channel_id
		.widen()
		.edit_message(http, message_id, resolution.into_edit())
		.await
	{
		Ok(_) => {
			if let Err(e) = data.suggestion_store.remove(data.suggestion_id).await {
				warn!(
					"failed to remove suggestion {} from store after resolve: {e}",
					data.suggestion_id
				);
			}
		}
		Err(e) => {
			warn!(
				"failed to update resolved card for suggestion {} (keeping store entry for reconcile): {e}",
				data.suggestion_id
			);
		}
	}
}

async fn get_playmatch_user_ctx(ctx: CommandContext<'_>) -> anyhow::Result<PlaymatchUserCtx> {
	let author = ctx.author();
	let member_opt = ctx.author_member().await;

	let is_admin = ctx.framework().options().owners.contains(&author.id);
	let is_trusted = match member_opt {
		None => false,
		Some(member) => member
			.roles
			.iter()
			.any(|role| TRUSTED_ROLE_IDS.contains(&role.get())),
	};

	let permissions = if is_admin {
		UserPermissions::Admin
	} else if is_trusted {
		UserPermissions::Trusted
	} else {
		UserPermissions::User
	};

	let mut playmatch_user = ctx
		.data()
		.playmatch_client
		.create_or_get_by_discord_id(CreateOrGetUserRequestV2 {
			discord_id: author.id.get() as i64,
			permissions,
			username: author.name.to_string(),
		})
		.await
		.concise()?;

	if playmatch_user.permissions == UserPermissions::User && (is_trusted || is_admin) {
		debug!(
			"User permission needs to be updated for user: {}",
			author.id
		);

		let new_permission = if is_admin {
			UserPermissions::Admin
		} else {
			UserPermissions::Trusted
		};

		ctx.data()
			.playmatch_client
			.update_user_permission_level(
				playmatch_user.id,
				UpdateUserPermissionsRequestV2 { new_permission },
			)
			.await
			.concise()?;

		debug!("User permission updated for user: {}", author.id);

		playmatch_user.permissions = new_permission;
	}

	Ok(PlaymatchUserCtx {
		is_admin,
		playmatch_user,
	})
}
