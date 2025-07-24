use crate::abstraction::command::{
	CommandContext, CommandResult, TRUSTED_ROLE_IDS, is_user_trusted_or_above,
};
use crate::abstraction::playmatch::paginate_playmatch_response;
use crate::command::SUGGESTION_CHANNEL_ID;
use anyhow::anyhow;
use log::{debug, error, warn};
use playmatch_client::Error;
use playmatch_client::types::ManualMatchMode::{Admin, Trusted};
use playmatch_client::types::MetadataProvider::Igdb;
use playmatch_client::types::{
	CompanyOrPlatformMatchRequest, CompanyOrPlatformSuggestionRequest, CreateOrGetUserRequest,
	GameMatchRequest, GameMatchType, GameSuggestionRequest, UpdateUserPermissionsRequest,
	UserPermissions,
};
use poise::CreateReply;
use reqwest::StatusCode;
use serenity::all::{
	ButtonStyle, Cache, ChannelId, ComponentInteractionDataKind, CreateEmbedFooter, ShardMessenger,
	UserId,
};
use serenity::builder::{CreateButton, CreateEmbed, CreateMessage};
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
	let response = ctx.data().playmatch_client.get_all_companies().await?;

	let companies = response.into_inner();

	paginate_playmatch_response(ctx, companies).await
}

/// Shows a list of platforms with its metadata matches
#[poise::command(slash_command, category = "Playmatch", rename = "platforms")]
pub async fn list_platforms(ctx: CommandContext<'_>) -> CommandResult {
	let response = ctx.data().playmatch_client.get_all_platforms().await?;

	let companies = response.into_inner();

	paginate_playmatch_response(ctx, companies).await
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
		.identify_game_and_relations(
			&file_name,
			file_size,
			md5_hash.as_deref(),
			sha1_hash.as_deref(),
			sha256_hash.as_deref(),
		)
		.await?;

	let inner = response.into_inner();

	if inner.game_match_type == GameMatchType::NoMatch {
		ctx.reply("No matching game found for the provided hashes or file name and size.")
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

	let files_info = game_files
		.iter()
		.enumerate()
		.map(|(i, file)| {
			let mut out = format!("**{}. {}**", i + 1, file.file_name);
			if let Some(size) = file.file_size_in_bytes {
				out.push_str(&format!("Size: `{:.2} MB`", size as f64 / 1024.0 / 1024.0));
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

	let dat_file_value = match dat_file.tags {
		None => format!(
			"**{}**\nCurrent Version: `{}`\n",
			dat_file.name, dat_file.current_version
		),
		Some(tags) => format!(
			"**{}**\nCurrent Version: `{}`\nTags: `{}`",
			dat_file.name,
			dat_file.current_version,
			tags.join(", ")
		),
	};

	let signature_group_value = match signature_group.website_link {
		None => signature_group.name,
		Some(website_link) => format!("[{}]({})", signature_group.name, website_link),
	};

	// Build the embed
	let mut embed = CreateEmbed::new()
		.title(game.name)
		.color(0x2ecc71)
		.field("Match Type", format!("`{}`", inner.game_match_type), true)
		.field("Platform", platform.name, true);

	if let Some(c) = company {
		embed = embed.field("Company", c.name, true)
	}

	embed = embed
		.field("ROM Files", files_info, false)
		.field("DAT File", dat_file_value, true)
		.field("Signature Group", signature_group_value, false);

	for metadata_mapping in metadata_mappings {
		embed = embed.field(
			format!("{} Mapping:", metadata_mapping.provider_name),
			format!(
				"Status: `{}`{}",
				metadata_mapping.match_type,
				if let Some(provider_id) = metadata_mapping.provider_id {
					format!(
						"\nMatch Type: `{}`\nProvider ID: `{}`",
						metadata_mapping.match_type, provider_id
					)
				} else {
					"".to_string()
				}
			),
			true,
		);
	}

	embed = embed.footer(CreateEmbedFooter::new(format!(
		"Playmatch Game ID: {}",
		game.id
	)));

	ctx.send(CreateReply::default().embed(embed)).await?;

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
		ctx.reply("You must provide at least one of the following: MD5, SHA1, SHA256 or name")
			.await?;
		return Ok(());
	}

	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.create_game_suggestion(&GameSuggestionRequest {
			provider_id: igdb_id.to_string(),
			sha1: sha1_hash,
			provider: Igdb,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
			md5: md5_hash,
			sha256: sha256_hash,
		})
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value.into_inner(),
		Err(e) => {
			match e {
				Error::ErrorResponse(e_res) => {
					if e_res.status() == StatusCode::NOT_FOUND {
						ctx.reply("No Game found with the provided hashes or names")
							.await?;
					} else if e_res.status() == StatusCode::CONFLICT {
						ctx.reply(
							"A suggestion for this Game already exists with the same provider and provider id.",
						)
						.await?;
					}
				}
				_ => {
					ctx.reply(format!("Failed to create a suggestion for Game: {e}"))
						.await?;
					warn!("Failed to create a suggestion for Game: {e}");
				}
			}
			return Ok(());
		}
	};

	let game = match suggestion.game_id {
		Some(game_id) => game_id,
		None => {
			error!("Game ID is missing in the suggestion response! This should not happen.");
			ctx.reply("Failed to create a suggestion for Game: Game ID is missing in the suggestion response! This should not happen.")
				.await?;
			return Ok(());
		}
	};

	let game_response = ctx
		.data()
		.playmatch_client
		.get_playmatch_game_with_relations_by_id(&game)
		.await?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let http = ctx.serenity_context().http.clone();
		let cache = ctx.serenity_context().cache.clone();
		let shard_messenger = ctx.serenity_context().shard.clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let game_name = game_response.game.name.clone();
		let platform = game_response.platform.name.clone();
		let company = game_response.company.clone().map(|c| c.name);
		async move {
			handle_suggestion_message(SuggestionMessageHandleData {
				playmatch_client,
				http,
				cache,
				shard_messenger,
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

	ctx.reply(format!(
		"Successfully created suggestion for Game {}, Thank you for your contribution 🎉!\nWe will notify you once it has been approved or rejected.",
		&game_response.game.name
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
		.create_company_suggestion(&CompanyOrPlatformSuggestionRequest {
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value.into_inner(),
		Err(e) => {
			match e {
				Error::ErrorResponse(e_res) => {
					if e_res.status() == StatusCode::NOT_FOUND {
						ctx.reply("No Company found with the provided name").await?;
					} else if e_res.status() == StatusCode::CONFLICT {
						ctx.reply(
							"A suggestion for this Company already exists with the same provider and provider id.",
						)
						.await?;
					}
				}
				_ => {
					ctx.reply(format!("Failed to create a suggestion for Company: {e}"))
						.await?;
					warn!("Failed to create a suggestion for Company: {e}");
				}
			}
			return Ok(());
		}
	};

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let http = ctx.serenity_context().http.clone();
		let cache = ctx.serenity_context().cache.clone();
		let shard_messenger = ctx.serenity_context().shard.clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let company_name = name.clone();
		async move {
			handle_suggestion_message(SuggestionMessageHandleData {
				playmatch_client,
				http,
				cache,
				shard_messenger,
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

	ctx.reply(format!(
		"Successfully created suggestion for Platform {}, Thank you for your contribution 🎉!\nWe will notify you once it has been approved or rejected.",
		&name
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
		.create_platform_suggestion(&CompanyOrPlatformSuggestionRequest {
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			name: name.clone(),
			comment,
			user_id: Some(playmatch_user_ctx.playmatch_user.id),
		})
		.await;

	let suggestion = match result {
		Ok(suggestion_value) => suggestion_value.into_inner(),
		Err(e) => {
			match e {
				Error::ErrorResponse(e_res) => {
					if e_res.status() == StatusCode::NOT_FOUND {
						ctx.reply("No Platform found with the provided name")
							.await?;
					} else if e_res.status() == StatusCode::CONFLICT {
						ctx.reply(
							"A suggestion for this Platform already exists with the same provider and provider id.",
						)
						.await?;
					}
				}
				_ => {
					ctx.reply(format!("Failed to create a suggestion for Platform: {e}"))
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
		.get_platform_by_id(&platform_id)
		.await?;

	tokio::spawn({
		let playmatch_client = ctx.data().playmatch_client.clone();
		let http = ctx.serenity_context().http.clone();
		let cache = ctx.serenity_context().cache.clone();
		let shard_messenger = ctx.serenity_context().shard.clone();
		let owners = ctx.framework().options().owners.clone();
		let author_id = ctx.author().id;
		let platform_name = name.clone();
		let company = platform.company_name.clone();
		async move {
			handle_suggestion_message(SuggestionMessageHandleData {
				playmatch_client,
				http,
				cache,
				shard_messenger,
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

	ctx.reply(format!(
		"Successfully created suggestion for Platform {}, Thank you for your contribution 🎉!\nWe will notify you once it has been approved or rejected.",
		&name
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
		.manually_match_platform(&CompanyOrPlatformMatchRequest {
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
		.await;

	if let Err(e) = &result {
		match e {
			Error::ErrorResponse(e_res) => {
				if e_res.status() == StatusCode::NOT_FOUND {
					ctx.reply("No Platform found with the provided name")
						.await?;
					return Ok(());
				}
			}
			_ => {
				ctx.reply(format!("Failed to match Platform: {e}")).await?;
				warn!("Failed to match Platform: {e}");
				return Ok(());
			}
		}
	}

	ctx.reply("Successfully matched Platform, Thank you for your contribution 🎉!")
		.await?;

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
		.manually_match_company(&CompanyOrPlatformMatchRequest {
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
		.await;

	if let Err(e) = &result {
		match e {
			Error::ErrorResponse(e_res) => {
				if e_res.status() == StatusCode::NOT_FOUND {
					ctx.reply("No Company found with the provided name").await?;
					return Ok(());
				}
			}
			_ => {
				ctx.reply(format!("Failed to match Company: {e}")).await?;
				warn!("Failed to match Company: {e}");
				return Ok(());
			}
		}
	}

	ctx.reply("Successfully matched Company, Thank you for your contribution 🎉!")
		.await?;

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
		ctx.reply("You must provide at least one of the following: MD5, SHA1, SHA256 or name")
			.await?;
		return Ok(());
	}

	let playmatch_user_ctx = get_playmatch_user_ctx(ctx).await?;

	let result = ctx
		.data()
		.playmatch_client
		.manually_match_game(&GameMatchRequest {
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
		.await;

	if let Err(e) = &result {
		match e {
			Error::ErrorResponse(e_res) => {
				if e_res.status() == StatusCode::NOT_FOUND {
					ctx.reply("No game found with the provided hash or name")
						.await?;
					return Ok(());
				}
			}
			_ => {
				ctx.reply(format!("Failed to match game: {e}")).await?;
				warn!("Failed to match game: {e}");
				return Ok(());
			}
		}
	}

	ctx.reply(format!(
		"Successfully matched game, Playmatch matched {} roms thanks to you 🎉!",
		result?.into_inner().len()
	))
	.await?;

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
	http: Arc<Http>,
	cache: Arc<Cache>,
	shard_messenger: ShardMessenger,
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
	let channel_opt = data.cache.guilds().iter().find_map(|guild_id| {
		data.cache.guild(*guild_id).and_then(|guild| {
			guild
				.channels
				.get(&ChannelId::from(*SUGGESTION_CHANNEL_ID))
				.cloned()
		})
	});

	let channel = channel_opt.ok_or_else(|| anyhow!("Suggestion channel not found"))?;

	let suggestion = data
		.playmatch_client
		.get_suggestion_by_id(&data.suggestion_id)
		.await?;

	let author = data.http.get_user(data.author_id).await?;

	let display_type = match data.r#type {
		SuggestionType::Platform => "Platform",
		SuggestionType::Company => "Company",
		SuggestionType::Game => "Game",
	};

	let mut embed = CreateEmbed::new()
		.title(format!("📝 New {} Metadata Suggestion", display_type))
		.color(0x3498DB);

	if let Some(company) = &data.company {
		embed = embed.field("Company", company, true);
	}

	if let Some(platform) = &data.platform {
		embed = embed.field("Platform", platform, true);
	}

	embed = embed
		.field(display_type.to_string(), format!("**{}**", data.name), true)
		.field(
			"Suggested by",
			format!("<@{}> ({})", author.id, author.name),
			false,
		)
		.field("Metadata Provider", "IGDB", true)
		.field("Provider ID", format!("`{}`", suggestion.provider_id), true)
		.field(
			"Comment",
			data.comment
				.unwrap_or_else(|| "No comment provided".to_string()),
			false,
		)
		.footer(CreateEmbedFooter::new(
			"Use the buttons below to approve or decline this suggestion.",
		));

	let message = channel
		.send_message(
			&data.http,
			CreateMessage::new()
				.embed(embed)
				.button(
					CreateButton::new("approve")
						.label("Approve")
						.style(ButtonStyle::Success),
				)
				.button(
					CreateButton::new("decline")
						.label("Decline")
						.style(ButtonStyle::Danger),
				),
		)
		.await?;

	let interaction_opt = message
		.await_component_interaction(data.shard_messenger)
		.filter(move |i| data.owners.clone().contains(&i.user.id))
		.timeout(Duration::from_secs(7 * 24 * 60)) // 7 days
		.await;

	let interaction = match interaction_opt {
		Some(interaction) => interaction,
		None => {
			debug!("No interaction received in time, deleting suggestion");
			message.delete(&data.http).await?;
			return Ok(());
		}
	};

	match &interaction.data.kind {
		ComponentInteractionDataKind::Button => {
			debug!("Interaction data kind: Button");
		}
		_ => warn!("unexpected interaction data kind"),
	};

	let platform_text = if let Some(platform) = &data.platform {
		format!(" on Platform **{platform}**")
	} else {
		String::new()
	};

	match interaction.data.custom_id.as_str() {
		"approve" => {
			let updated = data
				.playmatch_client
				.approve_suggestion(&data.suggestion_id)
				.await?;

			author
				.dm(
					&data.http,
					CreateMessage::new().embed(
						CreateEmbed::new().description(
							format!("Your Playmatch Metadata Suggestion for {display_type} **{}**{} was approved! Playmatch was able to match {} game(s) thanks to you. Thank you very much for your contribution 🎉!", data.name, platform_text, updated.updated)
						),
					),
				)
				.await?;
		}
		"decline" => {
			data.playmatch_client
				.delete_suggestion(&data.suggestion_id)
				.await?;

			author
				.dm(
					&data.http,
					CreateMessage::new()
						.embed(CreateEmbed::new().description(format!("Your Playmatch Metadata Suggestion for {display_type} **{}**{}  was declined! If you want to find out why, please contact the Playmatch team.", data.name, platform_text))),
				)
				.await?;
		}
		_ => {
			warn!("Unexpected button interaction");
			return Ok(());
		}
	};

	match message.delete(&data.http).await {
		Ok(_) => {}
		Err(e) => {
			error!("{e}")
		}
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
		.create_or_get_by_discord_id(&CreateOrGetUserRequest {
			discord_id: author.id.get() as i64,
			permissions,
			username: author.name.clone(),
		})
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
			.update_user_permission_level(
				&playmatch_user_response.id,
				&UpdateUserPermissionsRequest { new_permission },
			)
			.await?;

		debug!("User permission updated for user: {}", author.id);

		playmatch_user_response.permissions = new_permission;
	}

	Ok(PlaymatchUserCtx {
		is_admin,
		playmatch_user: playmatch_user_response.into_inner(),
	})
}
