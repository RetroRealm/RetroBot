use std::num::NonZeroU32;
use std::time::Duration;

use governor::Quota;
use governor::RateLimiter;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use playmatch_client::types::{
	Company, CompanyLogo, CompanyMetadataResponse, CompanyOrPlatformMatchRequest,
	CompanyOrPlatformSuggestionRequest, Cover, CreateOrGetUserRequest, Game,
	GameAndRelationMatchResult, GameAndRelationsResult, GameMatchRequest, GameSuggestionRequest,
	LbGame, LbGameImage, MgGame, OvgdbRelease, Platform, PlatformLogo, PlatformMetadataResponse,
	RaGame, Screenshot, SgdbGame, SsGame, Suggestion, UpdateUserPermissionsRequest,
	UpdatedMatchResult, UpdatedMetadataMatchesFromSuggestionResponse, User,
};
use playmatch_client::{Client as Inner, Error, ResponseValue};
use reqwest::StatusCode;
use uuid::Uuid;

type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Thin newtype wrapper that gates every outbound playmatch request through a shared
/// token bucket sized to playmatch's own server-side limit (4 req/s replenish, 20 burst).
/// One 429 retry as a backstop in case the bot's clock drifts vs the server's.
pub struct PlaymatchClient {
	inner: Inner,
	limiter: Limiter,
}

impl PlaymatchClient {
	pub fn new(inner: Inner) -> Self {
		let quota = Quota::per_second(NonZeroU32::new(4).unwrap())
			.allow_burst(NonZeroU32::new(20).unwrap());
		Self {
			inner,
			limiter: RateLimiter::direct(quota),
		}
	}

	async fn run<T, Fut, F>(&self, make: F) -> Result<T, Error<()>>
	where
		F: Fn() -> Fut,
		Fut: std::future::Future<Output = Result<ResponseValue<T>, Error<()>>>,
	{
		self.limiter.until_ready().await;
		match make().await {
			Err(Error::UnexpectedResponse(resp))
				if resp.status() == StatusCode::TOO_MANY_REQUESTS =>
			{
				let wait = parse_retry_after(&resp).unwrap_or(Duration::from_secs(1));
				tokio::time::sleep(wait.max(Duration::from_secs(1))).await;
				make().await.map(ResponseValue::into_inner)
			}
			result => result.map(ResponseValue::into_inner),
		}
	}

	pub async fn get_all_companies(&self) -> Result<Vec<CompanyMetadataResponse>, Error<()>> {
		self.run(|| self.inner.get_all_companies().send()).await
	}

	pub async fn get_all_platforms(&self) -> Result<Vec<PlatformMetadataResponse>, Error<()>> {
		self.run(|| self.inner.get_all_platforms().send()).await
	}

	pub async fn get_company_by_id(&self, id: Uuid) -> Result<CompanyMetadataResponse, Error<()>> {
		self.run(|| self.inner.get_company_by_id().id(id).send())
			.await
	}

	pub async fn get_platform_by_id(
		&self,
		id: Uuid,
	) -> Result<PlatformMetadataResponse, Error<()>> {
		self.run(|| self.inner.get_platform_by_id().id(id).send())
			.await
	}

	pub async fn get_playmatch_game_with_relations_by_id(
		&self,
		id: Uuid,
	) -> Result<GameAndRelationsResult, Error<()>> {
		self.run(|| {
			self.inner
				.get_playmatch_game_with_relations_by_id()
				.id(id)
				.send()
		})
		.await
	}

	pub async fn identify_game_and_relations(
		&self,
		file_name: String,
		file_size: i64,
		md5: Option<String>,
		sha1: Option<String>,
		sha256: Option<String>,
	) -> Result<GameAndRelationMatchResult, Error<()>> {
		self.run(|| {
			let mut b = self
				.inner
				.identify_game_and_relations()
				.file_name(file_name.clone())
				.file_size(file_size);
			if let Some(v) = md5.clone() {
				b = b.md5(v);
			}
			if let Some(v) = sha1.clone() {
				b = b.sha1(v);
			}
			if let Some(v) = sha256.clone() {
				b = b.sha256(v);
			}
			b.send()
		})
		.await
	}

	pub async fn get_all_suggestions(&self) -> Result<Vec<Suggestion>, Error<()>> {
		self.run(|| self.inner.get_all_suggestions().send()).await
	}

	pub async fn get_suggestion_by_id(&self, id: Uuid) -> Result<Suggestion, Error<()>> {
		self.run(|| self.inner.get_suggestion_by_id().id(id).send())
			.await
	}

	pub async fn approve_suggestion(
		&self,
		id: Uuid,
	) -> Result<UpdatedMetadataMatchesFromSuggestionResponse, Error<()>> {
		self.run(|| self.inner.approve_suggestion().id(id).send())
			.await
	}

	pub async fn delete_suggestion(&self, id: Uuid) -> Result<(), Error<()>> {
		self.run(|| self.inner.delete_suggestion().id(id).send())
			.await
	}

	pub async fn create_game_suggestion(
		&self,
		body: GameSuggestionRequest,
	) -> Result<Suggestion, Error<()>> {
		self.run(|| {
			self.inner
				.create_game_suggestion()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn create_company_suggestion(
		&self,
		body: CompanyOrPlatformSuggestionRequest,
	) -> Result<Suggestion, Error<()>> {
		self.run(|| {
			self.inner
				.create_company_suggestion()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn create_platform_suggestion(
		&self,
		body: CompanyOrPlatformSuggestionRequest,
	) -> Result<Suggestion, Error<()>> {
		self.run(|| {
			self.inner
				.create_platform_suggestion()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn manually_match_game(
		&self,
		body: GameMatchRequest,
	) -> Result<Vec<UpdatedMatchResult>, Error<()>> {
		self.run(|| self.inner.manually_match_game().body(body.clone()).send())
			.await
	}

	pub async fn manually_match_company(
		&self,
		body: CompanyOrPlatformMatchRequest,
	) -> Result<UpdatedMatchResult, Error<()>> {
		self.run(|| {
			self.inner
				.manually_match_company()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn manually_match_platform(
		&self,
		body: CompanyOrPlatformMatchRequest,
	) -> Result<UpdatedMatchResult, Error<()>> {
		self.run(|| {
			self.inner
				.manually_match_platform()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn create_or_get_by_discord_id(
		&self,
		body: CreateOrGetUserRequest,
	) -> Result<User, Error<()>> {
		self.run(|| {
			self.inner
				.create_or_get_by_discord_id()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn update_user_permission_level(
		&self,
		id: Uuid,
		body: UpdateUserPermissionsRequest,
	) -> Result<User, Error<()>> {
		self.run(|| {
			self.inner
				.update_user_permission_level()
				.id(id)
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn get_igdb_game_by_id(&self, id: i32) -> Result<Game, Error<()>> {
		self.run(|| self.inner.get_igdb_game_by_id().id(id).send())
			.await
	}

	pub async fn get_igdb_cover_by_id(&self, id: i32) -> Result<Cover, Error<()>> {
		self.run(|| self.inner.get_igdb_cover_by_id().id(id).send())
			.await
	}

	pub async fn get_igdb_screenshot_by_id(&self, id: i32) -> Result<Screenshot, Error<()>> {
		self.run(|| self.inner.get_igdb_screenshot_by_id().id(id).send())
			.await
	}

	pub async fn get_igdb_company_by_id(&self, id: i32) -> Result<Company, Error<()>> {
		self.run(|| self.inner.get_igdb_company_by_id().id(id).send())
			.await
	}

	pub async fn get_igdb_company_logo_by_id(&self, id: i32) -> Result<CompanyLogo, Error<()>> {
		self.run(|| self.inner.get_igdb_company_logo_by_id().id(id).send())
			.await
	}

	pub async fn get_igdb_platform_by_id(&self, id: i32) -> Result<Platform, Error<()>> {
		self.run(|| self.inner.get_igdb_platform_by_id().id(id).send())
			.await
	}

	pub async fn get_igdb_platform_logo_by_id(&self, id: i32) -> Result<PlatformLogo, Error<()>> {
		self.run(|| self.inner.get_igdb_platform_logo_by_id().id(id).send())
			.await
	}

	pub async fn get_lb_game_by_id(&self, id: i64) -> Result<LbGame, Error<()>> {
		self.run(|| self.inner.get_lb_game_by_id().id(id).send())
			.await
	}

	pub async fn get_lb_game_images(&self, game_id: i64) -> Result<Vec<LbGameImage>, Error<()>> {
		self.run(|| self.inner.get_lb_game_images().game_id(game_id).send())
			.await
	}

	pub async fn get_mg_game_by_id(&self, id: i64) -> Result<MgGame, Error<()>> {
		self.run(|| self.inner.get_mg_game_by_id().id(id).send())
			.await
	}

	pub async fn get_ovgdb_release_by_id(
		&self,
		release_id: i64,
	) -> Result<OvgdbRelease, Error<()>> {
		self.run(|| {
			self.inner
				.get_ovgdb_release_by_id()
				.release_id(release_id)
				.send()
		})
		.await
	}

	pub async fn get_ra_game_by_id(&self, id: i64) -> Result<RaGame, Error<()>> {
		self.run(|| self.inner.get_ra_game_by_id().id(id).send())
			.await
	}

	pub async fn get_ss_game_by_id(&self, id: i64) -> Result<SsGame, Error<()>> {
		self.run(|| self.inner.get_ss_game_by_id().id(id).send())
			.await
	}

	pub async fn get_sgdb_game_by_id(&self, id: i64) -> Result<SgdbGame, Error<()>> {
		self.run(|| self.inner.get_sgdb_game_by_id().id(id).send())
			.await
	}
}

fn parse_retry_after(resp: &reqwest::Response) -> Option<Duration> {
	resp.headers()
		.get("retry-after")
		.or_else(|| resp.headers().get("x-ratelimit-after"))
		.and_then(|v| v.to_str().ok())
		.and_then(|s| s.trim().parse::<u64>().ok())
		.map(Duration::from_secs)
}
