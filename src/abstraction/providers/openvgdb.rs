use log::warn;
use playmatch_client::types::MetadataProvider;

use super::{ProviderGameInfo, truncate_summary};
use crate::abstraction::playmatch_client::PlaymatchClient;

pub async fn fetch_game(client: &PlaymatchClient, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i64 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("OpenVGDB release_id '{provider_id}' is not a valid i64: {e}");
			return None;
		}
	};

	let release = match client.get_ovgdb_release_by_id(id).await {
		Ok(r) => r,
		Err(e) => {
			warn!("OpenVGDB release lookup failed for id {id}: {e}");
			return None;
		}
	};

	Some(ProviderGameInfo {
		provider: MetadataProvider::OpenVgdb,
		name: release.title_name,
		page_url: release.reference_url,
		summary: release.description.map(|s| truncate_summary(&s)),
		first_release_date: None,
		cover_url: release.cover_front,
		screenshot_urls: Vec::new(),
	})
}
