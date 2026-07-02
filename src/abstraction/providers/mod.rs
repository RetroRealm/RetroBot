use chrono::{DateTime, Utc};
use log::{debug, warn};
use playmatch_client::Error;
use playmatch_client::types::MetadataProvider;
use reqwest::StatusCode;

use crate::abstraction::playmatch::describe;
use crate::abstraction::playmatch_client::PlaymatchClient;

pub mod igdb;
pub mod launchbox;
pub mod mobygames;
pub mod openvgdb;
pub mod retroachievements;
pub mod screenscraper;
pub mod steamgriddb;

pub struct ProviderGameInfo {
	pub provider: MetadataProvider,
	pub name: String,
	pub page_url: Option<String>,
	pub summary: Option<String>,
	pub first_release_date: Option<DateTime<Utc>>,
	pub cover_url: Option<String>,
	pub screenshot_urls: Vec<String>,
}

pub struct ProviderCompanyInfo {
	pub provider: MetadataProvider,
	pub name: String,
	pub page_url: Option<String>,
	pub logo_url: Option<String>,
	pub description: Option<String>,
}

pub struct ProviderPlatformInfo {
	pub provider: MetadataProvider,
	pub name: String,
	pub page_url: Option<String>,
	pub logo_url: Option<String>,
	pub summary: Option<String>,
}

/// Order here is the order shown in Discord's dropdown.
#[derive(poise::ChoiceParameter, Clone, Copy, Debug)]
pub enum ProviderChoice {
	#[name = "IGDB"]
	Igdb,
	#[name = "MobyGames"]
	MobyGames,
	#[name = "LaunchBox"]
	LaunchBox,
	#[name = "ScreenScraper"]
	ScreenScraper,
	#[name = "RetroAchievements"]
	RetroAchievements,
	#[name = "OpenVGDB"]
	OpenVgdb,
	#[name = "SteamGridDB"]
	SteamGridDb,
	#[name = "EmuReady"]
	EmuReady,
	#[name = "TheGamesDB"]
	TheGamesDb,
	#[name = "Hasheous"]
	Hasheous,
}

impl ProviderChoice {
	pub fn to_metadata_provider(self) -> MetadataProvider {
		match self {
			Self::Igdb => MetadataProvider::Igdb,
			Self::MobyGames => MetadataProvider::MobyGames,
			Self::LaunchBox => MetadataProvider::LaunchBox,
			Self::ScreenScraper => MetadataProvider::ScreenScraper,
			Self::RetroAchievements => MetadataProvider::RetroAchievements,
			Self::OpenVgdb => MetadataProvider::OpenVgdb,
			Self::SteamGridDb => MetadataProvider::SteamGridDb,
			Self::EmuReady => MetadataProvider::EmuReady,
			Self::TheGamesDb => MetadataProvider::TheGamesDb,
			Self::Hasheous => MetadataProvider::Hasheous,
		}
	}
}

/// Order in which the identification flow tries to enrich the card.
/// `EmuReady`, `TheGamesDB` and `Hasheous` are intentionally excluded. They have no read endpoints.
pub const ENRICHMENT_PRIORITY: &[MetadataProvider] = &[
	MetadataProvider::Igdb,
	MetadataProvider::MobyGames,
	MetadataProvider::LaunchBox,
	MetadataProvider::ScreenScraper,
	MetadataProvider::RetroAchievements,
	MetadataProvider::OpenVgdb,
	MetadataProvider::SteamGridDb,
];

/// Order here determines the column order in the `/list` tables.
pub const ALL_PROVIDERS: &[MetadataProvider] = &[
	MetadataProvider::Igdb,
	MetadataProvider::MobyGames,
	MetadataProvider::LaunchBox,
	MetadataProvider::ScreenScraper,
	MetadataProvider::RetroAchievements,
	MetadataProvider::OpenVgdb,
	MetadataProvider::SteamGridDb,
	MetadataProvider::EmuReady,
	MetadataProvider::TheGamesDb,
	MetadataProvider::Hasheous,
];

pub fn display_name(provider: MetadataProvider) -> &'static str {
	match provider {
		MetadataProvider::Igdb => "IGDB",
		MetadataProvider::SteamGridDb => "SteamGridDB",
		MetadataProvider::ScreenScraper => "ScreenScraper",
		MetadataProvider::MobyGames => "MobyGames",
		MetadataProvider::LaunchBox => "LaunchBox",
		MetadataProvider::EmuReady => "EmuReady",
		MetadataProvider::OpenVgdb => "OpenVGDB",
		MetadataProvider::RetroAchievements => "RetroAchievements",
		MetadataProvider::TheGamesDb => "TheGamesDB",
		MetadataProvider::Hasheous => "Hasheous",
	}
}

pub fn short_label(provider: MetadataProvider) -> &'static str {
	match provider {
		MetadataProvider::Igdb => "IGDB",
		MetadataProvider::SteamGridDb => "SGDB",
		MetadataProvider::ScreenScraper => "SS",
		MetadataProvider::MobyGames => "MG",
		MetadataProvider::LaunchBox => "LB",
		MetadataProvider::EmuReady => "ER",
		MetadataProvider::OpenVgdb => "OVGDB",
		MetadataProvider::RetroAchievements => "RA",
		MetadataProvider::TheGamesDb => "TGDB",
		MetadataProvider::Hasheous => "HSH",
	}
}

/// The provider's own website page for a game, built from the provider id.
///
/// Only providers with a verified id-addressable page template appear here.
/// IGDB pages are slug-based, so `igdb::fetch_game` uses the API-supplied
/// `Game.url` instead. OpenVGDB is a distributed SQLite database with no
/// website. EmuReady, TheGamesDB and Hasheous have no read endpoints and no
/// verified page scheme.
pub fn game_page_url(provider: MetadataProvider, provider_id: &str) -> Option<String> {
	// Every template provider keys pages by a numeric id. Rejecting anything
	// non-numeric means we never interpolate an arbitrary string into a URL.
	let id: i64 = provider_id.trim().parse().ok()?;
	let url = match provider {
		MetadataProvider::LaunchBox => {
			format!("https://gamesdb.launchbox-app.com/games/details/{id}")
		}
		MetadataProvider::ScreenScraper => {
			format!("https://www.screenscraper.fr/gameinfos.php?gameid={id}")
		}
		MetadataProvider::RetroAchievements => {
			format!("https://retroachievements.org/game/{id}")
		}
		MetadataProvider::SteamGridDb => format!("https://www.steamgriddb.com/game/{id}"),
		MetadataProvider::MobyGames => format!("https://www.mobygames.com/game/{id}/"),
		_ => return None,
	};
	Some(url)
}

const SUMMARY_MAX: usize = 280;

pub(super) fn truncate_summary(s: &str) -> String {
	if s.chars().count() <= SUMMARY_MAX {
		s.to_string()
	} else {
		let truncated: String = s.chars().take(SUMMARY_MAX - 1).collect();
		format!("{truncated}…")
	}
}

/// A fetch against playmatch's provider mirror returned 404: the entry is absent
/// from the mirror. This is expected (an id can exist on the provider's own site
/// yet not be mirrored) and must not warn or dump headers.
fn is_absent<E>(e: &Error<E>) -> bool {
	e.status() == Some(StatusCode::NOT_FOUND)
}

/// Log a provider fetch error at the right level.
///
/// A 404 means the entry is not in playmatch's mirror, an expected absence, so it
/// drops to `debug!`. Anything else is a real failure and warns with a concise
/// status/kind rendering, never the header dump.
pub(super) fn log_fetch_error<E: std::fmt::Debug>(
	provider: MetadataProvider,
	entity: &str,
	id: impl std::fmt::Display,
	e: &Error<E>,
) {
	let provider = display_name(provider);
	if is_absent(e) {
		debug!("no {provider} {entity} for id {id} in playmatch");
	} else {
		warn!(
			"{provider} {entity} lookup failed for id {id}: {}",
			describe(e)
		);
	}
}

pub async fn fetch_game(
	client: &PlaymatchClient,
	provider: MetadataProvider,
	provider_id: &str,
) -> Option<ProviderGameInfo> {
	match provider {
		MetadataProvider::Igdb => igdb::fetch_game(client, provider_id).await,
		MetadataProvider::MobyGames => mobygames::fetch_game(client, provider_id).await,
		MetadataProvider::LaunchBox => launchbox::fetch_game(client, provider_id).await,
		MetadataProvider::ScreenScraper => screenscraper::fetch_game(client, provider_id).await,
		MetadataProvider::RetroAchievements => {
			retroachievements::fetch_game(client, provider_id).await
		}
		MetadataProvider::OpenVgdb => openvgdb::fetch_game(client, provider_id).await,
		MetadataProvider::SteamGridDb => steamgriddb::fetch_game(client, provider_id).await,
		MetadataProvider::EmuReady | MetadataProvider::TheGamesDb | MetadataProvider::Hasheous => {
			None
		}
	}
}

pub async fn fetch_company(
	client: &PlaymatchClient,
	provider: MetadataProvider,
	provider_id: &str,
) -> Option<ProviderCompanyInfo> {
	match provider {
		MetadataProvider::Igdb => igdb::fetch_company(client, provider_id).await,
		_ => None,
	}
}

pub async fn fetch_platform(
	client: &PlaymatchClient,
	provider: MetadataProvider,
	provider_id: &str,
) -> Option<ProviderPlatformInfo> {
	match provider {
		MetadataProvider::Igdb => igdb::fetch_platform(client, provider_id).await,
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::{game_page_url, is_absent};
	use playmatch_client::types::MetadataProvider;
	use playmatch_client::{Error, ResponseValue};
	use reqwest::StatusCode;
	use reqwest::header::HeaderMap;

	fn error_response(status: StatusCode) -> Error<()> {
		Error::ErrorResponse(ResponseValue::new((), status, HeaderMap::new()))
	}

	#[test]
	fn is_absent_only_matches_404() {
		assert!(is_absent(&error_response(StatusCode::NOT_FOUND)));
		assert!(!is_absent(&error_response(
			StatusCode::INTERNAL_SERVER_ERROR
		)));
		assert!(!is_absent(&Error::<()>::InvalidRequest("bad".to_string())));
	}

	#[test]
	fn game_page_url_templates() {
		// LaunchBox must use /games/details/{database_id}. /games/dbid/{id} uses a
		// different internal numbering and redirects to the wrong game.
		assert_eq!(
			game_page_url(MetadataProvider::LaunchBox, "3735").as_deref(),
			Some("https://gamesdb.launchbox-app.com/games/details/3735"),
		);
		assert_eq!(
			game_page_url(MetadataProvider::ScreenScraper, "3").as_deref(),
			Some("https://www.screenscraper.fr/gameinfos.php?gameid=3"),
		);
		assert_eq!(
			game_page_url(MetadataProvider::RetroAchievements, "1").as_deref(),
			Some("https://retroachievements.org/game/1"),
		);
		assert_eq!(
			game_page_url(MetadataProvider::SteamGridDb, "5254").as_deref(),
			Some("https://www.steamgriddb.com/game/5254"),
		);
		assert_eq!(
			game_page_url(MetadataProvider::MobyGames, "616").as_deref(),
			Some("https://www.mobygames.com/game/616/"),
		);
	}

	#[test]
	fn game_page_url_providers_without_template() {
		for provider in [
			MetadataProvider::OpenVgdb,
			MetadataProvider::EmuReady,
			MetadataProvider::TheGamesDb,
			MetadataProvider::Hasheous,
			MetadataProvider::Igdb,
		] {
			assert_eq!(game_page_url(provider, "1"), None);
		}
	}

	#[test]
	fn game_page_url_rejects_non_numeric_id() {
		assert_eq!(game_page_url(MetadataProvider::LaunchBox, "abc"), None);
		assert_eq!(game_page_url(MetadataProvider::LaunchBox, ""), None);
		// Whitespace-padded numeric ids are accepted.
		assert_eq!(
			game_page_url(MetadataProvider::LaunchBox, " 3735 ").as_deref(),
			Some("https://gamesdb.launchbox-app.com/games/details/3735"),
		);
	}
}
