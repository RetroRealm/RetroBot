use log::warn;
use playmatch_client::types::MetadataProvider;

use super::{ProviderGameInfo, game_page_url, log_fetch_error};
use crate::abstraction::playmatch_client::PlaymatchClient;

pub async fn fetch_game(client: &PlaymatchClient, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("RetroAchievements game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_ra_game_by_id(id).await {
		Ok(g) => g,
		Err(e) => {
			log_fetch_error(MetadataProvider::RetroAchievements, "game", id, &e);
			return None;
		}
	};

	Some(ProviderGameInfo {
		provider: MetadataProvider::RetroAchievements,
		name: game.title,
		page_url: game_page_url(MetadataProvider::RetroAchievements, provider_id),
		summary: None,
		first_release_date: None,
		cover_url: game.image_icon,
		screenshot_urls: Vec::new(),
	})
}
