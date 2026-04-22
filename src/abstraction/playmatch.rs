use crate::abstraction::command::{CommandContext, paginate};
use crate::abstraction::components_v2::{self, Status};
use crate::util::create_discord_markdown_table;
use log::warn;
use playmatch_client::Error;
use playmatch_client::types::{
	CompanyMetadataResponse, ExternalMetadata, MetadataMatchType, MetadataProvider,
	PlatformMetadataResponse,
};
use reqwest::StatusCode;
use uuid::Uuid;

pub enum ApiErrorAction {
	Suggest,
	Match,
}

pub async fn send_playmatch_api_error<E: std::fmt::Debug>(
	ctx: CommandContext<'_>,
	e: &Error<E>,
	entity: &str,
	identifier: &str,
	action: ApiErrorAction,
) -> Result<(), serenity::Error> {
	let action_verb = match action {
		ApiErrorAction::Suggest => "submit suggestion for",
		ApiErrorAction::Match => "match",
	};

	if let Error::ErrorResponse(e_res) = e {
		if e_res.status() == StatusCode::NOT_FOUND {
			ctx.send(components_v2::status_reply(
				Status::Error,
				format!("No {entity} found for the provided {identifier}."),
			))
			.await?;
			return Ok(());
		}
		if e_res.status() == StatusCode::CONFLICT && matches!(action, ApiErrorAction::Suggest) {
			ctx.send(components_v2::status_reply(
				Status::Error,
				format!("A suggestion for this {entity} already exists from the same provider."),
			))
			.await?;
			return Ok(());
		}
	}

	ctx.send(components_v2::status_reply(
		Status::Error,
		format!("Failed to {action_verb} {entity}: {e}"),
	))
	.await?;
	warn!("Failed to {action_verb} {entity}: {e}");
	Ok(())
}

pub trait PlaymatchResponse {
	fn get_id(&self) -> Uuid;
	fn get_name(&self) -> String;
	fn get_external_metadata(&self) -> Vec<ExternalMetadata>;
}

impl PlaymatchResponse for CompanyMetadataResponse {
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

impl PlaymatchResponse for PlatformMetadataResponse {
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
		"IGDB Match".to_string(),
		"IGDB ID".to_string(),
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
					MetadataMatchType::Automatic | MetadataMatchType::Manual => true,
					MetadataMatchType::Failed | MetadataMatchType::None => false,
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

	paginate(ctx, markdown_pages_ref.as_slice()).await?;

	Ok(())
}
