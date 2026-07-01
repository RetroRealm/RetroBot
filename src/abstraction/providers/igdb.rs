use chrono::{DateTime, Utc};
use log::{debug, warn};
use playmatch_client::types::MetadataProvider;

use super::{ProviderCompanyInfo, ProviderGameInfo, ProviderPlatformInfo, truncate_summary};
use crate::abstraction::playmatch_client::PlaymatchClient;

fn normalize_image_url(raw: &str, size: &str) -> String {
	let https = if let Some(rest) = raw.strip_prefix("//") {
		format!("https://{rest}")
	} else if raw.starts_with("http") {
		raw.to_string()
	} else {
		format!("https://{raw}")
	};
	https.replacen("t_thumb", size, 1)
}

fn timestamp_to_datetime(ts: i64) -> Option<DateTime<Utc>> {
	DateTime::<Utc>::from_timestamp(ts, 0)
}

pub async fn fetch_game(client: &PlaymatchClient, provider_id: &str) -> Option<ProviderGameInfo> {
	let id: i32 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("IGDB game provider_id '{provider_id}' is not a valid i32: {e}");
			return None;
		}
	};

	let game = match client.get_igdb_game_by_id(id).await {
		Ok(g) => g,
		Err(e) => {
			warn!("IGDB game lookup failed for id {id}: {e}");
			return None;
		}
	};

	debug!(
		"IGDB game {id}: cover={:?}, screenshots={:?}",
		game.cover,
		game.screenshots.as_ref().map(|v| v.len())
	);

	let cover_url = match game.cover {
		Some(cover_id) => match client.get_igdb_cover_by_id(cover_id).await {
			Ok(cover) => {
				let url = normalize_image_url(&cover.url, "t_cover_big");
				debug!("IGDB cover {cover_id} resolved to {url}");
				Some(url)
			}
			Err(e) => {
				warn!("IGDB cover lookup failed for id {cover_id}: {e}");
				None
			}
		},
		None => None,
	};

	let mut screenshot_urls = Vec::new();
	if let Some(ids) = game.screenshots.as_ref() {
		let wanted: Vec<i32> = ids.iter().copied().take(3).collect();
		if !wanted.is_empty() {
			match client.get_igdb_screenshots_by_ids(wanted).await {
				Ok(shots) => {
					screenshot_urls = shots
						.iter()
						.map(|s| normalize_image_url(&s.url, "t_1080p"))
						.collect();
				}
				Err(e) => warn!("IGDB screenshots lookup failed for game {id}: {e}"),
			}
		}
	}

	Some(ProviderGameInfo {
		provider: MetadataProvider::Igdb,
		name: game.name,
		page_url: Some(game.url),
		summary: game.summary.map(|s| truncate_summary(&s)),
		first_release_date: game.first_release_date.and_then(timestamp_to_datetime),
		cover_url,
		screenshot_urls,
	})
}

pub async fn fetch_company(
	client: &PlaymatchClient,
	provider_id: &str,
) -> Option<ProviderCompanyInfo> {
	let id: i32 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("IGDB company provider_id '{provider_id}' is not a valid i32: {e}");
			return None;
		}
	};

	let company = match client.get_igdb_company_by_id(id).await {
		Ok(c) => c,
		Err(e) => {
			warn!("IGDB company lookup failed for id {id}: {e}");
			return None;
		}
	};

	let logo_url = match company.logo {
		Some(logo_id) => match client.get_igdb_company_logo_by_id(logo_id).await {
			Ok(logo) => {
				let url = normalize_image_url(&logo.url, "t_logo_med");
				debug!("IGDB company logo {logo_id} resolved to {url}");
				Some(url)
			}
			Err(e) => {
				warn!("IGDB company logo lookup failed for id {logo_id}: {e}");
				None
			}
		},
		None => None,
	};

	Some(ProviderCompanyInfo {
		provider: MetadataProvider::Igdb,
		name: company.name,
		page_url: company.url,
		logo_url,
		description: company.description.map(|s| truncate_summary(&s)),
	})
}

pub async fn fetch_platform(
	client: &PlaymatchClient,
	provider_id: &str,
) -> Option<ProviderPlatformInfo> {
	let id: i32 = match provider_id.parse() {
		Ok(id) => id,
		Err(e) => {
			warn!("IGDB platform provider_id '{provider_id}' is not a valid i32: {e}");
			return None;
		}
	};

	let platform = match client.get_igdb_platform_by_id(id).await {
		Ok(p) => p,
		Err(e) => {
			warn!("IGDB platform lookup failed for id {id}: {e}");
			return None;
		}
	};

	let logo_url = match platform.platform_logo {
		Some(logo_id) => match client.get_igdb_platform_logo_by_id(logo_id).await {
			Ok(logo) => {
				let url = normalize_image_url(&logo.url, "t_logo_med");
				debug!("IGDB platform logo {logo_id} resolved to {url}");
				Some(url)
			}
			Err(e) => {
				warn!("IGDB platform logo lookup failed for id {logo_id}: {e}");
				None
			}
		},
		None => None,
	};

	Some(ProviderPlatformInfo {
		provider: MetadataProvider::Igdb,
		name: platform.name,
		page_url: Some(platform.url),
		logo_url,
		summary: platform.summary.map(|s| truncate_summary(&s)),
	})
}
