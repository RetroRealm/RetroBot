use crate::abstraction::activity_data::FromStringTuple;
use crate::abstraction::command::CommandData;
use crate::util::discord_ratelimit::DiscordCooldown;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use serenity::all::{ActivityData, Context, EventHandler, FullEvent, RatelimitInfo};
use serenity::async_trait;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Handler {
	/// Shared with `CommandData`; armed here from `ratelimit`, honored by the poller.
	pub discord_cooldown: Arc<DiscordCooldown>,
}

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
						crate::suggestions::run(ctx, data).await;
					});
				}
			}
			FullEvent::Resume { .. } => {
				debug!("Resumed");
			}
			_ => {}
		}
	}

	/// serenity calls this for every 429 it sees. We log each one at warn (serenity itself
	/// only logs them at debug, so this is the operator's one window into rate limiting) and
	/// arm the global stand-down when the wait is ban-length, so the poller stops feeding
	/// the ban.
	async fn ratelimit(&self, info: RatelimitInfo) {
		warn!(
			"discord ratelimit: {:?} {} (global={}, wait {:.1}s)",
			info.method,
			info.path,
			info.global,
			info.timeout.as_secs_f64()
		);
		self.discord_cooldown.record(info.timeout);
	}
}
