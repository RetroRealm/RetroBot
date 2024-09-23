pub mod abstraction;
mod command;
mod events;

use crate::command::get_commands;
use dotenvy::dotenv;
use log::info;
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
			commands: get_commands(),
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(abstraction::command::CommandData::default())
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
