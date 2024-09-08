use poise::Command;
use serenity::Error;

pub struct UserData {
	client: reqwest::Client,
}

impl Default for UserData {
	fn default() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}
}

type Context<'a> = poise::Context<'a, UserData, Error>;

pub fn get_commands() -> Vec<Command<UserData, Error>> {
	vec![ping()]
}

/// Responds with pong!
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("pong!").await?;
	Ok(())
}
