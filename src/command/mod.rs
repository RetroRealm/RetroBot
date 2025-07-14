mod playmatch;
mod role;
mod util;

use crate::abstraction::command::{CommandData, CommandError};
use crate::command::playmatch::{list, r#match, suggest};
use crate::command::role::toggle_update_role;
use crate::command::util::{help, ping};
use lazy_static::lazy_static;
use poise::Command;

lazy_static! {
	static ref RETROREALM_SERVER_ID: u64 = std::env::var("DISCORD_RETROREALM_SERVER_ID")
		.unwrap_or_default()
		.parse()
		.unwrap();
	static ref UPDATE_ROLE_ID: String =
		std::env::var("DISCORD_RETROREALM_UPDATE_ROLE_ID").unwrap_or_default();
	static ref SUGGESTION_CHANNEL_ID: u64 =
		std::env::var("DISCORD_RETROREALM_SUGGESTION_CHANNEL_ID")
			.unwrap_or_default()
			.parse()
			.unwrap();
}

pub fn get_commands() -> Vec<Command<CommandData, CommandError>> {
	vec![
		help(),
		ping(),
		toggle_update_role(),
		list(),
		r#match(),
		suggest(),
	]
}
