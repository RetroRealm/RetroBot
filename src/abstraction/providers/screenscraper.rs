use log::warn;
use playmatch_client::types::MetadataProvider;

use super::ProviderGameInfo;
use crate::abstraction::playmatch_client::PlaymatchClient;

pub async fn fetch_game(client: &PlaymatchClient, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("ScreenScraper game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_ss_game_by_id(id).await {
		Ok(g) => g,
		Err(e) => {
			warn!("ScreenScraper game lookup failed for id {id}: {e}");
			return None;
		}
	};

	let name = game
		.noms
		.iter()
		.find(|n| n.region.eq_ignore_ascii_case("en") || n.region.eq_ignore_ascii_case("us"))
		.or_else(|| game.noms.first())
		.map(|n| n.text.clone())?;

	Some(ProviderGameInfo {
		provider: MetadataProvider::ScreenScraper,
		name,
		page_url: None,
		summary: None,
		first_release_date: None,
		cover_url: None,
		screenshot_urls: Vec::new(),
	})
}
