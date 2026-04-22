use chrono::{DateTime, Utc};
use playmatch_client::Client;

pub struct IgdbGameInfo {
	pub name: String,
	pub page_url: String,
	pub summary: Option<String>,
	pub first_release_date: Option<DateTime<Utc>>,
	pub cover_url: Option<String>,
	pub screenshot_urls: Vec<String>,
}

pub struct IgdbCompanyInfo {
	pub name: String,
	pub page_url: Option<String>,
	pub logo_url: Option<String>,
	pub description: Option<String>,
}

pub struct IgdbPlatformInfo {
	pub name: String,
	pub page_url: String,
	pub logo_url: Option<String>,
	pub summary: Option<String>,
}

const SUMMARY_MAX: usize = 280;

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

fn truncate_summary(s: &str) -> String {
	if s.chars().count() <= SUMMARY_MAX {
		s.to_string()
	} else {
		let truncated: String = s.chars().take(SUMMARY_MAX - 1).collect();
		format!("{truncated}…")
	}
}

fn timestamp_to_datetime(ts: i64) -> Option<DateTime<Utc>> {
	DateTime::<Utc>::from_timestamp(ts, 0)
}

pub async fn fetch_game(client: &Client, provider_id: &str) -> Option<IgdbGameInfo> {
	let id: i32 = provider_id.parse().ok()?;
	let game = client
		.get_igdb_game_by_id()
		.id(id)
		.send()
		.await
		.ok()?
		.into_inner();

	let cover_url = match game.cover {
		Some(cover_id) => client
			.get_igdb_cover_by_id()
			.id(cover_id)
			.send()
			.await
			.ok()
			.map(|resp| normalize_image_url(&resp.into_inner().url, "t_cover_big")),
		None => None,
	};

	let mut screenshot_urls = Vec::new();
	if let Some(ids) = game.screenshots.as_ref() {
		for screenshot_id in ids.iter().take(3) {
			if let Ok(resp) = client
				.get_igdb_screenshot_by_id()
				.id(*screenshot_id)
				.send()
				.await
			{
				screenshot_urls.push(normalize_image_url(&resp.into_inner().url, "t_1080p"));
			}
		}
	}

	Some(IgdbGameInfo {
		name: game.name,
		page_url: game.url,
		summary: game.summary.map(|s| truncate_summary(&s)),
		first_release_date: game.first_release_date.and_then(timestamp_to_datetime),
		cover_url,
		screenshot_urls,
	})
}

pub async fn fetch_company(client: &Client, provider_id: &str) -> Option<IgdbCompanyInfo> {
	let id: i32 = provider_id.parse().ok()?;
	let company = client
		.get_igdb_company_by_id()
		.id(id)
		.send()
		.await
		.ok()?
		.into_inner();

	let logo_url = match company.logo {
		Some(logo_id) => client
			.get_igdb_company_logo_by_id()
			.id(logo_id)
			.send()
			.await
			.ok()
			.map(|resp| normalize_image_url(&resp.into_inner().url, "t_logo_med")),
		None => None,
	};

	Some(IgdbCompanyInfo {
		name: company.name,
		page_url: company.url,
		logo_url,
		description: company.description.map(|s| truncate_summary(&s)),
	})
}

pub async fn fetch_platform(client: &Client, provider_id: &str) -> Option<IgdbPlatformInfo> {
	let id: i32 = provider_id.parse().ok()?;
	let platform = client
		.get_igdb_platform_by_id()
		.id(id)
		.send()
		.await
		.ok()?
		.into_inner();

	let logo_url = match platform.platform_logo {
		Some(logo_id) => client
			.get_igdb_platform_logo_by_id()
			.id(logo_id)
			.send()
			.await
			.ok()
			.map(|resp| normalize_image_url(&resp.into_inner().url, "t_logo_med")),
		None => None,
	};

	Some(IgdbPlatformInfo {
		name: platform.name,
		page_url: platform.url,
		logo_url,
		summary: platform.summary.map(|s| truncate_summary(&s)),
	})
}
