use crate::abstraction::command::{
	CommandContext, CommandData, CommandResult, TRUSTED_ROLE_IDS, is_user_trusted_or_above,
};
use crate::abstraction::components_v2::{self, Card, Status};
use crate::abstraction::playmatch::{
	ApiErrorAction, ConciseError, paginate_playmatch_response, send_playmatch_api_error,
};
use crate::abstraction::providers::{self, ENRICHMENT_PRIORITY, ProviderChoice, display_name};
use anyhow::anyhow;
use log::{debug, error};
use playmatch_client::types::ManualMatchMode::{Admin, Trusted};
use playmatch_client::types::{
	CompanyOrPlatformMatchRequest, CompanyOrPlatformSuggestionRequest, CreateOrGetUserRequestV2,
	GameMatchRequest, GameMatchType, GameSuggestionRequest, ManualMatchMode, MetadataMatchType,
	UpdateUserPermissionsRequestV2, UserPermissions,
};

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

/// Post a just-created suggestion's card immediately, in the background, attributing it to
/// the command author. The card post is a side effect of the command, so it runs detached:
/// the command replies to the user without waiting for Discord.
fn post_suggestion_card(ctx: &CommandContext<'_>, suggestion: playmatch_client::types::Suggestion) {
	let serenity_ctx = ctx.serenity_context().clone();
	let data = serenity_ctx.data::<CommandData>();
	let author_id = ctx.author().id;
	tokio::spawn(async move {
		crate::suggestions::post_immediately(
			&serenity_ctx,
			&data,
			suggestion,
			crate::suggestions::card::SuggestionSubmitter::DiscordUser(author_id),
		)
		.await;
	});
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

	post_suggestion_card(&ctx, suggestion);

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

	post_suggestion_card(&ctx, suggestion);

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

	if suggestion.platform_id.is_none() {
		error!("Platform ID is missing in the suggestion response! This should not happen.");
		ctx.send(components_v2::status_reply(
			Status::Error,
			"Internal error: platform ID missing from suggestion response.",
		))
		.await?;
		return Ok(());
	}

	post_suggestion_card(&ctx, suggestion);

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
