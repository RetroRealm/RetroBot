pub mod abstraction;
mod command;
mod events;
pub mod util;

use crate::command::{
	RETROREALM_SERVER_ID, get_all_commands, get_global_commands, retrorealm_server_commands,
};
use abstraction::command::CommandData;
use dotenvy::dotenv;
use log::info;
use serenity::all::GuildId;
use serenity::prelude::GatewayIntents;

pub mod built_info {
	// The file has been placed there by the build script.
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load environment variables from .env file, if present but do nothing if it fails
	let _ = dotenv();
	tracing_subscriber::fmt::init();

	info!(
		"Starting {} v{} ({}) built on {}",
		built_info::PKG_NAME,
		built_info::PKG_VERSION,
		built_info::GIT_COMMIT_HASH.unwrap_or("build commit unknown"),
		built_info::BUILT_TIME_UTC
	);

	let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

	let intents = GatewayIntents::non_privileged();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: get_all_commands(),
			post_command: |ctx| {
				Box::pin(async move {
					let author = ctx.author();
					let guild = ctx.guild();
					let cmd = ctx.command();

					let user_info = format!("{}[{}]", author.name, author.id);

					let guild_info = if let Some(guild) = guild {
						format!("{}[{}]", guild.name, guild.id)
					} else {
						"Direct Messages".to_string()
					};

					let command_name = cmd.name.to_lowercase();

					info!("{user_info} @ {guild_info} {command_name}");
				})
			},
			..Default::default()
		})
		.setup(|ctx, _ready, _framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, get_global_commands().as_slice()).await?;

				poise::builtins::register_in_guild(
					ctx,
					retrorealm_server_commands().as_slice(),
					GuildId::from(*RETROREALM_SERVER_ID),
				)
				.await?;

				Ok(CommandData::default())
			})
		})
		.initialize_owners(true)
		.build();

	let mut client = serenity::Client::builder(token, intents)
		.event_handler(events::Handler)
		.framework(framework)
		.await?;

	client.start().await?;

	Ok(())
}
