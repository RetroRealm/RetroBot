use crate::abstraction::command::{
	CommandContext, CommandResult, TRUSTED_ROLE_IDS, is_user_trusted_or_above,
};
use crate::abstraction::components_v2::{self, Card, Status};
use crate::abstraction::igdb;
use crate::abstraction::playmatch::paginate_playmatch_response;
use crate::command::SUGGESTION_CHANNEL_ID;
use anyhow::anyhow;
use log::{debug, error, warn};
use playmatch_client::Error;
use playmatch_client::types::ManualMatchMode::{Admin, Trusted};
use playmatch_client::types::MetadataProvider::Igdb;
use playmatch_client::types::{
	CompanyOrPlatformMatchRequest, CompanyOrPlatformSuggestionRequest, CreateOrGetUserRequest,
	GameMatchRequest, GameMatchType, GameSuggestionRequest, MetadataMatchType,
	UpdateUserPermissionsRequest, UserPermissions,
};
use reqwest::StatusCode;
use serenity::all::{
	ButtonStyle, Cache, ChannelId, ComponentInteractionCollector, ComponentInteractionDataKind,
	Context, CreateActionRow, CreateButton, CreateContainer, CreateContainerComponent,
	CreateSeparator, CreateTextDisplay, UserId,
};
use serenity::http::Http;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
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
	let response = ctx
		.data()
		.playmatch_client
		.get_all_companies()
		.send()
		.await?;
	let companies = response.into_inner();
	paginate_playmatch_response(ctx, companies).await
}

/// Shows a list of platforms with its metadata matches
#[poise::command(slash_command, category = "Playmatch", rename = "platforms")]
pub async fn list_platforms(ctx: CommandContext<'_>) -> CommandResult {
	let response = ctx
		.data()
		.playmatch_client
		.get_all_platforms()
		.send()
		.await?;
	let platforms = response.into_inner();
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
	let response = ctx
		.data()
		.playmatch_client
		.identify_game_and_relations()
		.file_name(file_name)
		.file_size(file_size)
		.md5(md5_hash.unwrap_or_default())
		.sha1(sha1_hash.unwrap_or_default())
		.sha256(sha256_hash.unwrap_or_default())
		.send()
		.await?;

	let inner = response.into_inner();

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

	let igdb_provider_id = metadata_mappings
		.iter()
		.find(|m| {
			m.provider_name == Igdb
				&& matches!(
					m.match_type,
					MetadataMatchType::Automatic | MetadataMatchType::Manual
				)
		})
		.and_then(|m| m.provider_id.clone());

	let igdb_info = match igdb_provider_id.as_deref() {
		Some(pid) => igdb::fetch_game(&ctx.data().playmatch_client, pid).await,
		None => None,
	};

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

	if let Some(info) = igdb_info.as_ref() {
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
						.expect("Automatic match reason is missing");
					let provider_id = metadata_mapping
						.provider_id
						.expect("Provider ID is missing for automatic match");
					format!("\nReason: `{reason}`\nProvider ID: `{provider_id}`")
				}
				MetadataMatchType::Manual => {
					let manual_match_type = metadata_mapping
						.manual_match_type
						.expect("Manual match type is missing");
					let provider_id = metadata_mapping
						.provider_id
						.expect("Provider ID is missing for manual match");
					format!("\nMatched By: `{manual_match_type}`\nProvider ID: `{provider_id}`")
				}
				MetadataMatchType::Failed => {
					let failed_reason = metadata_mapping
						.failed_match_reason
						.expect("Failed match reason is missing");
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

	if let Some(info) = igdb_info.as_ref()
		&& !info.screenshot_urls.is_empty()
	{
		card = card.media(info.screenshot_urls.clone());
	}

	if let Some(info) = igdb_info.as_ref() {
		card = card.link(CreateButton::new_link(info.page_url.clone()).label("View on IGDB"));
	}
	if let Some(link) = signature_group.website_link.clone() {
		card = card.link(CreateButton::new_link(link).label("Signature Group"));
	}

	card = card.footer(format!("Playmatch Game ID: `{}`", game.id));

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Creates a metadata match suggestion for a Game by hashes or name to the IGDB database.
#[poise::command(slash_command, category = "Playmatch", rename = "game")]
pub async fn create_game_suggestion(
	ctx: CommandContext<'_>,
	igdb_id: i64,
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

	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_game_suggestion()
		.body(GameSuggestionRequest {
			provider_id: igdb_id.to_string(),
			sha1: sha1_hash,
			provider: Igdb,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
			md5: md5_hash,
			sha256: sha256_hash,
		})
		.send()
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value.into_inner(),
		Err(e) => {
			match &e {
				Error::ErrorResponse(e_res) => {
					if e_res.status() == StatusCode::NOT_FOUND {
						ctx.send(components_v2::status_reply(
							Status::Error,
							"No Game found with the provided hashes or names.",
						))
						.await?;
					} else if e_res.status() == StatusCode::CONFLICT {
						ctx.send(components_v2::status_reply(
							Status::Error,
							"A suggestion for this Game already exists with the same provider and provider id.",
						))
						.await?;
					}
				}
				_ => {
					ctx.send(components_v2::status_reply(
						Status::Error,
						format!("Failed to create a suggestion for Game: {e}"),
					))
					.await?;
					warn!("Failed to create a suggestion for Game: {e}");
				}
			}
			return Ok(());
		}
	};

	let game_id = match suggestion.game_id {
		Some(id) => id,
		None => {
			error!("Game ID is missing in the suggestion response! This should not happen.");
			ctx.send(components_v2::status_reply(
				Status::Error,
				"Failed to create a suggestion for Game: Game ID is missing in the suggestion response.",
			))
			.await?;
			return Ok(());
		}
	};

	let game_response = ctx
		.data()
		.playmatch_client
		.get_playmatch_game_with_relations_by_id()
		.id(game_id)
		.send()
		.await?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let game_name = game_response.game.name.clone();
		let platform = game_response.platform.name.clone();
		let company = game_response.company.clone().map(|c| c.name);
		async move {
			handle_suggestion_message(SuggestionMessageHandleData {
				playmatch_client,
				serenity_ctx,
				suggestion_id: suggestion.id,
				owners,
				author_id,
				r#type: SuggestionType::Game,
				name: game_name,
				platform: Some(platform),
				company,
				comment: suggestion.comment,
			})
			.await
		}
	});

	ctx.send(components_v2::status_reply(
		Status::Success,
		format!(
			"Successfully created suggestion for Game {}. Thank you for your contribution 🎉! We will notify you once it has been approved or rejected.",
			&game_response.game.name
		),
	))
	.await?;

	Ok(())
}

/// Creates a metadata match suggestion for a Company by name to the IGDB database.
#[poise::command(slash_command, category = "Playmatch", rename = "company")]
pub async fn create_company_suggestion(
	ctx: CommandContext<'_>,
	igdb_id: i64,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_company_suggestion()
		.body(CompanyOrPlatformSuggestionRequest {
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.send()
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value.into_inner(),
		Err(e) => {
			match &e {
				Error::ErrorResponse(e_res) => {
					if e_res.status() == StatusCode::NOT_FOUND {
						ctx.send(components_v2::status_reply(
							Status::Error,
							"No Company found with the provided name.",
						))
						.await?;
					} else if e_res.status() == StatusCode::CONFLICT {
						ctx.send(components_v2::status_reply(
							Status::Error,
							"A suggestion for this Company already exists with the same provider and provider id.",
						))
						.await?;
					}
				}
				_ => {
					ctx.send(components_v2::status_reply(
						Status::Error,
						format!("Failed to create a suggestion for Company: {e}"),
					))
					.await?;
					warn!("Failed to create a suggestion for Company: {e}");
				}
			}
			return Ok(());
		}
	};

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let company_name = name.clone();
		async move {
			handle_suggestion_message(SuggestionMessageHandleData {
				playmatch_client,
				serenity_ctx,
				suggestion_id: suggestion.id,
				owners,
				author_id,
				r#type: SuggestionType::Company,
				name: company_name,
				platform: None,
				company: None,
				comment: suggestion.comment,
			})
			.await
		}
	});

	ctx.send(components_v2::status_reply(
		Status::Success,
		format!(
			"Successfully created suggestion for Company {}. Thank you for your contribution 🎉! We will notify you once it has been approved or rejected.",
			&name
		),
	))
	.await?;

	Ok(())
}

/// Creates a metadata match suggestion for a Platform by name to the IGDB database.
#[poise::command(slash_command, category = "Playmatch", rename = "platform")]
pub async fn create_platform_suggestion(
	ctx: CommandContext<'_>,
	igdb_id: i64,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_platform_suggestion()
		.body(CompanyOrPlatformSuggestionRequest {
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.send()
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value.into_inner(),
		Err(e) => {
			match &e {
				Error::ErrorResponse(e_res) => {
					if e_res.status() == StatusCode::NOT_FOUND {
						ctx.send(components_v2::status_reply(
							Status::Error,
							"No Platform found with the provided name.",
						))
						.await?;
					} else if e_res.status() == StatusCode::CONFLICT {
						ctx.send(components_v2::status_reply(
							Status::Error,
							"A suggestion for this Platform already exists with the same provider and provider id.",
						))
						.await?;
					}
				}
				_ => {
					ctx.send(components_v2::status_reply(
						Status::Error,
						format!("Failed to create a suggestion for Platform: {e}"),
					))
					.await?;
					warn!("Failed to create a suggestion for Platform: {e}");
				}
			}
			return Ok(());
		}
	};

	let platform_id = suggestion.platform_id.ok_or_else(|| {
		anyhow!("Platform ID is missing in the suggestion response! This should not happen.")
	})?;

	let platform = ctx
		.data()
		.playmatch_client
		.get_platform_by_id()
		.id(platform_id)
		.send()
		.await?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let serenity_ctx = ctx.serenity_context().clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let platform_name = name.clone();
		let company = platform.company_name.clone();
		async move {
			handle_suggestion_message(SuggestionMessageHandleData {
				playmatch_client,
				serenity_ctx,
				suggestion_id: suggestion.id,
				owners,
				author_id,
				r#type: SuggestionType::Platform,
				name: platform_name,
				platform: None,
				company,
				comment: suggestion.comment,
			})
			.await
		}
	});

	ctx.send(components_v2::status_reply(
		Status::Success,
		format!(
			"Successfully created suggestion for Platform {}. Thank you for your contribution 🎉! We will notify you once it has been approved or rejected.",
			&name
		),
	))
	.await?;

	Ok(())
}

/// Manually matches a Platform by name to the IGDB database.
#[poise::command(slash_command, category = "Playmatch", rename = "platform", check = is_user_trusted_or_above)]
pub async fn manual_match_platform(
	ctx: CommandContext<'_>,
	igdb_id: i64,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_platform()
		.body(CompanyOrPlatformMatchRequest {
			manual_match_type: if playmatch_user_ctx.is_admin {
				Admin
			} else {
				Trusted
			},
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			name,
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.send()
		.await;

	if let Err(e) = &result {
		match e {
			Error::ErrorResponse(e_res) if e_res.status() == StatusCode::NOT_FOUND => {
				ctx.send(components_v2::status_reply(
					Status::Error,
					"No Platform found for the provided name.",
				))
				.await?;
				return Ok(());
			}
			_ => {
				ctx.send(components_v2::status_reply(
					Status::Error,
					format!("Failed to match Platform: {e}"),
				))
				.await?;
				warn!("Failed to match Platform: {e}");
				return Ok(());
			}
		}
	}

	let igdb_id_str = igdb_id.to_string();
	let info = igdb::fetch_platform(&ctx.data().playmatch_client, &igdb_id_str).await;

	let mut card = Card::new(Status::Success, "Matched Platform");
	if let Some(info) = info.as_ref() {
		if let Some(logo) = info.logo_url.clone() {
			card = card.thumbnail(logo);
		}
		card = card.subheading(info.name.clone());
	}
	card = card.row("IGDB ID", format!("`{igdb_id_str}`"));
	if let Some(info) = info.as_ref() {
		card = card.link(CreateButton::new_link(info.page_url.clone()).label("View on IGDB"));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Manually matches a Company by name to the IGDB database.
#[poise::command(slash_command, category = "Playmatch", rename = "company", check = is_user_trusted_or_above)]
pub async fn manual_match_company(
	ctx: CommandContext<'_>,
	igdb_id: i64,
	name: String,
	comment: Option<String>,
) -> CommandResult {
	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_company()
		.body(CompanyOrPlatformMatchRequest {
			manual_match_type: if playmatch_user_ctx.is_admin {
				Admin
			} else {
				Trusted
			},
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			name,
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.send()
		.await;

	if let Err(e) = &result {
		match e {
			Error::ErrorResponse(e_res) if e_res.status() == StatusCode::NOT_FOUND => {
				ctx.send(components_v2::status_reply(
					Status::Error,
					"No Company found for the provided name.",
				))
				.await?;
				return Ok(());
			}
			_ => {
				ctx.send(components_v2::status_reply(
					Status::Error,
					format!("Failed to match Company: {e}"),
				))
				.await?;
				warn!("Failed to match Company: {e}");
				return Ok(());
			}
		}
	}

	let igdb_id_str = igdb_id.to_string();
	let info = igdb::fetch_company(&ctx.data().playmatch_client, &igdb_id_str).await;

	let mut card = Card::new(Status::Success, "Matched Company");
	if let Some(info) = info.as_ref() {
		if let Some(logo) = info.logo_url.clone() {
			card = card.thumbnail(logo);
		}
		card = card.subheading(info.name.clone());
	}
	card = card.row("IGDB ID", format!("`{igdb_id_str}`"));
	if let Some(info) = info.as_ref()
		&& let Some(page_url) = info.page_url.clone()
	{
		card = card.link(CreateButton::new_link(page_url).label("View on IGDB"));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

/// Manually matches a game with provided hashes or name to the IGDB database.
#[poise::command(slash_command, category = "Playmatch", rename = "game", check = is_user_trusted_or_above)]
pub async fn manual_match_game(
	ctx: CommandContext<'_>,
	igdb_id: i64,
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

	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_game()
		.body(GameMatchRequest {
			manual_match_type: if playmatch_user_ctx.is_admin {
				Admin
			} else {
				Trusted
			},
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			md5: md5_hash,
			sha1: sha1_hash,
			sha256: sha256_hash,
			name,
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.send()
		.await;

	let matched = match result {
		Err(e) => match &e {
			Error::ErrorResponse(e_res) if e_res.status() == StatusCode::NOT_FOUND => {
				ctx.send(components_v2::status_reply(
					Status::Error,
					"No Game found for the provided hashes or name.",
				))
				.await?;
				return Ok(());
			}
			_ => {
				ctx.send(components_v2::status_reply(
					Status::Error,
					format!("Failed to match Game: {e}"),
				))
				.await?;
				warn!("Failed to match Game: {e}");
				return Ok(());
			}
		},
		Ok(value) => value.into_inner().len(),
	};

	let igdb_id_str = igdb_id.to_string();
	let info = igdb::fetch_game(&ctx.data().playmatch_client, &igdb_id_str).await;

	let mut card = Card::new(Status::Success, "Matched Game");
	if let Some(info) = info.as_ref() {
		if let Some(cover) = info.cover_url.clone() {
			card = card.thumbnail(cover);
		}
		card = card.subheading(info.name.clone());
	}
	card = card
		.row("IGDB ID", format!("`{igdb_id_str}`"))
		.row("ROMs updated", matched.to_string());
	if let Some(info) = info.as_ref() {
		card = card.link(CreateButton::new_link(info.page_url.clone()).label("View on IGDB"));
	}

	ctx.send(card.into_reply()).await?;

	Ok(())
}

struct PlaymatchUserCtx {
	is_admin: bool,
	playmatch_user: playmatch_client::types::User,
}

enum SuggestionType {
	Platform,
	Company,
	Game,
}

struct SuggestionMessageHandleData {
	playmatch_client: Arc<playmatch_client::Client>,
	serenity_ctx: Context,
	suggestion_id: Uuid,
	owners: HashSet<UserId>,
	author_id: UserId,
	r#type: SuggestionType,
	name: String,
	platform: Option<String>,
	company: Option<String>,
	comment: Option<String>,
}

async fn handle_suggestion_message(data: SuggestionMessageHandleData) -> CommandResult {
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
		.get_suggestion_by_id()
		.id(data.suggestion_id)
		.send()
		.await?;

	let author = http.get_user(data.author_id).await?;

	let display_type = match data.r#type {
		SuggestionType::Platform => "Platform",
		SuggestionType::Company => "Company",
		SuggestionType::Game => "Game",
	};

	let mut components: Vec<CreateContainerComponent<'static>> = Vec::new();

	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(format!("📝 New {display_type} Metadata Suggestion")),
	));
	components.push(CreateContainerComponent::Separator(CreateSeparator::new()));

	if let Some(company) = &data.company {
		components.push(CreateContainerComponent::TextDisplay(
			CreateTextDisplay::new(format!("**Company:** {company}")),
		));
	}
	if let Some(platform) = &data.platform {
		components.push(CreateContainerComponent::TextDisplay(
			CreateTextDisplay::new(format!("**Platform:** {platform}")),
		));
	}
	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(format!("**{display_type}:** **{}**", data.name)),
	));
	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(format!(
			"**Suggested by:** <@{}> ({})",
			author.id, author.name
		)),
	));
	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new("**Metadata Provider:** IGDB".to_owned()),
	));
	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(format!("**Provider ID:** `{}`", suggestion.provider_id)),
	));
	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(format!(
			"**Comment:** {}",
			data.comment
				.clone()
				.unwrap_or_else(|| "No comment provided".to_string())
		)),
	));
	components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(
			"-# Use the buttons below to approve or decline this suggestion.".to_owned(),
		),
	));
	components.push(CreateContainerComponent::ActionRow(
		CreateActionRow::buttons(vec![
			CreateButton::new("approve")
				.label("Approve")
				.style(ButtonStyle::Success),
			CreateButton::new("decline")
				.label("Decline")
				.style(ButtonStyle::Danger),
		]),
	));

	let container = CreateContainer::new(components).accent_colour(serenity::all::Colour(0x3498DB));

	let message = channel_id
		.widen()
		.send_message(http, components_v2::message_from_container(container))
		.await?;

	let owners = data.owners.clone();
	let interaction_opt = ComponentInteractionCollector::new(&data.serenity_ctx)
		.message_id(message.id)
		.timeout(Duration::from_secs(7 * 24 * 60 * 60))
		.filter(move |i| owners.contains(&i.user.id))
		.await;

	let interaction = match interaction_opt {
		Some(interaction) => interaction,
		None => {
			debug!("No interaction received in time, deleting suggestion message");
			message.delete(http, None).await?;
			return Ok(());
		}
	};

	match &interaction.data.kind {
		ComponentInteractionDataKind::Button => debug!("Interaction data kind: Button"),
		_ => warn!("unexpected interaction data kind"),
	}

	let platform_text = if let Some(platform) = &data.platform {
		format!(" on Platform **{platform}**")
	} else {
		String::new()
	};

	match interaction.data.custom_id.as_str() {
		"approve" => {
			let updated = data
				.playmatch_client
				.approve_suggestion()
				.id(data.suggestion_id)
				.send()
				.await?;

			let message_text = match data.r#type {
				SuggestionType::Platform | SuggestionType::Company => format!(
					"Your Playmatch Metadata Suggestion for {display_type} **{}** was approved! Thank you very much for your contribution 🎉!",
					data.name
				),
				SuggestionType::Game => format!(
					"Your Playmatch Metadata Suggestion for {display_type} **{}**{} was approved! Playmatch was able to match {} game(s) thanks to this! Thank you very much for your contribution 🎉!",
					data.name, platform_text, updated.updated
				),
			};

			let dm_container = CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
				CreateTextDisplay::new(message_text),
			)])
			.accent_colour(serenity::all::Colour(0x2ECC71));
			author
				.id
				.dm(http, components_v2::message_from_container(dm_container))
				.await?;
		}
		"decline" => {
			data.playmatch_client
				.delete_suggestion()
				.id(data.suggestion_id)
				.send()
				.await?;

			let dm_text = format!(
				"Your Playmatch Metadata Suggestion for {display_type} **{}**{} was declined! If you want to find out why, please contact the Playmatch team.",
				data.name, platform_text
			);
			let dm_container = CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
				CreateTextDisplay::new(dm_text),
			)])
			.accent_colour(serenity::all::Colour::RED);
			author
				.id
				.dm(http, components_v2::message_from_container(dm_container))
				.await?;
		}
		_ => {
			warn!("Unexpected button interaction");
			return Ok(());
		}
	};

	if let Err(e) = message.delete(http, None).await {
		error!("{e}");
	}

	Ok(())
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

	let mut playmatch_user_response = ctx
		.data()
		.playmatch_client
		.create_or_get_by_discord_id()
		.body(CreateOrGetUserRequest {
			discord_id: author.id.get() as i64,
			permissions,
			username: author.name.to_string(),
		})
		.send()
		.await?;

	if playmatch_user_response.permissions == UserPermissions::User && (is_trusted || is_admin) {
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
			.update_user_permission_level()
			.id(playmatch_user_response.id)
			.body(UpdateUserPermissionsRequest { new_permission })
			.send()
			.await?;

		debug!("User permission updated for user: {}", author.id);

		playmatch_user_response.permissions = new_permission;
	}

	Ok(PlaymatchUserCtx {
		is_admin,
		playmatch_user: playmatch_user_response.into_inner(),
	})
}
