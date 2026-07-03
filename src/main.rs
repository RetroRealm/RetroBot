pub mod abstraction;
mod command;
mod events;
pub mod util;

use crate::command::{
	RETROREALM_SERVER_ID, get_all_commands, get_global_commands, retrorealm_server_commands,
};
use crate::util::retry::{backoff_delay, jittered, retry_discord_startup};
use abstraction::command::CommandData;
use dotenvy::dotenv;
use log::{error, info};
use serenity::all::{GuildId, Http, Token};
use serenity::prelude::GatewayIntents;
use std::sync::Arc;
use std::time::{Duration, Instant};

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

	// Build the local dependencies (Redis + reqwest) before touching Discord. If Redis is
	// down the process crash-loops here without ever sending a Discord request, so a local
	// outage can never feed the invalid-request ban counter. Redis failure stays
	// fatal-fast: it is a local dependency and container restart is the right remedy.
	let data = Arc::new(CommandData::new().await?);

	let http = Http::new(token.clone());

	// The three startup REST calls retry in-process instead of propagating: a Discord
	// error here must not exit and hand control to the container restart loop (see
	// util::retry). Command lists are built once and reborrowed per attempt.
	let app_info =
		retry_discord_startup("application info", || http.get_current_application_info()).await;
	http.set_application_id(app_info.id);

	let global_commands = get_global_commands();
	retry_discord_startup("global command registration", || {
		poise::builtins::register_globally(&http, global_commands.iter())
	})
	.await;

	let guild_commands = retrorealm_server_commands();
	let guild_id = GuildId::new(*RETROREALM_SERVER_ID);
	retry_discord_startup("guild command registration", || {
		poise::builtins::register_in_guild(&http, guild_commands.iter(), guild_id)
	})
	.await;

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

	let mut client = serenity::Client::builder(token, intents)
		.framework(Box::new(framework))
		.event_handler(Arc::new(events::Handler))
		.data(data as _)
		.await?;

	// Gateway loop: start() is re-callable (&mut self). On error, back off with jitter
	// and retry instead of exiting; a clean Ok(()) exits normally. Retrying in-process
	// protects the 1000-IDENTIFYs/24h budget (worst case ~288 attempts/day at the ceiling
	// vs 10k+ from the container loop). The attempt counter resets after a session that
	// stayed up >=5 min: a long-lived connection dropping is a fresh incident, not
	// attempt N+1.
	let mut attempt: u32 = 0;
	loop {
		let started = Instant::now();
		match client.start().await {
			Ok(()) => return Ok(()),
			Err(e) => {
				if started.elapsed() >= Duration::from_secs(300) {
					attempt = 0;
				}
				let delay = jittered(backoff_delay(attempt, false));
				error!(
					"gateway: session ended, reconnecting in {:.1}s (attempt {attempt}): {e}",
					delay.as_secs_f64()
				);
				tokio::time::sleep(delay).await;
				attempt = attempt.saturating_add(1);
			}
		}
	}
}
