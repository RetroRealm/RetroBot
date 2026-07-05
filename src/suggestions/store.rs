//! Redis-backed record of which suggestions have a live card, and under which layout
//! version. Backed by Redis so the bot can restart without re-posting cards or losing
//! track of already-posted ones. Pure persistence: the store never decides the current
//! layout version, callers pass it.

use std::collections::HashMap;
use std::str::FromStr;

use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serenity::all::MessageId;
use uuid::Uuid;

pub type Error = redis::RedisError;

const KEY_PREFIX: &str = "playmatch:suggestion:";

fn key_for(uuid: Uuid) -> String {
	format!("{KEY_PREFIX}{uuid}")
}

/// A tracked suggestion card: its Discord message id and the layout version it was last
/// rendered under. The version is what lets a redeploy that bumped the layout find and
/// re-render only the stale cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackedCard {
	pub message_id: MessageId,
	pub version: u32,
}

/// Serialize an entry as `"<message_id>:<version>"`.
fn encode(card: TrackedCard) -> String {
	format!("{}:{}", card.message_id.get(), card.version)
}

/// Parse a `"<message_id>:<version>"` entry. Anything else (empty, non-numeric, missing
/// half, extra colons) is rejected as `None` so a corrupt value is skipped, not panicked on.
fn decode(raw: &str) -> Option<TrackedCard> {
	let (id, version) = raw.split_once(':')?;
	Some(TrackedCard {
		message_id: MessageId::new(id.parse().ok()?),
		version: version.parse().ok()?,
	})
}

pub struct SuggestionStore {
	conn: ConnectionManager,
}

impl SuggestionStore {
	pub async fn connect(url: &str) -> Result<Self, Error> {
		let client = redis::Client::open(url)?;
		let conn = ConnectionManager::new(client).await?;
		Ok(Self { conn })
	}

	pub async fn mark_posted(
		&self,
		uuid: Uuid,
		message_id: MessageId,
		version: u32,
	) -> Result<(), Error> {
		let mut c = self.conn.clone();
		c.set::<_, _, ()>(
			key_for(uuid),
			encode(TrackedCard {
				message_id,
				version,
			}),
		)
		.await
	}

	pub async fn get(&self, uuid: Uuid) -> Result<Option<TrackedCard>, Error> {
		let mut c = self.conn.clone();
		let raw: Option<String> = c.get(key_for(uuid)).await?;
		Ok(raw.as_deref().and_then(decode))
	}

	pub async fn remove(&self, uuid: Uuid) -> Result<(), Error> {
		let mut c = self.conn.clone();
		c.del::<_, ()>(key_for(uuid)).await
	}

	/// Every tracked card, keyed by suggestion UUID. Corrupt entries are skipped.
	pub async fn list_all(&self) -> Result<HashMap<Uuid, TrackedCard>, Error> {
		let mut conn = self.conn.clone();
		let pattern = format!("{KEY_PREFIX}*");

		let keys: Vec<String> = {
			let mut iter: redis::AsyncIter<'_, String> = conn.scan_match(&pattern).await?;
			let mut collected = Vec::new();
			while let Some(item) = iter.next_item().await {
				collected.push(item?);
			}
			collected
		};

		if keys.is_empty() {
			return Ok(HashMap::new());
		}

		let values: Vec<Option<String>> = conn.mget(&keys).await?;
		let mut result = HashMap::with_capacity(keys.len());
		for (key, value) in keys.into_iter().zip(values) {
			let Some(uuid_str) = key.strip_prefix(KEY_PREFIX) else {
				continue;
			};
			let Ok(uuid) = Uuid::from_str(uuid_str) else {
				continue;
			};
			let Some(card) = value.as_deref().and_then(decode) else {
				continue;
			};
			result.insert(uuid, card);
		}
		Ok(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encode_decode_roundtrip() {
		let card = TrackedCard {
			message_id: MessageId::new(987654321),
			version: 3,
		};
		assert_eq!(decode(&encode(card)), Some(card));
	}

	#[test]
	fn decode_rejects_garbage() {
		for raw in ["", "abc", "12", "12:xy", "12:", ":3", "1:2:3"] {
			assert_eq!(decode(raw), None, "expected {raw:?} to be rejected");
		}
	}
}
