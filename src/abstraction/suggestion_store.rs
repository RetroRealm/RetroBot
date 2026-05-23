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

/// Persistent map of suggestion UUID → Discord message id. Backed by Redis so the bot
/// can restart without re-posting cards or losing track of already-posted ones.
pub struct SuggestionStore {
	conn: ConnectionManager,
}

impl SuggestionStore {
	pub async fn connect(url: &str) -> Result<Self, Error> {
		let client = redis::Client::open(url)?;
		let conn = ConnectionManager::new(client).await?;
		Ok(Self { conn })
	}

	pub async fn mark_posted(&self, uuid: Uuid, message_id: MessageId) -> Result<(), Error> {
		let mut c = self.conn.clone();
		c.set::<_, _, ()>(key_for(uuid), message_id.get()).await
	}

	pub async fn get(&self, uuid: Uuid) -> Result<Option<MessageId>, Error> {
		let mut c = self.conn.clone();
		let raw: Option<u64> = c.get(key_for(uuid)).await?;
		Ok(raw.map(MessageId::new))
	}

	pub async fn remove(&self, uuid: Uuid) -> Result<(), Error> {
		let mut c = self.conn.clone();
		c.del::<_, ()>(key_for(uuid)).await
	}

	pub async fn list_all(&self) -> Result<HashMap<Uuid, MessageId>, Error> {
		let mut conn = self.conn.clone();
		let pattern = format!("{KEY_PREFIX}*");
		let mut cursor = 0u64;
		let mut keys: Vec<String> = Vec::new();
		loop {
			let (next_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
				.arg(cursor)
				.arg("MATCH")
				.arg(&pattern)
				.arg("COUNT")
				.arg(100)
				.query_async(&mut conn)
				.await?;
			keys.extend(batch);
			if next_cursor == 0 {
				break;
			}
			cursor = next_cursor;
		}

		if keys.is_empty() {
			return Ok(HashMap::new());
		}

		let values: Vec<Option<u64>> = conn.mget(&keys).await?;
		let mut result = HashMap::with_capacity(keys.len());
		for (key, value) in keys.into_iter().zip(values) {
			let Some(uuid_str) = key.strip_prefix(KEY_PREFIX) else {
				continue;
			};
			let Ok(uuid) = Uuid::from_str(uuid_str) else {
				continue;
			};
			let Some(message_id) = value else {
				continue;
			};
			result.insert(uuid, MessageId::new(message_id));
		}
		Ok(result)
	}

	pub async fn is_empty(&self) -> Result<bool, Error> {
		Ok(self.list_all().await?.is_empty())
	}
}
