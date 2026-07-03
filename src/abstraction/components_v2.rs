use chrono::{DateTime, Utc};
use log::warn;
use poise::CreateReply;
use reqwest::Url;
use serenity::all::{
	Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer,
	CreateContainerComponent, CreateMediaGallery, CreateMediaGalleryItem, CreateMessage,
	CreateSection, CreateSectionAccessory, CreateSectionComponent, CreateSeparator,
	CreateTextDisplay, CreateThumbnail, CreateUnfurledMediaItem, EditMessage, MessageFlags,
};
use std::borrow::Cow;

/// Normalizes a URL that came from an external metadata source before it is
/// handed to a Discord component (link button, thumbnail, media gallery).
///
/// Discord rejects the *entire* message payload if a single link button's URL
/// lacks an `http`/`https`/`discord` scheme, so one bad field (e.g. LaunchBox's
/// bare-domain `en.wikipedia.org/wiki/...` Wikipedia links) would otherwise take
/// down the whole card. Callers turn `None` into "omit that link/section" so a
/// malformed value degrades gracefully instead of failing the post.
///
/// Behavior:
/// - trims surrounding whitespace,
/// - accepts an absolute `http`/`https` URL as-is,
/// - treats a protocol-relative `//host/path` as `https://host/path` (mirrors the
///   IGDB image normalizer, which already upgrades IGDB's `//images.igdb.com/...`
///   URLs the same way),
/// - retries a scheme-less bare host/path (`en.wikipedia.org/wiki/...`) with
///   `https://` prepended,
/// - rejects everything else, including `discord://` deep links (not useful on the
///   link buttons we build) and other non-http(s) schemes.
pub fn normalize_external_url(raw: &str) -> Option<String> {
	let trimmed = raw.trim();
	if trimmed.is_empty() {
		return None;
	}

	// Protocol-relative "//host/path": upgrade to https before parsing so it goes
	// down the absolute-URL path below instead of being treated as bare host/path.
	let candidate = match trimmed.strip_prefix("//") {
		Some(rest) => Cow::Owned(format!("https://{rest}")),
		None => Cow::Borrowed(trimmed),
	};

	match Url::parse(&candidate) {
		// Already absolute with a Discord-accepted scheme: pass through.
		Ok(url) if matches!(url.scheme(), "http" | "https") => Some(url.into()),
		// Absolute URL with some other scheme (mailto:, discord://, ftp://, …).
		// Reject: Discord only allows http/https/discord on link buttons and we
		// never build discord:// links, so anything non-http here is unusable.
		Ok(_) => None,
		// Any parse error is treated as a possible scheme-less bare host/path
		// (`en.wikipedia.org/wiki/...`). Retry with an explicit https:// prefix
		// and keep it only if it now parses into a real, non-empty host. This
		// drops garbage like "not a url" whose prefixed form has no valid host.
		Err(_) => Url::parse(&format!("https://{candidate}"))
			.ok()
			.filter(|url| url.host_str().is_some_and(|h| !h.is_empty()))
			.map(Into::into),
	}
}

/// `<t:unix:R>` — renders as a live, localized relative time ("3 hours ago").
pub fn relative_timestamp(dt: DateTime<Utc>) -> String {
	format!("<t:{}:R>", dt.timestamp())
}

/// `<t:unix:D>` — renders as a localized long date ("21 November 1992").
pub fn long_date(dt: DateTime<Utc>) -> String {
	format!("<t:{}:D>", dt.timestamp())
}

#[derive(Clone, Copy)]
pub enum Status {
	Success,
	Error,
	Info,
}

fn accent(status: Status) -> Colour {
	match status {
		Status::Success => Colour(0x2ECC71),
		Status::Error => Colour::RED,
		Status::Info => Colour(0x3498DB),
	}
}

fn is_ephemeral(status: Status) -> bool {
	matches!(status, Status::Error)
}

fn container_reply<'a>(container: CreateContainer<'a>) -> CreateReply<'a> {
	CreateReply::new()
		.flags(MessageFlags::IS_COMPONENTS_V2)
		.components(vec![CreateComponent::Container(container)])
}

fn container_message<'a>(container: CreateContainer<'a>) -> CreateMessage<'a> {
	CreateMessage::new()
		.flags(MessageFlags::IS_COMPONENTS_V2)
		.components(vec![CreateComponent::Container(container)])
}

fn container_edit<'a>(container: CreateContainer<'a>) -> EditMessage<'a> {
	EditMessage::new()
		.flags(MessageFlags::IS_COMPONENTS_V2)
		.components(vec![CreateComponent::Container(container)])
}

/// Error and Warning statuses are sent as ephemeral.
pub fn status_reply<'a, S: Into<Cow<'a, str>>>(status: Status, text: S) -> CreateReply<'a> {
	let container = CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(text),
	)])
	.accent_colour(accent(status));

	let reply = container_reply(container);
	if is_ephemeral(status) {
		reply.ephemeral(true)
	} else {
		reply
	}
}

pub fn reply_from_container<'a>(container: CreateContainer<'a>) -> CreateReply<'a> {
	container_reply(container)
}

pub fn message_from_container<'a>(container: CreateContainer<'a>) -> CreateMessage<'a> {
	container_message(container)
}

pub fn paginate_container<'a>(
	body: String,
	page: usize,
	total_pages: usize,
	buttons: Vec<CreateButton<'a>>,
) -> CreateContainer<'a> {
	CreateContainer::new(vec![
		CreateContainerComponent::TextDisplay(CreateTextDisplay::new(body)),
		CreateContainerComponent::Separator(CreateSeparator::new()),
		CreateContainerComponent::TextDisplay(CreateTextDisplay::new(format!(
			"Page {} / {}",
			page + 1,
			total_pages
		))),
		CreateContainerComponent::ActionRow(CreateActionRow::buttons(buttons)),
	])
	.accent_colour(Colour::BLURPLE)
}

enum CardBlock<'a> {
	Row {
		label: Cow<'a, str>,
		value: Cow<'a, str>,
	},
	Text(Cow<'a, str>),
	Separator,
	SectionHeader(Cow<'a, str>),
}

/// Changing how a `Card` renders changes any persistent card built from it: bump
/// `command::playmatch::SUGGESTION_CARD_LAYOUT_VERSION` so tracked suggestion cards
/// re-render on the next boot.
pub struct Card<'a> {
	status: Status,
	heading: Cow<'a, str>,
	subheading: Option<Cow<'a, str>>,
	intro: Option<Cow<'a, str>>,
	thumbnail_url: Option<Cow<'a, str>>,
	body: Vec<CardBlock<'a>>,
	media: Vec<CreateMediaGalleryItem<'a>>,
	links: Vec<CreateButton<'a>>,
	footer: Option<Cow<'a, str>>,
}

impl<'a> Card<'a> {
	pub fn new(status: Status, heading: impl Into<Cow<'a, str>>) -> Self {
		Self {
			status,
			heading: heading.into(),
			subheading: None,
			intro: None,
			thumbnail_url: None,
			body: Vec::new(),
			media: Vec::new(),
			links: Vec::new(),
			footer: None,
		}
	}

	pub fn subheading(mut self, text: impl Into<Cow<'a, str>>) -> Self {
		self.subheading = Some(text.into());
		self
	}

	pub fn intro(mut self, text: impl Into<Cow<'a, str>>) -> Self {
		self.intro = Some(text.into());
		self
	}

	/// External image URL from a provider. A URL that can't be normalized to an
	/// http(s) scheme is dropped so it can't fail the whole message.
	pub fn thumbnail(mut self, url: impl Into<Cow<'a, str>>) -> Self {
		self.thumbnail_url = normalize_external_url(&url.into()).map(Cow::Owned);
		self
	}

	pub fn section(mut self, title: impl Into<Cow<'a, str>>) -> Self {
		self.body.push(CardBlock::SectionHeader(title.into()));
		self
	}

	pub fn row(mut self, label: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
		self.body.push(CardBlock::Row {
			label: label.into(),
			value: value.into(),
		});
		self
	}

	pub fn text(mut self, body: impl Into<Cow<'a, str>>) -> Self {
		self.body.push(CardBlock::Text(body.into()));
		self
	}

	/// Body text rendered as a Discord blockquote. Each line is prefixed
	/// individually because Discord's `> ` only quotes a single line.
	pub fn quote(mut self, text: impl AsRef<str>) -> Self {
		let quoted = text
			.as_ref()
			.lines()
			.map(|line| format!("> {line}"))
			.collect::<Vec<_>>()
			.join("\n");
		self.body.push(CardBlock::Text(quoted.into()));
		self
	}

	pub fn separator(mut self) -> Self {
		self.body.push(CardBlock::Separator);
		self
	}

	/// External image URLs from a provider. Any URL that can't be normalized to
	/// an http(s) scheme is skipped so it can't fail the whole message.
	pub fn media(mut self, urls: impl IntoIterator<Item = String>) -> Self {
		for url in urls {
			if self.media.len() >= 10 {
				warn!("Card media gallery capped at 10 items, dropping extras");
				break;
			}
			let Some(url) = normalize_external_url(&url) else {
				warn!("dropping malformed media URL from card");
				continue;
			};
			self.media
				.push(CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new(
					url,
				)));
		}
		self
	}

	/// Adds a pre-built button (e.g. an Approve/Decline `custom_id` button).
	/// For externally sourced link URLs use [`Card::link_external`] instead, which
	/// validates the scheme so a bad URL can't fail the whole message.
	pub fn link(mut self, button: CreateButton<'a>) -> Self {
		self.links.push(button);
		self
	}

	/// Adds a link button for an externally sourced URL. The URL is normalized to
	/// an http(s) scheme; if it can't be, the button is omitted (the rest of the
	/// card still posts) rather than letting Discord reject the whole message.
	pub fn link_external(
		mut self,
		url: impl Into<Cow<'a, str>>,
		label: impl Into<Cow<'a, str>>,
	) -> Self {
		match normalize_external_url(&url.into()) {
			Some(url) => self
				.links
				.push(CreateButton::new_link(url).label(label.into())),
			None => warn!("dropping malformed link URL from card"),
		}
		self
	}

	pub fn footer(mut self, text: impl Into<Cow<'a, str>>) -> Self {
		self.footer = Some(text.into());
		self
	}

	pub fn into_container(self) -> CreateContainer<'a> {
		let mut components: Vec<CreateContainerComponent<'a>> = Vec::new();

		let mut heading_texts: Vec<Cow<'a, str>> = vec![format!("## {}", self.heading).into()];
		if let Some(sub) = self.subheading {
			heading_texts.push(format!("-# {sub}").into());
		}
		if let Some(intro) = self.intro {
			heading_texts.push(intro);
		}

		if let Some(thumb_url) = self.thumbnail_url {
			let section_children: Vec<CreateSectionComponent<'a>> = heading_texts
				.into_iter()
				.take(3)
				.map(|t| CreateSectionComponent::TextDisplay(CreateTextDisplay::new(t)))
				.collect();
			let thumb = CreateThumbnail::new(CreateUnfurledMediaItem::new(thumb_url));
			let section =
				CreateSection::new(section_children, CreateSectionAccessory::Thumbnail(thumb));
			components.push(CreateContainerComponent::Section(section));
		} else {
			for t in heading_texts {
				components.push(CreateContainerComponent::TextDisplay(
					CreateTextDisplay::new(t),
				));
			}
		}

		let has_body_after = !self.body.is_empty()
			|| !self.media.is_empty()
			|| !self.links.is_empty()
			|| self.footer.is_some();
		if has_body_after {
			components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
		}

		for block in self.body {
			match block {
				CardBlock::Row { label, value } => {
					components.push(CreateContainerComponent::TextDisplay(
						CreateTextDisplay::new(format!("**{label}:** {value}")),
					));
				}
				CardBlock::Text(t) => {
					components.push(CreateContainerComponent::TextDisplay(
						CreateTextDisplay::new(t),
					));
				}
				CardBlock::Separator => {
					components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
				}
				CardBlock::SectionHeader(title) => {
					components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
					components.push(CreateContainerComponent::TextDisplay(
						CreateTextDisplay::new(format!("### {title}")),
					));
				}
			}
		}

		if !self.media.is_empty() {
			components.push(CreateContainerComponent::MediaGallery(
				CreateMediaGallery::new(self.media),
			));
		}

		if !self.links.is_empty() {
			components.push(CreateContainerComponent::ActionRow(
				CreateActionRow::buttons(self.links),
			));
		}

		if let Some(footer) = self.footer {
			components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
			components.push(CreateContainerComponent::TextDisplay(
				CreateTextDisplay::new(format!("-# {footer}")),
			));
		}

		CreateContainer::new(components).accent_colour(accent(self.status))
	}

	pub fn into_reply(self) -> CreateReply<'a> {
		let ephemeral = is_ephemeral(self.status);
		let reply = container_reply(self.into_container());
		if ephemeral {
			reply.ephemeral(true)
		} else {
			reply
		}
	}

	pub fn into_message(self) -> CreateMessage<'a> {
		container_message(self.into_container())
	}

	pub fn into_edit(self) -> EditMessage<'a> {
		container_edit(self.into_container())
	}
}

#[cfg(test)]
mod tests {
	use super::{Card, Status, long_date, normalize_external_url, relative_timestamp};
	use chrono::{TimeZone, Utc};

	#[test]
	fn quote_prefixes_every_line() {
		let card = Card::new(Status::Info, "h").quote("first line\nsecond line");
		match &card.body[0] {
			super::CardBlock::Text(t) => assert_eq!(t.as_ref(), "> first line\n> second line"),
			_ => panic!("quote should push a Text block"),
		}
	}

	#[test]
	fn timestamps_format_as_discord_markers() {
		// 1992-11-21T00:00:00Z.
		let dt = Utc.with_ymd_and_hms(1992, 11, 21, 0, 0, 0).unwrap();
		let unix = dt.timestamp();
		assert_eq!(relative_timestamp(dt), format!("<t:{unix}:R>"));
		assert_eq!(long_date(dt), format!("<t:{unix}:D>"));
	}

	#[test]
	fn scheme_less_host_path_gets_https() {
		// The production incident: LaunchBox's bare-domain Wikipedia link.
		assert_eq!(
			normalize_external_url("en.wikipedia.org/wiki/Sink_or_Swim_(video_game)").as_deref(),
			Some("https://en.wikipedia.org/wiki/Sink_or_Swim_(video_game)"),
		);
	}

	#[test]
	fn valid_https_passes_through() {
		assert_eq!(
			normalize_external_url("https://www.igdb.com/games/foo").as_deref(),
			Some("https://www.igdb.com/games/foo"),
		);
	}

	#[test]
	fn valid_http_passes_through() {
		assert_eq!(
			normalize_external_url("http://example.com/").as_deref(),
			Some("http://example.com/"),
		);
	}

	#[test]
	fn whitespace_is_trimmed() {
		assert_eq!(
			normalize_external_url("  https://example.com/path  ").as_deref(),
			Some("https://example.com/path"),
		);
	}

	#[test]
	fn garbage_is_rejected() {
		assert_eq!(normalize_external_url("not a url"), None);
	}

	#[test]
	fn empty_is_rejected() {
		assert_eq!(normalize_external_url(""), None);
		assert_eq!(normalize_external_url("   "), None);
	}

	#[test]
	fn protocol_relative_is_upgraded_to_https() {
		// Matches the IGDB image normalizer, which turns IGDB's //images.igdb.com
		// URLs into https. Discord needs an explicit scheme, so we supply https.
		assert_eq!(
			normalize_external_url("//images.example.com/a.png").as_deref(),
			Some("https://images.example.com/a.png"),
		);
	}

	#[test]
	fn discord_scheme_is_rejected() {
		// Discord technically allows discord:// on buttons, but we never build
		// deep links, so an incoming discord:// from a metadata source is unusable.
		assert_eq!(normalize_external_url("discord://channel/1"), None);
	}

	#[test]
	fn other_absolute_schemes_are_rejected() {
		assert_eq!(normalize_external_url("mailto:a@b.com"), None);
		assert_eq!(normalize_external_url("ftp://host/file"), None);
	}
}
