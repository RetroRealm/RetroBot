use poise::CreateReply;
use serenity::all::{
	Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer,
	CreateContainerComponent, CreateMessage, CreateSeparator, CreateTextDisplay, MessageFlags,
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
		Status::Info => Colour::BLUE,
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

/// One-line Container reply with a TextDisplay, accent-coloured by `status`.
/// Errors and warnings are auto-ephemeral.
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

/// Wrap an owned Container as a V2-flagged reply (for commands that need custom layout).
pub fn reply_from_container<'a>(container: CreateContainer<'a>) -> CreateReply<'a> {
	container_reply(container)
}

/// Wrap an owned Container as a V2-flagged CreateMessage (for ChannelId::send_message /
/// User::dm / interaction responses).
pub fn message_from_container<'a>(container: CreateContainer<'a>) -> CreateMessage<'a> {
	container_message(container)
}

/// Card reply: heading + separator + one labelled line per (label, value) row.
/// Values are rendered with `**label:** value` markdown.
pub fn card_reply<'a>(
	status: Status,
	heading: impl Into<Cow<'a, str>>,
	rows: Vec<(String, String)>,
) -> CreateReply<'a> {
	let heading_cow: Cow<'a, str> = heading.into();
	let heading_text = format!("## {heading_cow}");

	let mut components: Vec<CreateContainerComponent<'a>> = Vec::with_capacity(rows.len() + 2);

	components.push(CreateContainerComponent::TextDisplay(
		CreateTextDisplay::new(heading_text),
	));
	components.push(CreateContainerComponent::Separator(CreateSeparator::new()));

	for (label, value) in rows {
		components.push(CreateContainerComponent::TextDisplay(
			CreateTextDisplay::new(format!("**{label}:** {value}")),
		));
	}

	let container = CreateContainer::new(components).accent_colour(accent(status));
	container_reply(container)
}

/// Pagination container: body TextDisplay, separator, page indicator, navigation buttons row.
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
