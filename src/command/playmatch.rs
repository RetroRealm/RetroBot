use crate::abstraction::command::{
	CommandContext, CommandResult, TRUSTED_ROLE_IDS, is_user_trusted_or_above,
};
use crate::abstraction::components_v2::{self, Card, Status};
use crate::abstraction::playmatch::{
	ApiErrorAction, paginate_playmatch_response, send_playmatch_api_error,
};
use crate::abstraction::providers::{self, ENRICHMENT_PRIORITY, ProviderChoice, display_name};
use crate::command::SUGGESTION_CHANNEL_ID;
use anyhow::anyhow;
use log::{debug, error, warn};
use playmatch_client::types::ManualMatchMode::{Admin, Trusted};
use playmatch_client::types::{
	CompanyOrPlatformMatchRequest, CompanyOrPlatformSuggestionRequest, CreateOrGetUserRequest,
	GameMatchRequest, GameMatchType, GameSuggestionRequest, ManualMatchMode, MetadataMatchType,
	MetadataProvider, UpdateUserPermissionsRequest, UserPermissions,
};
use serenity::all::{
	ButtonStyle, Cache, ChannelId, ComponentInteractionCollector, Context, CreateButton,
	CreateComponent, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage,
	MessageFlags, UserId,
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
		"create_game_suggestion"
	)
)]
pub async fn suggest(_: CommandContext<'_>) -> CommandResult {
	Ok(())
}

/// Shows a list of companies with its metadata matches
#[poise::command(slash_command, category = "Playmatch", rename = "companies")]
pub async fn list_companies(ctx: CommandContext<'_>) -> CommandResult {
	let companies = ctx.data().playmatch_client.get_all_companies().await?;
	paginate_playmatch_response(ctx, companies).await
}

/// Shows a list of platforms with its metadata matches
#[poise::command(slash_command, category = "Playmatch", rename = "platforms")]
pub async fn list_platforms(ctx: CommandContext<'_>) -> CommandResult {
	let platforms = ctx.data().playmatch_client.get_all_platforms().await?;
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
		.await?;

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
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
	}
	if let Some(link) = signature_group.website_link.clone() {
		card = card.link(CreateButton::new_link(link).label("Signature Group"));
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
		.get_playmatch_game_with_relations_by_id(game_id)
		.await?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let suggestion_store = ctx.data().suggestion_store.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let game_name = game_response.game.name.clone();
		let platform = game_response.platform.name.clone();
		let company = game_response.company.clone().map(|c| c.name);
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
					name: game_name,
					platform: Some(platform),
					company,
					comment: suggestion.comment,
				},
				None,
			)
			.await
		}
	});

	let info =
		providers::fetch_game(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Suggestion Submitted");
	if let Some(info) = info.as_ref()
		&& let Some(cover) = info.cover_url.clone()
	{
		card = card.thumbnail(cover);
	}
	card = card
		.row("Game", game_response.game.name.clone())
		.row(
			format!("{} ID", display_name(provider_meta)),
			format!("`{provider_id}`"),
		)
		.text("We'll DM you when a maintainer approves or declines this.");
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
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

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let suggestion_store = ctx.data().suggestion_store.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let company_name = name.clone();
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
					name: company_name,
					platform: None,
					company: None,
					comment: suggestion.comment,
				},
				None,
			)
			.await
		}
	});

	let info =
		providers::fetch_company(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Suggestion Submitted");
	if let Some(info) = info.as_ref()
		&& let Some(logo) = info.logo_url.clone()
	{
		card = card.thumbnail(logo);
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
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
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
		.await?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let suggestion_store = ctx.data().suggestion_store.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let platform_name = name.clone();
		let company = platform.company_name.clone();
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
					name: platform_name,
					platform: None,
					company,
					comment: suggestion.comment,
				},
				None,
			)
			.await
		}
	});

	let info =
		providers::fetch_platform(&ctx.data().playmatch_client, provider_meta, &provider_id).await;

	let mut card = Card::new(Status::Success, "Suggestion Submitted");
	if let Some(info) = info.as_ref()
		&& let Some(logo) = info.logo_url.clone()
	{
		card = card.thumbnail(logo);
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
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
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
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
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
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
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
		card = card.link(
			CreateButton::new_link(page_url)
				.label(format!("View on {}", display_name(info.provider))),
		);
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

pub(crate) struct SuggestionMessageHandleData {
	pub playmatch_client: Arc<crate::abstraction::playmatch_client::PlaymatchClient>,
	pub suggestion_store: Arc<crate::abstraction::suggestion_store::SuggestionStore>,
	pub serenity_ctx: Context,
	pub suggestion_id: Uuid,
	pub owners: HashSet<UserId>,
	pub submitter: SuggestionSubmitter,
	pub r#type: SuggestionType,
	pub provider: MetadataProvider,
	pub name: String,
	pub platform: Option<String>,
	pub company: Option<String>,
	pub comment: Option<String>,
}

/// Posts the staff card and waits on the Approve/Decline buttons.
/// Pass `Some(message_id)` to re-attach to an existing card instead of posting a new one.
pub(crate) async fn handle_suggestion_message(
	data: SuggestionMessageHandleData,
	existing_message_id: Option<serenity::all::MessageId>,
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

	let suggestion = data
		.playmatch_client
		.get_suggestion_by_id(data.suggestion_id)
		.await?;

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

	let provider_page_url: Option<String> = match data.r#type {
		SuggestionType::Game => providers::fetch_game(
			&data.playmatch_client,
			data.provider,
			&suggestion.provider_id,
		)
		.await
		.and_then(|i| i.page_url),
		SuggestionType::Company => providers::fetch_company(
			&data.playmatch_client,
			data.provider,
			&suggestion.provider_id,
		)
		.await
		.and_then(|i| i.page_url),
		SuggestionType::Platform => providers::fetch_platform(
			&data.playmatch_client,
			data.provider,
			&suggestion.provider_id,
		)
		.await
		.and_then(|i| i.page_url),
	};

	let provider_label = display_name(data.provider);

	let build_card = |status: Status, heading: String| -> Card<'static> {
		let mut card = Card::new(status, heading).row("Suggested by", author_label.clone());
		card = card.row(display_type.to_string(), data.name.clone());
		if let Some(platform) = data.platform.clone() {
			card = card.row("Platform", platform);
		}
		if let Some(company) = data.company.clone() {
			card = card.row("Company", company);
		}
		card = card.row(
			format!("{provider_label} ID"),
			format!("`{}`", suggestion.provider_id.clone()),
		);
		card = card.row(
			"Comment",
			data.comment.clone().unwrap_or_else(|| "—".to_string()),
		);
		card
	};

	let message_id = match existing_message_id {
		Some(id) => id,
		None => {
			let mut staff_card = build_card(
				Status::Info,
				format!("New {display_type} Metadata Suggestion"),
			);
			if let Some(url) = provider_page_url.clone() {
				staff_card = staff_card
					.link(CreateButton::new_link(url).label(format!("View on {provider_label}")));
			}
			staff_card = staff_card
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
				.footer("Only bot owners can approve or decline.");

			let message = channel_id
				.widen()
				.send_message(http, staff_card.into_message())
				.await?;
			data.suggestion_store
				.mark_posted(data.suggestion_id, message.id)
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

	let staff_id = interaction.user.id;

	let action = interaction.data.custom_id.split(':').next().unwrap_or("");

	match action {
		"approve" => {
			let updated = data
				.playmatch_client
				.approve_suggestion(data.suggestion_id)
				.await?;

			if let Err(e) = data.suggestion_store.remove(data.suggestion_id).await {
				warn!(
					"failed to remove suggestion {} from store after approve: {e}",
					data.suggestion_id
				);
			}

			let mut resolution = build_card(Status::Success, "Suggestion Approved".to_string())
				.row("Handled by", format!("<@{staff_id}>"));
			if matches!(data.r#type, SuggestionType::Game) {
				resolution = resolution.row("ROMs updated", updated.updated.to_string());
			}
			if let Some(url) = provider_page_url.clone() {
				resolution = resolution
					.link(CreateButton::new_link(url).label(format!("View on {provider_label}")));
			}

			interaction
				.create_response(
					http,
					CreateInteractionResponse::UpdateMessage(
						CreateInteractionResponseMessage::new()
							.flags(MessageFlags::IS_COMPONENTS_V2)
							.components(vec![CreateComponent::Container(
								resolution.into_container(),
							)]),
					),
				)
				.await?;

			if let Some(dm_id) = dm_target {
				let mut dm = Card::new(Status::Success, "Suggestion Approved")
					.row(display_type.to_string(), data.name.clone());
				if matches!(data.r#type, SuggestionType::Game) {
					dm = dm.row("ROMs updated", updated.updated.to_string());
				}
				dm = dm.text("Thanks for contributing.");
				if let Err(e) = dm_id.dm(http, dm.into_message()).await {
					error!("failed to DM submitter on approval: {e}");
				}
			}
		}
		"decline" => {
			data.playmatch_client
				.delete_suggestion(data.suggestion_id)
				.await?;

			if let Err(e) = data.suggestion_store.remove(data.suggestion_id).await {
				warn!(
					"failed to remove suggestion {} from store after decline: {e}",
					data.suggestion_id
				);
			}

			let mut resolution = build_card(Status::Error, "Suggestion Declined".to_string())
				.row("Handled by", format!("<@{staff_id}>"));
			if let Some(url) = provider_page_url.clone() {
				resolution = resolution
					.link(CreateButton::new_link(url).label(format!("View on {provider_label}")));
			}

			interaction
				.create_response(
					http,
					CreateInteractionResponse::UpdateMessage(
						CreateInteractionResponseMessage::new()
							.flags(MessageFlags::IS_COMPONENTS_V2)
							.components(vec![CreateComponent::Container(
								resolution.into_container(),
							)]),
					),
				)
				.await?;

			if let Some(dm_id) = dm_target {
				let dm = Card::new(Status::Error, "Suggestion Declined")
					.row(display_type.to_string(), data.name.clone())
					.text("If you'd like context, reach out to the Playmatch team.");
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

/// Edits an existing suggestion card into a "Resolved externally" state with no buttons.
/// Used by the poller when it notices a suggestion was actioned via the playmatch API
/// while the bot was down or before the bot saw it.
pub(crate) async fn mark_external_resolved(
	http: &Http,
	message_id: serenity::all::MessageId,
) -> Result<(), serenity::Error> {
	let channel_id = ChannelId::new(*SUGGESTION_CHANNEL_ID);
	let card = Card::new(
		Status::Warning,
		"Suggestion Resolved Externally".to_string(),
	)
	.text("This suggestion was approved or declined outside Discord.");
	let edit = EditMessage::new()
		.flags(MessageFlags::IS_COMPONENTS_V2)
		.components(vec![CreateComponent::Container(card.into_container())]);
	channel_id
		.widen()
		.edit_message(http, message_id, edit)
		.await
		.map(|_| ())
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
		.create_or_get_by_discord_id(CreateOrGetUserRequest {
			discord_id: author.id.get() as i64,
			permissions,
			username: author.name.to_string(),
		})
		.await?;

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
				UpdateUserPermissionsRequest { new_permission },
			)
			.await?;

		debug!("User permission updated for user: {}", author.id);

		playmatch_user.permissions = new_permission;
	}

	Ok(PlaymatchUserCtx {
		is_admin,
		playmatch_user,
	})
}
