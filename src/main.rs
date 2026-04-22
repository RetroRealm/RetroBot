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
use serenity::all::{GuildId, Http, Token};
use serenity::prelude::GatewayIntents;
use std::sync::Arc;

pub mod built_info {
	// The file has been placed there by the build script.
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let _ = dotenv();
	tracing_subscriber::fmt::init();

	info!(
		"Starting {} v{} ({}) built on {}",
		built_info::PKG_NAME,
		built_info::PKG_VERSION,
		built_info::GIT_COMMIT_HASH.unwrap_or("build commit unknown"),
		built_info::BUILT_TIME_UTC
	);

	let token = Token::from_env("DISCORD_TOKEN")?;
	let intents = GatewayIntents::non_privileged();

	let http = Http::new(token.clone());
	poise::builtins::register_globally(&http, get_global_commands().iter()).await?;
	poise::builtins::register_in_guild(
		&http,
		retrorealm_server_commands().iter(),
		GuildId::new(*RETROREALM_SERVER_ID),
	)
	.await?;

	let options = poise::FrameworkOptions {
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
	};

	let framework = poise::Framework::new(options);

	let client = serenity::Client::builder(token, intents)
		.framework(Box::new(framework))
		.event_handler(Arc::new(events::Handler))
		.data(Arc::new(CommandData::default()) as _)
		.await;

	client?.start().await?;

	Ok(())
}
