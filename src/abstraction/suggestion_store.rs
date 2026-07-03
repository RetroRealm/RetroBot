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

/// A tracked suggestion card: its Discord message id plus the layout version it was
/// last rendered under. `layout_version` is `None` for legacy entries (bare message id,
/// written before versioning) and for seeded entries, both of which mean "layout unknown
/// ⇒ refresh once". See `SUGGESTION_CARD_LAYOUT_VERSION` for the bump contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackedCard {
	pub message_id: MessageId,
	pub layout_version: Option<u32>,
}

/// Serialize a store entry. `None` stays the bare id so a fresh unversioned write is
/// indistinguishable from a legacy one; `Some(v)` appends `:v`.
fn encode_entry(message_id: MessageId, layout_version: Option<u32>) -> String {
	match layout_version {
		Some(v) => format!("{}:{}", message_id.get(), v),
		None => message_id.get().to_string(),
	}
}

/// Parse a store value in either shape: bare `"123"` (legacy/unversioned) or `"123:7"`
/// (versioned). Anything else (empty, non-numeric, extra colons, empty halves) is
/// rejected as `None`, matching the existing skip-on-unparseable posture.
fn parse_entry(raw: &str) -> Option<TrackedCard> {
	match raw.split_once(':') {
		Some((id, ver)) => {
			let message_id = MessageId::new(id.parse().ok()?);
			let layout_version = ver.parse().ok()?;
			Some(TrackedCard {
				message_id,
				layout_version: Some(layout_version),
			})
		}
		None => Some(TrackedCard {
			message_id: MessageId::new(raw.parse().ok()?),
			layout_version: None,
		}),
	}
}

/// Persistent map of suggestion UUID → tracked card. Backed by Redis so the bot can
/// restart without re-posting cards or losing track of already-posted ones. Pure
/// persistence: the store never knows the current layout version, callers pass it.
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
		layout_version: Option<u32>,
	) -> Result<(), Error> {
		let mut c = self.conn.clone();
		c.set::<_, _, ()>(key_for(uuid), encode_entry(message_id, layout_version))
			.await
	}

	pub async fn get(&self, uuid: Uuid) -> Result<Option<TrackedCard>, Error> {
		let mut c = self.conn.clone();
		let raw: Option<String> = c.get(key_for(uuid)).await?;
		Ok(raw.as_deref().and_then(parse_entry))
	}

	pub async fn remove(&self, uuid: Uuid) -> Result<(), Error> {
		let mut c = self.conn.clone();
		c.del::<_, ()>(key_for(uuid)).await
	}

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
			let Some(tracked) = value.as_deref().and_then(parse_entry) else {
				continue;
			};
			result.insert(uuid, tracked);
		}
		Ok(result)
	}

	pub async fn is_empty(&self) -> Result<bool, Error> {
		Ok(self.list_all().await?.is_empty())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_entry_legacy_bare_id() {
		assert_eq!(
			parse_entry("123456"),
			Some(TrackedCard {
				message_id: MessageId::new(123456),
				layout_version: None,
			})
		);
	}

	#[test]
	fn parse_entry_versioned() {
		assert_eq!(
			parse_entry("123456:7"),
			Some(TrackedCard {
				message_id: MessageId::new(123456),
				layout_version: Some(7),
			})
		);
	}

	#[test]
	fn parse_entry_rejects_garbage() {
		for raw in ["", "abc", "12:xy", "12:", ":3", "1:2:3"] {
			assert_eq!(parse_entry(raw), None, "expected {raw:?} to be rejected");
		}
	}

	#[test]
	fn encode_parse_roundtrip() {
		let id = MessageId::new(987654321);
		for version in [None, Some(0), Some(1), Some(42)] {
			let encoded = encode_entry(id, version);
			assert_eq!(
				parse_entry(&encoded),
				Some(TrackedCard {
					message_id: id,
					layout_version: version,
				})
			);
		}
	}
}
