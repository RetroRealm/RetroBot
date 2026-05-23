use chrono::{DateTime, Utc};
use playmatch_client::types::MetadataProvider;

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
		}
	}
}

/// Order in which the identification flow tries to enrich the card.
/// `EmuReady` and `TheGamesDB` are intentionally excluded. They have no read endpoints.
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
	}
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
		MetadataProvider::EmuReady | MetadataProvider::TheGamesDb => None,
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
