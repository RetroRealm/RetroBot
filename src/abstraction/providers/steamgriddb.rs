use log::warn;
use playmatch_client::types::MetadataProvider;

use super::{ProviderGameInfo, game_page_url};
use crate::abstraction::playmatch_client::PlaymatchClient;

pub async fn fetch_game(client: &PlaymatchClient, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("SteamGridDB game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_sgdb_game_by_id(id).await {
		Ok(g) => g,
		Err(e) => {
			warn!("SteamGridDB game lookup failed for id {id}: {e}");
			return None;
		}
	};

	Some(ProviderGameInfo {
		provider: MetadataProvider::SteamGridDb,
		name: game.name,
		page_url: game_page_url(MetadataProvider::SteamGridDb, provider_id),
		summary: None,
		first_release_date: None,
		cover_url: None,
		screenshot_urls: Vec::new(),
	})
}
