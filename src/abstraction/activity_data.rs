use serenity::all::ActivityData;

pub trait FromStringTuple {
	fn from_tuple(first: &str, second: &str) -> Self;
}

impl FromStringTuple for ActivityData {
	fn from_tuple(first: &str, second: &str) -> Self {
		match first.to_lowercase().as_str() {
			"playing" => ActivityData::playing(second),
			"listening" => ActivityData::listening(second),
			"watching" => ActivityData::watching(second),
			"competing" => ActivityData::competing(second),
			_ => ActivityData::custom(format!("{} {}", first, second)),
		}
	}
}
