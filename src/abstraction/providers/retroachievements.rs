use log::warn;
use playmatch_client::types::MetadataProvider;

use super::{ProviderGameInfo, game_page_url, log_fetch_error};
use crate::abstraction::playmatch_client::PlaymatchClient;

/// RetroAchievements `image_icon` is a root-relative path (e.g. `/Images/080107.png`)
/// served from this host. The API never returns an absolute URL, so we join the
/// host here.
const IMAGE_CDN_BASE: &str = "https://media.retroachievements.org";

fn image_url(path: &str) -> String {
	// The path is root-relative; join without doubling the separator regardless
	// of whether the API includes the leading slash.
	format!("{IMAGE_CDN_BASE}/{}", path.trim_start_matches('/'))
}

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
		cover_url: game.image_icon.as_deref().map(image_url),
		screenshot_urls: Vec::new(),
	})
}

#[cfg(test)]
mod tests {
	use super::image_url;

	#[test]
	fn root_relative_path_joins_without_double_slash() {
		// The live shape: a leading-slash root-relative path.
		assert_eq!(
			image_url("/Images/080107.png"),
			"https://media.retroachievements.org/Images/080107.png",
		);
	}

	#[test]
	fn path_without_leading_slash_still_joins_once() {
		assert_eq!(
			image_url("Images/080107.png"),
			"https://media.retroachievements.org/Images/080107.png",
		);
	}
}
