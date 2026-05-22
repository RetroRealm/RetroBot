use log::warn;
use playmatch_client::Client;
use playmatch_client::types::MetadataProvider;

use super::ProviderGameInfo;

pub async fn fetch_game(client: &Client, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("RetroAchievements game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_ra_game_by_id().id(id).send().await {
		Ok(resp) => resp.into_inner(),
		Err(e) => {
			warn!("RetroAchievements game lookup failed for id {id}: {e}");
			return None;
		}
	};

	let page_url = Some(format!("https://retroachievements.org/game/{id}"));

	Some(ProviderGameInfo {
		provider: MetadataProvider::RetroAchievements,
		name: game.title,
		page_url,
		summary: None,
		first_release_date: None,
		cover_url: game.image_icon,
		screenshot_urls: Vec::new(),
	})
}
