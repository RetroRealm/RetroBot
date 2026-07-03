use log::warn;
use playmatch_client::types::{LbGameImage, MetadataProvider};

use super::{ProviderGameInfo, game_page_url, log_fetch_error, truncate_summary};
use crate::abstraction::playmatch_client::PlaymatchClient;

/// LaunchBox image `file_name`s are bare, root-relative paths under this CDN
/// host (e.g. `24fcea1e-...png`). The API never returns an absolute URL, so we
/// join the host here; the `/games/` subpath 404s, the bare host is correct.
const IMAGE_CDN_BASE: &str = "https://images.launchbox-app.com/";

fn image_url(file_name: &str) -> String {
	format!("{IMAGE_CDN_BASE}{file_name}")
}

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
			log_fetch_error(MetadataProvider::LaunchBox, "game", id, &e);
			return None;
		}
	};

	let (cover_url, screenshot_urls) = match client.get_lb_game_images(id).await {
		Ok(images) => {
			let cover = images
				.iter()
				.find(|i| is_cover(i))
				.or_else(|| images.first())
				.map(|i| image_url(&i.file_name));
			let screenshots: Vec<String> = images
				.iter()
				.filter(|i| is_screenshot(i))
				.take(3)
				.map(|i| image_url(&i.file_name))
				.collect();
			(cover, screenshots)
		}
		Err(e) => {
			log_fetch_error(MetadataProvider::LaunchBox, "images", id, &e);
			(None, Vec::new())
		}
	};

	Some(ProviderGameInfo {
		provider: MetadataProvider::LaunchBox,
		name: game.name,
		page_url: game_page_url(MetadataProvider::LaunchBox, provider_id),
		summary: game.overview.map(|s| truncate_summary(&s)),
		first_release_date: None,
		cover_url,
		screenshot_urls,
	})
}

#[cfg(test)]
mod tests {
	use super::image_url;

	#[test]
	fn bare_file_name_becomes_absolute_cdn_url() {
		// The API returns a bare UUID filename; it must reach Discord as an
		// absolute URL under the images host, not a bare filename.
		assert_eq!(
			image_url("24fcea1e-451e-4f72-9092-a22a2455f4ad.png"),
			"https://images.launchbox-app.com/24fcea1e-451e-4f72-9092-a22a2455f4ad.png",
		);
	}

	#[test]
	fn subpath_file_name_is_joined_under_host() {
		assert_eq!(
			image_url("Nintendo/box.jpg"),
			"https://images.launchbox-app.com/Nintendo/box.jpg",
		);
	}
}
