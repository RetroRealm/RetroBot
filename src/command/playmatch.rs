use crate::abstraction::command::{CommandContext, CommandResult};
use crate::abstraction::playmatch::paginate_playmatch_response;

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
