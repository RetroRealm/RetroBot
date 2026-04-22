use log::warn;
use poise::CreateReply;
use serenity::all::{
	Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer,
	CreateContainerComponent, CreateMediaGallery, CreateMediaGalleryItem, CreateMessage,
	CreateSection, CreateSectionAccessory, CreateSectionComponent, CreateSeparator,
	CreateTextDisplay, CreateThumbnail, CreateUnfurledMediaItem, MessageFlags,
};
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub enum Status {
	Success,
	Error,
	Info,
	Warning,
}

fn accent(status: Status) -> Colour {
	match status {
		Status::Success => Colour(0x2ECC71),
		Status::Error => Colour::RED,
		Status::Info => Colour(0x3498DB),
		Status::Warning => Colour::ORANGE,
	}
}

fn is_ephemeral(status: Status) -> bool {
	matches!(status, Status::Error | Status::Warning)
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

	pub fn thumbnail(mut self, url: impl Into<Cow<'a, str>>) -> Self {
		self.thumbnail_url = Some(url.into());
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

	pub fn separator(mut self) -> Self {
		self.body.push(CardBlock::Separator);
		self
	}

	pub fn media(mut self, urls: impl IntoIterator<Item = String>) -> Self {
		for url in urls {
			if self.media.len() >= 10 {
				warn!("Card media gallery capped at 10 items, dropping extras");
				break;
			}
			self.media
				.push(CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new(
					url,
				)));
		}
		self
	}

	pub fn link(mut self, button: CreateButton<'a>) -> Self {
		self.links.push(button);
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
}
