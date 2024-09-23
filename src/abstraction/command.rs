pub type CommandError = anyhow::Error;
pub type CommandContext<'a> = poise::Context<'a, CommandData, CommandError>;
pub type CommandResult = Result<(), CommandError>;

pub struct CommandData {
	client: reqwest::Client,
}

impl Default for CommandData {
	fn default() -> Self {
		Self {
			client: reqwest::Client::new(),
		}
	}
}
