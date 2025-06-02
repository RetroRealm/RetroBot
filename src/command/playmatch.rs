use crate::abstraction::command::{CommandContext, CommandResult, is_user_trusted_or_above};
use crate::abstraction::playmatch::paginate_playmatch_response;
use log::warn;
use playmatch_client::types::ManualMatchMode::{Admin, Trusted};
use playmatch_client::types::MatchRequest;
use playmatch_client::types::MetadataProvider::Igdb;

/// Shows a list of playmatch entities with its metadata matches
#[poise::command(
	slash_command,
	category = "Playmatch",
	required_permissions = "SEND_MESSAGES | EMBED_LINKS",
	subcommands("list_companies", "list_platforms")
)]
pub async fn list(_: CommandContext<'_>) -> CommandResult {
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

/// Manually matches a game with a company and platform
#[poise::command(slash_command, category = "Playmatch", rename = "match", check = is_user_trusted_or_above)]
pub async fn manual_match(
	ctx: CommandContext<'_>,
	igdb_id: i64,
	md5_hash: Option<String>,
	sha1_hash: Option<String>,
	sha256_hash: Option<String>,
	name: Option<String>,
) -> CommandResult {
	if md5_hash.is_none() && sha1_hash.is_none() && sha256_hash.is_none() && name.is_none() {
		ctx.reply("You must provide at least one of the following: MD5, SHA1, SHA256 or name")
			.await?;
		return Ok(());
	}

	let is_admin = ctx.framework().options().owners.contains(&ctx.author().id);

	let response = ctx
		.data()
		.playmatch_client
		.match_game(&MatchRequest {
			manual_match_type: if is_admin { Admin } else { Trusted },
			provider_id: igdb_id.to_string(),
			provider: Igdb,
			md5: md5_hash,
			sha1: sha1_hash,
			sha256: sha256_hash,
			name,
			comment: None,
		})
		.await?;

	if !response.status().is_success() {
		ctx.reply("Failed to match game").await?;
		warn!("Failed to match game: {}", response.status());
	}

	ctx.reply(format!(
		"Successfully matched game, Playmatch was able to match {} roms thanks to this!",
		response.into_inner().len()
	))
	.await?;

	Ok(())
}
