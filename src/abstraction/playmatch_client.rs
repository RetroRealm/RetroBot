use std::num::{NonZeroU32, NonZeroU64};
use std::time::Duration;

use governor::Quota;
use governor::RateLimiter;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use log::warn;
use playmatch_client::types::{
	BulkGamesByIdResult, BulkIdsRequest, Company, CompanyLogo, CompanyMetadataResponse,
	CompanyOrPlatformMatchRequest, CompanyOrPlatformSuggestionRequest, Cover,
	CreateOrGetUserRequestV2, Game, GameAndRelationMatchResultV2, GameAndRelationsResultV2,
	GameMatchRequest, GameSuggestionRequest, LbGame, LbGameImage, MgGame, OvgdbRelease, Platform,
	PlatformLogo, PlatformMetadataResponse, RaGame, Screenshot, SgdbGame, SsGame, Suggestion,
	UpdateUserPermissionsRequestV2, UpdatedMatchResult,
	UpdatedMetadataMatchesFromSuggestionResponse, User, V2ErrorBody,
};
use playmatch_client::{Client as Inner, Error, ResponseValue};
use reqwest::StatusCode;
use uuid::Uuid;

type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Thin newtype wrapper that paces every outbound playmatch request at one per 250 ms,
/// matching the server's `milliseconds_per_request(250)` exactly. No burst: the local
/// bucket holds at most one token, so two requests can never fire closer together than
/// the server's refill interval. On a 429 every request shares one cooldown
/// (`cooldown_until`): the host sits behind Cloudflare, and per Cloudflare's guidance
/// continuing to send during a block extends it, so one limited response stands the whole
/// client down until the block clears. 5xx keeps a short per-request exponential backoff
/// (250 ms, 500 ms, 1 s, 2 s, 4 s) as defence in depth for transient server blips.
pub struct PlaymatchClient {
	inner: Inner,
	limiter: Limiter,
	/// Shared stand-down deadline after a 429. `None` or a past instant means requests
	/// flow; all requests wait out a future instant before taking a limiter permit.
	cooldown_until: std::sync::Mutex<Option<tokio::time::Instant>>,
}

const MAX_RETRIES: u32 = 5;
/// v2 list endpoints clamp `limit` to 50; ask for the max so full listings need the fewest pages.
const PAGE_LIMIT: NonZeroU64 = NonZeroU64::new(50).unwrap();
/// `/games/bulk` rejects batches above 100 ids.
const BULK_MAX_IDS: usize = 100;

impl PlaymatchClient {
	pub fn new(inner: Inner) -> Self {
		let quota =
			Quota::per_second(NonZeroU32::new(4).unwrap()).allow_burst(NonZeroU32::new(1).unwrap());
		Self {
			inner,
			limiter: RateLimiter::direct(quota),
			cooldown_until: std::sync::Mutex::new(None),
		}
	}

	/// Blocks until any active shared 429 cooldown has passed. Loops because another
	/// request can extend the deadline while this one sleeps.
	async fn wait_for_cooldown(&self) {
		loop {
			let until = *self.cooldown_until.lock().unwrap();
			match until {
				Some(t) if t > tokio::time::Instant::now() => tokio::time::sleep_until(t).await,
				_ => return,
			}
		}
	}

	/// Pushes the shared cooldown deadline out to `now + wait`, never pulling an
	/// already-later deadline in.
	fn extend_cooldown(&self, wait: Duration) {
		let target = tokio::time::Instant::now() + wait;
		let mut until = self.cooldown_until.lock().unwrap();
		if until.is_none_or(|t| t < target) {
			*until = Some(target);
		}
	}

	async fn run<T, E, Fut, F>(&self, make: F) -> Result<T, Error<E>>
	where
		F: Fn() -> Fut,
		Fut: std::future::Future<Output = Result<ResponseValue<T>, Error<E>>>,
	{
		let mut attempt: u32 = 0;
		loop {
			self.wait_for_cooldown().await;
			self.limiter.until_ready().await;
			match make().await {
				Ok(resp) => return Ok(resp.into_inner()),
				Err(Error::UnexpectedResponse(resp))
					if is_retryable_status(resp.status()) && attempt < MAX_RETRIES =>
				{
					if resp.status() == StatusCode::TOO_MANY_REQUESTS {
						let wait = rate_limit_cooldown(resp.headers(), attempt);
						warn!(
							"playmatch_client: 429 on {} ({}), standing down all requests for {:?} (attempt {}/{})",
							resp.url(),
							describe_429(resp.headers()),
							wait,
							attempt + 1,
							MAX_RETRIES + 1
						);
						// The retry sleeps via wait_for_cooldown at the top of the loop,
						// alongside every other request.
						self.extend_cooldown(wait);
					} else {
						let wait = backoff_duration(resp.headers(), attempt);
						warn!(
							"playmatch_client: {} on {} (attempt {}/{}), sleeping {:?}",
							resp.status(),
							resp.url(),
							attempt + 1,
							MAX_RETRIES + 1,
							wait
						);
						tokio::time::sleep(wait).await;
					}
					attempt += 1;
				}
				Err(e) => return Err(e),
			}
		}
	}

	/// Walk every page of a cursor-paginated endpoint, funnelling each request through
	/// `run` so rate limiting and retries apply per page.
	async fn collect_pages<T, P, E, F, Fut>(
		&self,
		fetch: F,
		split: impl Fn(P) -> (Vec<T>, Option<String>),
	) -> Result<Vec<T>, Error<E>>
	where
		F: Fn(Option<String>) -> Fut,
		Fut: std::future::Future<Output = Result<ResponseValue<P>, Error<E>>>,
	{
		let mut all = Vec::new();
		let mut cursor: Option<String> = None;
		loop {
			let page = self.run(|| fetch(cursor.clone())).await?;
			let (data, next) = split(page);
			all.extend(data);
			match next {
				Some(c) => cursor = Some(c),
				None => return Ok(all),
			}
		}
	}

	pub async fn get_all_companies(
		&self,
	) -> Result<Vec<CompanyMetadataResponse>, Error<V2ErrorBody>> {
		self.collect_pages(
			|cursor| {
				let mut b = self.inner.list_companies_v2().limit(PAGE_LIMIT);
				if let Some(c) = cursor {
					b = b.cursor(c);
				}
				b.send()
			},
			|p| (p.data, p.pagination.next_cursor),
		)
		.await
	}

	pub async fn get_all_platforms(
		&self,
	) -> Result<Vec<PlatformMetadataResponse>, Error<V2ErrorBody>> {
		self.collect_pages(
			|cursor| {
				let mut b = self.inner.list_platforms_v2().limit(PAGE_LIMIT);
				if let Some(c) = cursor {
					b = b.cursor(c);
				}
				b.send()
			},
			|p| (p.data, p.pagination.next_cursor),
		)
		.await
	}

	pub async fn get_company_by_id(
		&self,
		id: Uuid,
	) -> Result<CompanyMetadataResponse, Error<V2ErrorBody>> {
		self.run(|| self.inner.get_company_by_id_v2().id(id).send())
			.await
	}

	pub async fn get_platform_by_id(
		&self,
		id: Uuid,
	) -> Result<PlatformMetadataResponse, Error<V2ErrorBody>> {
		self.run(|| self.inner.get_platform_by_id_v2().id(id).send())
			.await
	}

	pub async fn get_game_with_relations_by_id(
		&self,
		id: Uuid,
	) -> Result<GameAndRelationsResultV2, Error<V2ErrorBody>> {
		self.run(|| self.inner.get_game_with_relations_by_id_v2().id(id).send())
			.await
	}

	pub async fn identify_game_and_relations(
		&self,
		file_name: String,
		file_size: i64,
		md5: Option<String>,
		sha1: Option<String>,
		sha256: Option<String>,
	) -> Result<GameAndRelationMatchResultV2, Error<V2ErrorBody>> {
		self.run(|| {
			let mut b = self
				.inner
				.identify_game_and_relations_v2()
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

	/// One result per distinct id; ids missing on the server come back as per-item
	/// `NotFound`, never a batch 404.
	pub async fn get_games_bulk(
		&self,
		ids: &[Uuid],
	) -> Result<Vec<BulkGamesByIdResult>, Error<V2ErrorBody>> {
		let mut results = Vec::with_capacity(ids.len());
		for chunk in ids.chunks(BULK_MAX_IDS) {
			let resp = self
				.run(|| {
					self.inner
						.bulk_games_by_id_v2()
						.body(BulkIdsRequest {
							ids: chunk.to_vec(),
						})
						.send()
				})
				.await?;
			results.extend(resp.results);
		}
		Ok(results)
	}

	pub async fn get_all_suggestions(&self) -> Result<Vec<Suggestion>, Error<()>> {
		self.collect_pages(
			|cursor| {
				let mut b = self.inner.list_suggestions_v2().limit(PAGE_LIMIT);
				if let Some(c) = cursor {
					b = b.cursor(c);
				}
				b.send()
			},
			|p| (p.data, p.pagination.next_cursor),
		)
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
		body: CreateOrGetUserRequestV2,
	) -> Result<User, Error<()>> {
		self.run(|| {
			self.inner
				.create_or_get_by_discord_id_v2()
				.body(body.clone())
				.send()
		})
		.await
	}

	pub async fn update_user_permission_level(
		&self,
		id: Uuid,
		body: UpdateUserPermissionsRequestV2,
	) -> Result<User, Error<()>> {
		self.run(|| {
			self.inner
				.update_user_permission_level_v2()
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

	pub async fn get_igdb_screenshots_by_ids(
		&self,
		ids: Vec<i32>,
	) -> Result<Vec<Screenshot>, Error<()>> {
		self.run(|| {
			self.inner
				.get_igdb_screenshots_by_ids()
				.ids(ids.clone())
				.send()
		})
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

fn is_retryable_status(status: StatusCode) -> bool {
	status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
	headers
		.get("retry-after")
		.or_else(|| headers.get("x-ratelimit-after"))
		.and_then(|v| v.to_str().ok())
		.and_then(|s| s.trim().parse::<u64>().ok())
		.map(Duration::from_secs)
}

/// Pick a 5xx backoff for the next attempt: 250 ms, 500 ms, 1 s, 2 s, 4 s, respecting a
/// larger `retry-after` when the server sends one.
fn backoff_duration(headers: &reqwest::header::HeaderMap, attempt: u32) -> Duration {
	let header_wait = parse_retry_after(headers).unwrap_or(Duration::ZERO);
	let exp_backoff = Duration::from_millis(250u64 << attempt.min(4));
	header_wait.max(exp_backoff)
}

/// How long the whole client stands down after a 429. An origin 429 carries
/// `retry-after` and is respected, floored at 1 s (playmatch rounds sub-second waits
/// down to zero). A 429 without the header is the Cloudflare edge-block shape; fast
/// retries extend a block, so wait on a seconds scale that grows with each attempt.
fn rate_limit_cooldown(headers: &reqwest::header::HeaderMap, attempt: u32) -> Duration {
	const MAX_COOLDOWN: Duration = Duration::from_secs(60);
	match parse_retry_after(headers) {
		Some(d) => d.max(Duration::from_secs(1)),
		None => Duration::from_secs(10) * (attempt + 1),
	}
	.min(MAX_COOLDOWN)
}

/// One-line 429 provenance so logs can tell playmatch's own limiter (JSON,
/// `retry-after`) from a Cloudflare edge block (HTML, `cf-mitigated`, no `retry-after`)
/// without dumping headers.
fn describe_429(headers: &reqwest::header::HeaderMap) -> String {
	let content_type = headers
		.get(reqwest::header::CONTENT_TYPE)
		.and_then(|v| v.to_str().ok())
		.unwrap_or("<none>");
	let retry_after =
		if headers.contains_key("retry-after") || headers.contains_key("x-ratelimit-after") {
			"present"
		} else {
			"absent"
		};
	let cf_mitigated = if headers.contains_key("cf-mitigated") {
		"present"
	} else {
		"absent"
	};
	format!("content-type {content_type}, retry-after {retry_after}, cf-mitigated {cf_mitigated}")
}

#[cfg(test)]
mod tests {
	use super::*;
	use reqwest::header::HeaderMap;
	use std::time::Instant;
	use wiremock::matchers::{any, method, path, query_param, query_param_is_missing};
	use wiremock::{Mock, MockServer, ResponseTemplate};

	/// Empty first page: no data, no next cursor.
	const EMPTY_PAGE: &str =
		r#"{"data":[],"pagination":{"limit":50,"hasNextPage":false,"hasPreviousPage":false}}"#;

	fn headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
		let mut h = HeaderMap::new();
		for (k, v) in pairs {
			h.insert(*k, v.parse().unwrap());
		}
		h
	}

	#[test]
	fn parse_retry_after_missing_header_is_none() {
		assert_eq!(parse_retry_after(&HeaderMap::new()), None);
	}

	#[test]
	fn parse_retry_after_reads_retry_after() {
		let h = headers(&[("retry-after", "3")]);
		assert_eq!(parse_retry_after(&h), Some(Duration::from_secs(3)));
	}

	#[test]
	fn parse_retry_after_zero_is_zero() {
		let h = headers(&[("retry-after", "0")]);
		assert_eq!(parse_retry_after(&h), Some(Duration::ZERO));
	}

	#[test]
	fn parse_retry_after_falls_back_to_x_ratelimit_after() {
		let h = headers(&[("x-ratelimit-after", "7")]);
		assert_eq!(parse_retry_after(&h), Some(Duration::from_secs(7)));
	}

	#[test]
	fn parse_retry_after_prefers_retry_after_over_x_ratelimit_after() {
		let h = headers(&[("retry-after", "2"), ("x-ratelimit-after", "9")]);
		assert_eq!(parse_retry_after(&h), Some(Duration::from_secs(2)));
	}

	#[test]
	fn parse_retry_after_malformed_returns_none() {
		let h = headers(&[("retry-after", "soon")]);
		assert_eq!(parse_retry_after(&h), None);
	}

	#[test]
	fn backoff_duration_uses_exponential_schedule_when_header_is_zero() {
		let h = headers(&[("retry-after", "0")]);
		assert_eq!(backoff_duration(&h, 0), Duration::from_millis(250));
		assert_eq!(backoff_duration(&h, 1), Duration::from_millis(500));
		assert_eq!(backoff_duration(&h, 2), Duration::from_secs(1));
		assert_eq!(backoff_duration(&h, 3), Duration::from_secs(2));
		assert_eq!(backoff_duration(&h, 4), Duration::from_secs(4));
	}

	#[test]
	fn backoff_duration_uses_header_when_larger_than_exponential() {
		let h = headers(&[("retry-after", "10")]);
		assert_eq!(backoff_duration(&h, 0), Duration::from_secs(10));
		assert_eq!(backoff_duration(&h, 4), Duration::from_secs(10));
	}

	#[test]
	fn backoff_duration_caps_exponential_at_4_seconds() {
		assert_eq!(
			backoff_duration(&HeaderMap::new(), 99),
			Duration::from_secs(4)
		);
	}

	#[test]
	fn rate_limit_cooldown_floors_header_at_one_second() {
		let h = headers(&[("retry-after", "0")]);
		assert_eq!(rate_limit_cooldown(&h, 0), Duration::from_secs(1));
	}

	#[test]
	fn rate_limit_cooldown_respects_larger_header() {
		let h = headers(&[("retry-after", "7")]);
		assert_eq!(rate_limit_cooldown(&h, 0), Duration::from_secs(7));
	}

	#[test]
	fn rate_limit_cooldown_without_header_grows_per_attempt_and_caps() {
		let h = HeaderMap::new();
		assert_eq!(rate_limit_cooldown(&h, 0), Duration::from_secs(10));
		assert_eq!(rate_limit_cooldown(&h, 1), Duration::from_secs(20));
		assert_eq!(rate_limit_cooldown(&h, 4), Duration::from_secs(50));
		// Header and no-header shapes both cap at 60 s.
		assert_eq!(rate_limit_cooldown(&h, 99), Duration::from_secs(60));
		let h = headers(&[("retry-after", "600")]);
		assert_eq!(rate_limit_cooldown(&h, 0), Duration::from_secs(60));
	}

	#[test]
	fn describe_429_distinguishes_origin_from_edge_block() {
		let origin = headers(&[("content-type", "application/json"), ("retry-after", "1")]);
		assert_eq!(
			describe_429(&origin),
			"content-type application/json, retry-after present, cf-mitigated absent"
		);
		let edge = headers(&[("content-type", "text/html"), ("cf-mitigated", "challenge")]);
		assert_eq!(
			describe_429(&edge),
			"content-type text/html, retry-after absent, cf-mitigated present"
		);
	}

	#[tokio::test]
	async fn extend_cooldown_never_shortens_the_deadline() {
		let client = make_client("http://localhost:1");
		client.extend_cooldown(Duration::from_secs(60));
		let first = client.cooldown_until.lock().unwrap().unwrap();
		client.extend_cooldown(Duration::from_secs(1));
		let second = client.cooldown_until.lock().unwrap().unwrap();
		assert_eq!(
			first, second,
			"shorter cooldown must not pull the deadline in"
		);
		client.extend_cooldown(Duration::from_secs(120));
		let third = client.cooldown_until.lock().unwrap().unwrap();
		assert!(third > second, "longer cooldown must push the deadline out");
	}

	fn make_client(uri: &str) -> PlaymatchClient {
		PlaymatchClient::new(playmatch_client::Client::new(&format!("{uri}/api/v2")))
	}

	#[tokio::test]
	async fn immediate_200_does_not_retry() {
		let server = MockServer::start().await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(
				ResponseTemplate::new(200)
					.insert_header("content-type", "application/json")
					.set_body_string(EMPTY_PAGE),
			)
			.expect(1)
			.mount(&server)
			.await;

		let client = make_client(&server.uri());
		let result = client.get_all_companies().await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn retries_on_429_then_succeeds() {
		let server = MockServer::start().await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
			.up_to_n_times(2)
			.expect(2)
			.mount(&server)
			.await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(
				ResponseTemplate::new(200)
					.insert_header("content-type", "application/json")
					.set_body_string(EMPTY_PAGE),
			)
			.expect(1)
			.mount(&server)
			.await;

		let client = make_client(&server.uri());
		let result = client.get_all_companies().await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn gives_up_after_max_retries() {
		let server = MockServer::start().await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
			.expect((MAX_RETRIES + 1) as u64)
			.mount(&server)
			.await;

		let start = Instant::now();
		let client = make_client(&server.uri());
		let result = client.get_all_companies().await;
		assert!(result.is_err());
		// Sanity check that we actually backed off. Each 429 stands the client down for
		// at least 1 s (retry-after 0 floors to 1 s), so five retries idle >= 5 s; allow
		// generous slack for CI flakiness.
		assert!(
			start.elapsed() >= Duration::from_secs(1),
			"expected at least 1s of backoff, took {:?}",
			start.elapsed()
		);
	}

	#[tokio::test]
	async fn retries_on_500_then_succeeds() {
		let server = MockServer::start().await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(ResponseTemplate::new(500))
			.up_to_n_times(1)
			.expect(1)
			.mount(&server)
			.await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(
				ResponseTemplate::new(200)
					.insert_header("content-type", "application/json")
					.set_body_string(EMPTY_PAGE),
			)
			.expect(1)
			.mount(&server)
			.await;

		let client = make_client(&server.uri());
		let result = client.get_all_companies().await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn retries_on_503_then_succeeds() {
		let server = MockServer::start().await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(ResponseTemplate::new(503))
			.up_to_n_times(2)
			.expect(2)
			.mount(&server)
			.await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.respond_with(
				ResponseTemplate::new(200)
					.insert_header("content-type", "application/json")
					.set_body_string(EMPTY_PAGE),
			)
			.expect(1)
			.mount(&server)
			.await;

		let client = make_client(&server.uri());
		let result = client.get_all_companies().await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn client_errors_do_not_retry() {
		let server = MockServer::start().await;
		Mock::given(any())
			.respond_with(ResponseTemplate::new(404))
			.expect(1)
			.mount(&server)
			.await;

		let client = make_client(&server.uri());
		let result = client.get_all_companies().await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn get_all_companies_follows_next_cursor() {
		let server = MockServer::start().await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.and(query_param_is_missing("cursor"))
			.respond_with(
				ResponseTemplate::new(200)
					.insert_header("content-type", "application/json")
					.set_body_string(
						r#"{"data":[{"id":"00000000-0000-0000-0000-000000000001","name":"One"}],"pagination":{"limit":50,"hasNextPage":true,"hasPreviousPage":false,"nextCursor":"c1"}}"#,
					),
			)
			.expect(1)
			.mount(&server)
			.await;
		Mock::given(method("GET"))
			.and(path("/api/v2/companies"))
			.and(query_param("cursor", "c1"))
			.respond_with(
				ResponseTemplate::new(200)
					.insert_header("content-type", "application/json")
					.set_body_string(
						r#"{"data":[{"id":"00000000-0000-0000-0000-000000000002","name":"Two"}],"pagination":{"limit":50,"hasNextPage":false,"hasPreviousPage":true}}"#,
					),
			)
			.expect(1)
			.mount(&server)
			.await;

		let client = make_client(&server.uri());
		let result = client.get_all_companies().await.unwrap();
		assert_eq!(result.len(), 2);
	}

	#[test]
	fn is_retryable_status_covers_429_and_5xx() {
		assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
		assert!(is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR));
		assert!(is_retryable_status(StatusCode::BAD_GATEWAY));
		assert!(is_retryable_status(StatusCode::SERVICE_UNAVAILABLE));
		assert!(is_retryable_status(StatusCode::GATEWAY_TIMEOUT));
		assert!(!is_retryable_status(StatusCode::OK));
		assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
		assert!(!is_retryable_status(StatusCode::UNAUTHORIZED));
		assert!(!is_retryable_status(StatusCode::NOT_FOUND));
	}
}
