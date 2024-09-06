use log::{debug, info};
use serenity::all::{Ready, ResumedEvent};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);
	}

	async fn resume(&self, _ctx: Context, _resume: ResumedEvent) {
		debug!("Resumed");
	}
}
