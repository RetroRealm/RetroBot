use log::warn;
use playmatch_client::Client;
use playmatch_client::types::MetadataProvider;

use super::ProviderGameInfo;

pub async fn fetch_game(client: &Client, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("SteamGridDB game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_sgdb_game_by_id().id(id).send().await {
		Ok(resp) => resp.into_inner(),
		Err(e) => {
			warn!("SteamGridDB game lookup failed for id {id}: {e}");
			return None;
		}
	};

	Some(ProviderGameInfo {
		provider: MetadataProvider::SteamGridDb,
		name: game.name,
		page_url: Some(format!("https://www.steamgriddb.com/game/{id}")),
		summary: None,
		first_release_date: None,
		cover_url: None,
		screenshot_urls: Vec::new(),
	})
}
