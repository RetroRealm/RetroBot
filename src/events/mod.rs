use log::{debug, info};
use serenity::all::{Ready, ResumedEvent};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		// Log at the INFO level. This is a macro from the `tracing` crate.
		info!("{} is connected!", ready.user.name);
	}

	// For instrument to work, all parameters must implement Debug.
	//
	// Handler doesn't implement Debug here, so we specify to skip that argument.
	// Context doesn't implement Debug either, so it is also skipped.
	async fn resume(&self, _ctx: Context, _resume: ResumedEvent) {
		// Log at the DEBUG level.
		//
		// In this example, this will not show up in the logs because DEBUG is
		// below INFO, which is the set debug level.
		debug!("Resumed");
	}
}
