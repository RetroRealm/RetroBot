use crate::abstraction::activity_data::FromStringTuple;
use lazy_static::lazy_static;
use log::{debug, info};
use serenity::all::{ActivityData, Ready, ResumedEvent};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};
use std::env;

pub struct Handler;

lazy_static! {
	static ref DISCORD_STATUS: String = env::var("DISCORD_STATUS").unwrap_or_default();
	static ref DISCORD_STATUS_NAME: String = env::var("DISCORD_STATUS_NAME").unwrap_or_default();
}

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);

		if !DISCORD_STATUS.is_empty() && !DISCORD_STATUS_NAME.is_empty() {
			ctx.set_activity(Some(ActivityData::from_tuple(
				&DISCORD_STATUS,
				&DISCORD_STATUS_NAME,
			)));
		};
	}

	async fn resume(&self, _ctx: Context, _resume: ResumedEvent) {
		debug!("Resumed");
	}
}
