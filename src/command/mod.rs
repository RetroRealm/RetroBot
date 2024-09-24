mod playmatch;
mod role;
mod util;

use crate::abstraction::command::{CommandData, CommandError};
use crate::command::playmatch::list;
use crate::command::role::toggle_update_role;
use crate::command::util::{help, ping};
use lazy_static::lazy_static;
use poise::Command;

lazy_static! {
	static ref UPDATE_ROLE_ID: String =
		std::env::var("DISCORD_RETROREALM_UPDATE_ROLE_ID").unwrap_or_default();
}

pub fn get_commands() -> Vec<Command<CommandData, CommandError>> {
	vec![help(), ping(), toggle_update_role(), list()]
}
