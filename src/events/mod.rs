use crate::abstraction::activity_data::FromStringTuple;
use crate::abstraction::command::CommandData;
use lazy_static::lazy_static;
use log::{debug, info};
use serenity::all::{ActivityData, Context, EventHandler, FullEvent};
use serenity::async_trait;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod suggestion_poller;

pub struct Handler;

lazy_static! {
	static ref DISCORD_STATUS: String = env::var("DISCORD_STATUS").unwrap_or_default();
	static ref DISCORD_STATUS_NAME: String = env::var("DISCORD_STATUS_NAME").unwrap_or_default();
}

static POLLER_STARTED: AtomicBool = AtomicBool::new(false);

#[async_trait]
impl EventHandler for Handler {
	async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
		match event {
			FullEvent::Ready { data_about_bot, .. } => {
				info!("{} is connected!", data_about_bot.user.name);

				if !DISCORD_STATUS.is_empty() && !DISCORD_STATUS_NAME.is_empty() {
					ctx.set_activity(Some(ActivityData::from_tuple(
						&DISCORD_STATUS,
						&DISCORD_STATUS_NAME,
					)));
				}

				if !POLLER_STARTED.swap(true, Ordering::SeqCst) {
					let ctx = ctx.clone();
					let data = ctx.data::<CommandData>();
					tokio::spawn(async move {
						suggestion_poller::run(ctx, data).await;
					});
				}
			}
			FullEvent::Resume { .. } => {
				debug!("Resumed");
			}
			_ => {}
		}
	}
}
