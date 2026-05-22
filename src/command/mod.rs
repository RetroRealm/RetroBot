pub(crate) mod playmatch;
mod role;
mod util;

use crate::abstraction::command::{CommandData, CommandError};
use crate::command::playmatch::{get_game_metadata, list, r#match, suggest};
use crate::command::role::toggle_update_role;
use crate::command::util::{help, ping};
use lazy_static::lazy_static;
use poise::Command;

lazy_static! {
	pub static ref RETROREALM_SERVER_ID: u64 = std::env::var("DISCORD_RETROREALM_SERVER_ID")
		.unwrap_or_default()
		.parse()
		.unwrap();
	static ref UPDATE_ROLE_ID: String =
		std::env::var("DISCORD_RETROREALM_UPDATE_ROLE_ID").unwrap_or_default();
	pub(crate) static ref SUGGESTION_CHANNEL_ID: u64 =
		std::env::var("DISCORD_RETROREALM_SUGGESTION_CHANNEL_ID")
			.unwrap_or_default()
			.parse()
			.unwrap();
}

pub fn get_all_commands() -> Vec<Command<CommandData, CommandError>> {
	let mut commands = get_global_commands();
	commands.extend(retrorealm_server_commands());
	commands
}

pub fn get_global_commands() -> Vec<Command<CommandData, CommandError>> {
	vec![
		help(),
		ping(),
		list(),
		r#match(),
		suggest(),
		get_game_metadata(),
	]
}

pub fn retrorealm_server_commands() -> Vec<Command<CommandData, CommandError>> {
	vec![toggle_update_role()]
}
