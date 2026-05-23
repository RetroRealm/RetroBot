use log::warn;
use playmatch_client::types::{LbGameImage, MetadataProvider};

use super::{ProviderGameInfo, truncate_summary};
use crate::abstraction::playmatch_client::PlaymatchClient;

fn is_cover(img: &LbGameImage) -> bool {
	let t = img.image_type.to_ascii_lowercase();
	t.contains("box - front") || t.contains("clear logo")
}

fn is_screenshot(img: &LbGameImage) -> bool {
	img.image_type.to_ascii_lowercase().contains("screenshot")
}

pub async fn fetch_game(client: &PlaymatchClient, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("LaunchBox game provider_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let game = match client.get_lb_game_by_id(id).await {
		Ok(g) => g,
		Err(e) => {
			warn!("LaunchBox game lookup failed for id {id}: {e}");
			return None;
		}
	};

	let (cover_url, screenshot_urls) = match client.get_lb_game_images(id).await {
		Ok(images) => {
			let cover = images
				.iter()
				.find(|i| is_cover(i))
				.or_else(|| images.first())
				.map(|i| i.file_name.clone());
			let screenshots: Vec<String> = images
				.iter()
				.filter(|i| is_screenshot(i))
				.take(3)
				.map(|i| i.file_name.clone())
				.collect();
			(cover, screenshots)
		}
		Err(e) => {
			warn!("LaunchBox image lookup failed for id {id}: {e}");
			(None, Vec::new())
		}
	};

	Some(ProviderGameInfo {
		provider: MetadataProvider::LaunchBox,
		name: game.name,
		page_url: game.wikipedia_url,
		summary: game.overview.map(|s| truncate_summary(&s)),
		first_release_date: None,
		cover_url,
		screenshot_urls,
	})
}
