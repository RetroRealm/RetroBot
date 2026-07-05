//! A global stand-down gate for Discord requests the bot initiates.
//!
//! serenity's ratelimiter waits out a 429 on the offending route, but it does not gate
//! across routes, across the gateway reconnect loop, or across a second `Http` instance,
//! and it silently retries. That is fine for ordinary per-route buckets, but when the host
//! IP trips Discord's invalid-request cloud ban (a 429 from Cloudflare with a retry-after
//! of many seconds and no per-route headers) every further request only extends the ban.
//! Driven by `EventHandler::ratelimit`, this gate records the ban deadline once and holds
//! all of our own maintenance traffic until it clears.

use std::sync::Mutex;
use std::time::Duration;

use tokio::time::Instant;

/// Waits shorter than this are ordinary per-route bucket pacing that serenity already
/// handles; only a wait at least this long looks like the edge ban worth halting for.
const BAN_THRESHOLD: Duration = Duration::from_secs(30);

#[derive(Default)]
pub struct DiscordCooldown {
	/// Deadline until which our own Discord traffic stands down. `None` or a past instant
	/// means clear.
	until: Mutex<Option<Instant>>,
}

impl DiscordCooldown {
	pub fn new() -> Self {
		Self::default()
	}

	/// Record a rate-limit wait reported by serenity. Only ban-length waits arm the gate,
	/// and the deadline only ever moves later, never earlier.
	pub fn record(&self, timeout: Duration) {
		if timeout < BAN_THRESHOLD {
			return;
		}
		let target = Instant::now() + timeout;
		let mut until = self.until.lock().unwrap();
		if until.is_none_or(|t| t < target) {
			*until = Some(target);
		}
	}

	/// Block until the gate is clear. Loops because a fresh 429 can push the deadline out
	/// while we sleep.
	pub async fn wait(&self) {
		loop {
			let until = *self.until.lock().unwrap();
			match until {
				Some(t) if t > Instant::now() => tokio::time::sleep_until(t).await,
				_ => return,
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn short_waits_do_not_arm_the_gate() {
		let cd = DiscordCooldown::new();
		cd.record(Duration::from_secs(5));
		assert!(cd.until.lock().unwrap().is_none());
	}

	#[tokio::test]
	async fn ban_length_waits_arm_and_only_extend() {
		let cd = DiscordCooldown::new();
		cd.record(Duration::from_secs(60));
		let first = cd.until.lock().unwrap().unwrap();
		// A shorter follow-up must not pull the deadline in.
		cd.record(Duration::from_secs(31));
		assert_eq!(cd.until.lock().unwrap().unwrap(), first);
		// A longer one pushes it out.
		cd.record(Duration::from_secs(120));
		assert!(cd.until.lock().unwrap().unwrap() > first);
	}

	#[tokio::test]
	async fn wait_returns_immediately_when_clear() {
		let cd = DiscordCooldown::new();
		// Would hang if the gate were armed; a plain await proves it returns at once.
		cd.wait().await;
	}
}
