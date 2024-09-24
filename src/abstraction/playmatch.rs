use crate::abstraction::command::CommandContext;
use crate::util::create_discord_markdown_table;
use playmatch_client::types::{ExternalMetadata, MatchType, MetadataProvider};
use uuid::Uuid;

pub trait PlaymatchResponse {
	fn get_id(&self) -> Uuid;
	fn get_name(&self) -> String;
	fn get_external_metadata(&self) -> Vec<ExternalMetadata>;
}

impl PlaymatchResponse for playmatch_client::types::CompanyResponse {
	fn get_id(&self) -> Uuid {
		self.id
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn get_external_metadata(&self) -> Vec<ExternalMetadata> {
		self.external_metadata.clone()
	}
}

impl PlaymatchResponse for playmatch_client::types::PlatformResponse {
	fn get_id(&self) -> Uuid {
		self.id
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn get_external_metadata(&self) -> Vec<ExternalMetadata> {
		self.external_metadata.clone()
	}
}

pub async fn paginate_playmatch_response<T>(
	ctx: CommandContext<'_>,
	input: Vec<T>,
) -> anyhow::Result<()>
where
	T: PlaymatchResponse,
{
	const SUCCESS_EMOJI: &str = "✅";
	const FAILURE_EMOJI: &str = "❌";

	let default_headers: Vec<String> = vec![
		"Name".to_string(),
		"Igdb Match".to_string(),
		"Igdb Id".to_string(),
	];

	let mapped_markdown_rows = input
		.into_iter()
		.map(|c| {
			let external_metadata = c.get_external_metadata();
			let igdb_mapping = external_metadata
				.iter()
				.find(|ex| ex.provider_name == MetadataProvider::Igdb);

			let is_igdb_matched = if let Some(igdb_mapping) = igdb_mapping {
				match igdb_mapping.match_type {
					MatchType::Automatic | MatchType::Manual => true,
					MatchType::Failed | MatchType::None => false,
				}
			} else {
				false
			};

			let igdb_id = igdb_mapping
				.and_then(|ex| ex.provider_id.clone())
				.unwrap_or("".to_string());

			vec![
				c.get_name(),
				if is_igdb_matched {
					SUCCESS_EMOJI
				} else {
					FAILURE_EMOJI
				}
				.to_string(),
				igdb_id,
			]
		})
		.collect::<Vec<Vec<String>>>();

	let pages = mapped_markdown_rows
		.chunks(10)
		.map(|chunk| {
			let mut rows = vec![default_headers.to_vec()];
			rows.extend(chunk.to_vec());
			rows
		})
		.collect::<Vec<Vec<Vec<String>>>>();

	let markdown_pages = pages
		.iter()
		.map(|rows| create_discord_markdown_table(rows.to_vec()))
		.collect::<Vec<String>>();

	let markdown_pages_ref = markdown_pages
		.iter()
		.map(|s| s.as_str())
		.collect::<Vec<&str>>();

	poise::builtins::paginate(ctx, markdown_pages_ref.as_slice()).await?;

	Ok(())
}
