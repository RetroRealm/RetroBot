//! In-process startup retry for Discord REST calls. A Discord error at boot must never
//! terminate the process: exiting hands control to the container's fast restart loop,
//! which discards serenity's learned rate-limit state and re-fires the same requests
//! every few seconds. Cloudflare's own guidance is that repeated access during a 1015
//! block extends it, so the restart loop sustains the very ban it is reacting to. Instead
//! we retry forever with capped exponential backoff plus jitter, so a block decays.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use log::{error, info};
use reqwest::StatusCode;

/// First backoff step; doubles each attempt.
const BASE_DELAY: Duration = Duration::from_secs(2);
/// Ceiling. At 300 s a blocked boot idles at <=12 requests/hour, letting a Cloudflare
/// 1015 age out instead of being fed.
const MAX_DELAY: Duration = Duration::from_secs(300);
/// Backoff floor once we have seen a 429: a self-clocked wait for a ban response that
/// carries no retry_after header at our serenity rev. This floors the pre-jitter base, so
/// equal-jitter then yields an actual sleep in `[30s, 60s)` — still two orders of magnitude
/// above the 1.5-8s container restart loop that fed the original block.
const RATE_LIMITED_FLOOR: Duration = Duration::from_secs(60);

/// Deterministic backoff for a given attempt (0-based). `2s << attempt`, saturated at
/// 300 s; floored at 60 s when the last error was a 429.
pub fn backoff_delay(attempt: u32, rate_limited: bool) -> Duration {
	let raw = BASE_DELAY
		.checked_mul(1u32.checked_shl(attempt).unwrap_or(u32::MAX))
		.unwrap_or(MAX_DELAY)
		.min(MAX_DELAY);
	if rate_limited {
		raw.max(RATE_LIMITED_FLOOR)
	} else {
		raw
	}
}

/// Equal-jitter: keep half the delay fixed, randomize the other half into `[d/2, d)`.
/// Pure so it is testable; `entropy` supplies the randomness.
fn jittered_from(delay: Duration, entropy: u64) -> Duration {
	let half = delay / 2;
	let span = delay.as_nanos().saturating_sub(half.as_nanos());
	if span == 0 {
		return half;
	}
	let offset = (entropy as u128) % span;
	half + Duration::from_nanos(offset as u64)
}

/// `jittered_from` fed live entropy from the system clock. Avoids pulling in `rand` for a
/// single jitter call. We fold the full nanosecond count (not just `subsec_nanos`, which is
/// always < 1s and would confine the jitter to the bottom second of the range) so the
/// randomized half actually spans `[0, delay/2)`.
pub fn jittered(delay: Duration) -> Duration {
	let entropy = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map(|d| d.as_nanos() as u64)
		.unwrap_or(0);
	jittered_from(delay, entropy)
}

/// True for a 429 that reaches us as `Error::Http`. serenity handles most 429s internally
/// (a global 429, or an edge ban that carries `retry-after`, is slept-then-retried inside
/// its ratelimiter and never surfaces), so this only catches the residual shape: an edge
/// 429 with no parseable `retry-after`. The global stand-down driven by
/// `EventHandler::ratelimit` is what actually reacts to the common (retry-after-bearing)
/// ban; this floor is the fallback for the header-less variant.
pub fn is_rate_limited(e: &serenity::Error) -> bool {
	matches!(
		e,
		serenity::Error::Http(http) if http.status_code() == Some(StatusCode::TOO_MANY_REQUESTS)
	)
}

/// Runs `f` until it succeeds, backing off with jitter between attempts and never
/// returning an error. `label` names the call in the log narrative.
pub async fn retry_discord_startup<T, F, Fut>(label: &str, mut f: F) -> T
where
	F: FnMut() -> Fut,
	Fut: std::future::Future<Output = serenity::Result<T>>,
{
	let mut attempt: u32 = 0;
	loop {
		match f().await {
			Ok(value) => {
				if attempt > 0 {
					info!("{label}: succeeded after {attempt} failed attempt(s)");
				}
				return value;
			}
			Err(e) => {
				let rate_limited = is_rate_limited(&e);
				let delay = jittered(backoff_delay(attempt, rate_limited));
				match e {
					serenity::Error::Http(ref http) if http.status_code().is_some() => error!(
						"{label}: attempt {attempt} failed (HTTP {}), retrying in {:.1}s: {e}",
						http.status_code().unwrap(),
						delay.as_secs_f64()
					),
					_ => error!(
						"{label}: attempt {attempt} failed, retrying in {:.1}s: {e}",
						delay.as_secs_f64()
					),
				}
				tokio::time::sleep(delay).await;
				attempt = attempt.saturating_add(1);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn backoff_growth_and_cap() {
		assert_eq!(backoff_delay(0, false), Duration::from_secs(2));
		assert_eq!(backoff_delay(1, false), Duration::from_secs(4));
		assert_eq!(backoff_delay(2, false), Duration::from_secs(8));
		assert_eq!(backoff_delay(3, false), Duration::from_secs(16));
		// Large attempts saturate at the ceiling instead of overflowing.
		assert_eq!(backoff_delay(20, false), MAX_DELAY);
		assert_eq!(backoff_delay(64, false), MAX_DELAY);
	}

	#[test]
	fn rate_limited_floor() {
		// Early attempts floor at 60 s.
		assert_eq!(backoff_delay(0, true), RATE_LIMITED_FLOOR);
		assert_eq!(backoff_delay(3, true), RATE_LIMITED_FLOOR);
		// Above the floor the raw value wins; still capped at 300 s.
		assert_eq!(backoff_delay(6, true), Duration::from_secs(128));
		assert_eq!(backoff_delay(20, true), MAX_DELAY);
	}

	#[test]
	fn jitter_stays_in_half_open_upper_half() {
		let delay = Duration::from_secs(100);
		let half = delay / 2;
		for entropy in [0, 1, u64::MAX / 2, u64::MAX] {
			let jittered = jittered_from(delay, entropy);
			assert!(jittered >= half, "{jittered:?} below d/2");
			assert!(jittered < delay, "{jittered:?} reached d");
		}
	}

	#[test]
	fn jitter_spans_the_full_upper_half() {
		// Entropy large enough to select the top of the range must actually reach it. A
		// too-narrow entropy source (e.g. subsecond nanos, always < 1s) would pin the
		// result to the bottom second of a 300 s delay instead.
		let delay = Duration::from_secs(300);
		let span = (delay / 2).as_nanos() as u64;
		let near_top = jittered_from(delay, span - 1);
		assert!(
			near_top >= delay - Duration::from_secs(1),
			"{near_top:?} did not reach the top of [d/2, d)"
		);
	}

	#[test]
	fn jitter_zero_delay_is_stable() {
		assert_eq!(jittered_from(Duration::ZERO, 12345), Duration::ZERO);
	}
}
