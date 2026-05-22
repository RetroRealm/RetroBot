use log::warn;
use playmatch_client::Client;
use playmatch_client::types::MetadataProvider;

use super::ProviderGameInfo;

pub async fn fetch_game(client: &Client, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("ScreenScraper game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_ss_game_by_id().id(id).send().await {
		Ok(resp) => resp.into_inner(),
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
