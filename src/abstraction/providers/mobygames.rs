use log::warn;
use playmatch_client::Client;
use playmatch_client::types::MetadataProvider;

use super::{ProviderGameInfo, truncate_summary};

pub async fn fetch_game(client: &Client, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("MobyGames game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_mg_game_by_id().id(id).send().await {
		Ok(resp) => resp.into_inner(),
		Err(e) => {
			warn!("MobyGames game lookup failed for id {id}: {e}");
			return None;
		}
	};

	let cover_url = game.sample_cover.as_ref().map(|c| c.image.clone());
	let screenshot_urls = game
		.sample_screenshots
		.as_ref()
		.map(|ss| ss.iter().take(3).map(|s| s.image.clone()).collect())
		.unwrap_or_default();

	Some(ProviderGameInfo {
		provider: MetadataProvider::MobyGames,
		name: game.title,
		page_url: game.moby_url,
		summary: game.description.map(|s| truncate_summary(&s)),
		first_release_date: None,
		cover_url,
		screenshot_urls,
	})
}
