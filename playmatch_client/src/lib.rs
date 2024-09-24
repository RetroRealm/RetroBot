mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	#[allow(unused_imports)]
	use std::convert::TryFrom;
	/// Error types.
	pub mod error {
		/// Error from a TryFrom or FromStr implementation.
		pub struct ConversionError(::std::borrow::Cow<'static, str>);
		impl ::std::error::Error for ConversionError {}
		impl ::std::fmt::Display for ConversionError {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
				::std::fmt::Display::fmt(&self.0, f)
			}
		}

		impl ::std::fmt::Debug for ConversionError {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
				::std::fmt::Debug::fmt(&self.0, f)
			}
		}

		impl From<&'static str> for ConversionError {
			fn from(value: &'static str) -> Self {
				Self(value.into())
			}
		}

		impl From<String> for ConversionError {
			fn from(value: String) -> Self {
				Self(value.into())
			}
		}
	}

	///AgeRating
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "id",
	///    "rating"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/AgeRatingCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "content_descriptions": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int64"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "rating": {
	///      "$ref": "#/components/schemas/AgeRatingEnum"
	///    },
	///    "rating_cover_url": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "synopsis": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct AgeRating {
		pub category: AgeRatingCategory,
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub content_descriptions: Option<Vec<i64>>,
		pub id: i32,
		pub rating: AgeRatingEnum,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rating_cover_url: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub synopsis: Option<String>,
	}

	impl From<&AgeRating> for AgeRating {
		fn from(value: &AgeRating) -> Self {
			value.clone()
		}
	}

	///AgeRatingCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct AgeRatingCategory(i64);
	impl ::std::ops::Deref for AgeRatingCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<AgeRatingCategory> for i64 {
		fn from(value: AgeRatingCategory) -> Self {
			value.0
		}
	}

	impl From<&AgeRatingCategory> for AgeRatingCategory {
		fn from(value: &AgeRatingCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for AgeRatingCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AgeRatingCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///AgeRatingContentCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7,
	///    8,
	///    9,
	///    10,
	///    11,
	///    12,
	///    13,
	///    14,
	///    15,
	///    16,
	///    17,
	///    18,
	///    19,
	///    20,
	///    21,
	///    22,
	///    23,
	///    24,
	///    25,
	///    26,
	///    27,
	///    28,
	///    29,
	///    30,
	///    31,
	///    32,
	///    33,
	///    34,
	///    35,
	///    36,
	///    37,
	///    38,
	///    39,
	///    40,
	///    41,
	///    42,
	///    43,
	///    44,
	///    45,
	///    46,
	///    47,
	///    48,
	///    49,
	///    50,
	///    51,
	///    52,
	///    53,
	///    54,
	///    55,
	///    56,
	///    57,
	///    58,
	///    59,
	///    60,
	///    61,
	///    62,
	///    63,
	///    64,
	///    65,
	///    66,
	///    67,
	///    68,
	///    69,
	///    70,
	///    71,
	///    72,
	///    73,
	///    74,
	///    75,
	///    76,
	///    77,
	///    78,
	///    79,
	///    80,
	///    81,
	///    82,
	///    83,
	///    84,
	///    85
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct AgeRatingContentCategory(i64);
	impl ::std::ops::Deref for AgeRatingContentCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<AgeRatingContentCategory> for i64 {
		fn from(value: AgeRatingContentCategory) -> Self {
			value.0
		}
	}

	impl From<&AgeRatingContentCategory> for AgeRatingContentCategory {
		fn from(value: &AgeRatingContentCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for AgeRatingContentCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64, 11_i64,
				12_i64, 13_i64, 14_i64, 15_i64, 16_i64, 17_i64, 18_i64, 19_i64, 20_i64, 21_i64,
				22_i64, 23_i64, 24_i64, 25_i64, 26_i64, 27_i64, 28_i64, 29_i64, 30_i64, 31_i64,
				32_i64, 33_i64, 34_i64, 35_i64, 36_i64, 37_i64, 38_i64, 39_i64, 40_i64, 41_i64,
				42_i64, 43_i64, 44_i64, 45_i64, 46_i64, 47_i64, 48_i64, 49_i64, 50_i64, 51_i64,
				52_i64, 53_i64, 54_i64, 55_i64, 56_i64, 57_i64, 58_i64, 59_i64, 60_i64, 61_i64,
				62_i64, 63_i64, 64_i64, 65_i64, 66_i64, 67_i64, 68_i64, 69_i64, 70_i64, 71_i64,
				72_i64, 73_i64, 74_i64, 75_i64, 76_i64, 77_i64, 78_i64, 79_i64, 80_i64, 81_i64,
				82_i64, 83_i64, 84_i64, 85_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AgeRatingContentCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///AgeRatingContentDescription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "description",
	///    "id"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/AgeRatingContentCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct AgeRatingContentDescription {
		pub category: AgeRatingContentCategory,
		pub checksum: uuid::Uuid,
		pub description: String,
		pub id: i32,
	}

	impl From<&AgeRatingContentDescription> for AgeRatingContentDescription {
		fn from(value: &AgeRatingContentDescription) -> Self {
			value.clone()
		}
	}

	///AgeRatingEnum
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7,
	///    8,
	///    9,
	///    10,
	///    11,
	///    12,
	///    13,
	///    14,
	///    15,
	///    16,
	///    17,
	///    18,
	///    19,
	///    20,
	///    21,
	///    22,
	///    23,
	///    24,
	///    25,
	///    26,
	///    27,
	///    28,
	///    29,
	///    30,
	///    31,
	///    32,
	///    33,
	///    34,
	///    35,
	///    36,
	///    37,
	///    38,
	///    39
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct AgeRatingEnum(i64);
	impl ::std::ops::Deref for AgeRatingEnum {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<AgeRatingEnum> for i64 {
		fn from(value: AgeRatingEnum) -> Self {
			value.0
		}
	}

	impl From<&AgeRatingEnum> for AgeRatingEnum {
		fn from(value: &AgeRatingEnum) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for AgeRatingEnum {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64, 11_i64,
				12_i64, 13_i64, 14_i64, 15_i64, 16_i64, 17_i64, 18_i64, 19_i64, 20_i64, 21_i64,
				22_i64, 23_i64, 24_i64, 25_i64, 26_i64, 27_i64, 28_i64, 29_i64, 30_i64, 31_i64,
				32_i64, 33_i64, 34_i64, 35_i64, 36_i64, 37_i64, 38_i64, 39_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AgeRatingEnum {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///AlternativeName
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "game",
	///    "id",
	///    "name"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "comment": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct AlternativeName {
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub comment: Option<String>,
		pub game: i32,
		pub id: i32,
		pub name: String,
	}

	impl From<&AlternativeName> for AlternativeName {
		fn from(value: &AlternativeName) -> Self {
			value.clone()
		}
	}

	///Artwork
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "game",
	///    "height",
	///    "id",
	///    "image_id",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Artwork {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub game: i32,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub url: String,
		pub width: i32,
	}

	impl From<&Artwork> for Artwork {
		fn from(value: &Artwork) -> Self {
			value.clone()
		}
	}

	///Reason why a game was automatically matched.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Reason why a game was automatically matched.",
	///  "type": "string",
	///  "enum": [
	///    "AlternativeName",
	///    "DirectName",
	///    "ViaChild",
	///    "ViaParent"
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(
		:: serde :: Deserialize,
		:: serde :: Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
	)]
	pub enum AutomaticMatchReason {
		AlternativeName,
		DirectName,
		ViaChild,
		ViaParent,
	}

	impl From<&AutomaticMatchReason> for AutomaticMatchReason {
		fn from(value: &AutomaticMatchReason) -> Self {
			value.clone()
		}
	}

	impl ToString for AutomaticMatchReason {
		fn to_string(&self) -> String {
			match *self {
				Self::AlternativeName => "AlternativeName".to_string(),
				Self::DirectName => "DirectName".to_string(),
				Self::ViaChild => "ViaChild".to_string(),
				Self::ViaParent => "ViaParent".to_string(),
			}
		}
	}

	impl std::str::FromStr for AutomaticMatchReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AlternativeName" => Ok(Self::AlternativeName),
				"DirectName" => Ok(Self::DirectName),
				"ViaChild" => Ok(Self::ViaChild),
				"ViaParent" => Ok(Self::ViaParent),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AutomaticMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AutomaticMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AutomaticMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	///Character
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "akas",
	///    "checksum",
	///    "country_name",
	///    "created_at",
	///    "description",
	///    "games",
	///    "gender",
	///    "id",
	///    "mug_shot",
	///    "name",
	///    "slug",
	///    "species",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "akas": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      }
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "country_name": {
	///      "type": "string"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "games": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "gender": {
	///      "$ref": "#/components/schemas/CharacterGender"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "mug_shot": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "species": {
	///      "$ref": "#/components/schemas/CharacterSpecies"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Character {
		pub akas: Vec<String>,
		pub checksum: uuid::Uuid,
		pub country_name: String,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub games: Vec<i32>,
		pub gender: CharacterGender,
		pub id: i32,
		pub mug_shot: i32,
		pub name: String,
		pub slug: String,
		pub species: CharacterSpecies,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&Character> for Character {
		fn from(value: &Character) -> Self {
			value.clone()
		}
	}

	///CharacterGender
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct CharacterGender(i64);
	impl ::std::ops::Deref for CharacterGender {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<CharacterGender> for i64 {
		fn from(value: CharacterGender) -> Self {
			value.0
		}
	}

	impl From<&CharacterGender> for CharacterGender {
		fn from(value: &CharacterGender) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for CharacterGender {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64, 2_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CharacterGender {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///CharacterSpecies
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct CharacterSpecies(i64);
	impl ::std::ops::Deref for CharacterSpecies {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<CharacterSpecies> for i64 {
		fn from(value: CharacterSpecies) -> Self {
			value.0
		}
	}

	impl From<&CharacterSpecies> for CharacterSpecies {
		fn from(value: &CharacterSpecies) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for CharacterSpecies {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![1_i64, 2_i64, 3_i64, 4_i64, 5_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CharacterSpecies {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///Collection
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "games",
	///    "id",
	///    "name",
	///    "slug",
	///    "type",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "as_child_relations": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "as_parent_relations": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "games": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Collection {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub as_child_relations: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub as_parent_relations: Option<Vec<i32>>,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub games: Vec<i32>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		#[serde(rename = "type")]
		pub type_: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&Collection> for Collection {
		fn from(value: &Collection) -> Self {
			value.clone()
		}
	}

	///CollectionMembership
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "collection",
	///    "created_at",
	///    "game",
	///    "id",
	///    "type",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "collection": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CollectionMembership {
		pub checksum: uuid::Uuid,
		pub collection: i32,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub game: i32,
		pub id: i32,
		#[serde(rename = "type")]
		pub type_: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&CollectionMembership> for CollectionMembership {
		fn from(value: &CollectionMembership) -> Self {
			value.clone()
		}
	}

	///CollectionMembershipType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "allowed_collection_type",
	///    "checksum",
	///    "created_at",
	///    "description",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "allowed_collection_type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CollectionMembershipType {
		pub allowed_collection_type: i32,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub id: i32,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&CollectionMembershipType> for CollectionMembershipType {
		fn from(value: &CollectionMembershipType) -> Self {
			value.clone()
		}
	}

	///CollectionRelation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "child_collection",
	///    "created_at",
	///    "id",
	///    "parent_collection",
	///    "type",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "child_collection": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "parent_collection": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CollectionRelation {
		pub checksum: uuid::Uuid,
		pub child_collection: i32,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub parent_collection: i32,
		#[serde(rename = "type")]
		pub type_: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&CollectionRelation> for CollectionRelation {
		fn from(value: &CollectionRelation) -> Self {
			value.clone()
		}
	}

	///CollectionRelationType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "allowed_child_type",
	///    "allowed_parent_type",
	///    "checksum",
	///    "created_at",
	///    "description",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "allowed_child_type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "allowed_parent_type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CollectionRelationType {
		pub allowed_child_type: i32,
		pub allowed_parent_type: i32,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub id: i32,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&CollectionRelationType> for CollectionRelationType {
		fn from(value: &CollectionRelationType) -> Self {
			value.clone()
		}
	}

	///CollectionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "description",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CollectionType {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub id: i32,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&CollectionType> for CollectionType {
		fn from(value: &CollectionType) -> Self {
			value.clone()
		}
	}

	///Company
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "change_date",
	///    "change_date_category",
	///    "changed_company_id",
	///    "checksum",
	///    "country",
	///    "created_at",
	///    "description",
	///    "developed",
	///    "id",
	///    "logo",
	///    "name",
	///    "parent",
	///    "published",
	///    "slug",
	///    "start_date",
	///    "start_date_category",
	///    "updated_at",
	///    "url",
	///    "websites"
	///  ],
	///  "properties": {
	///    "change_date": {
	///      "type": "integer",
	///      "format": "int64"
	///    },
	///    "change_date_category": {
	///      "$ref": "#/components/schemas/CompanyChangeDateCategory"
	///    },
	///    "changed_company_id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "country": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "developed": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "logo": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "parent": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "published": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "start_date": {
	///      "type": "integer",
	///      "format": "int64"
	///    },
	///    "start_date_category": {
	///      "$ref": "#/components/schemas/CompanyStartDateCategory"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "websites": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Company {
		pub change_date: i64,
		pub change_date_category: CompanyChangeDateCategory,
		pub changed_company_id: i32,
		pub checksum: uuid::Uuid,
		pub country: i32,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub developed: Vec<i32>,
		pub id: i32,
		pub logo: i32,
		pub name: String,
		pub parent: i32,
		pub published: Vec<i32>,
		pub slug: String,
		pub start_date: i64,
		pub start_date_category: CompanyStartDateCategory,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
		pub websites: Vec<i32>,
	}

	impl From<&Company> for Company {
		fn from(value: &Company) -> Self {
			value.clone()
		}
	}

	///CompanyChangeDateCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct CompanyChangeDateCategory(i64);
	impl ::std::ops::Deref for CompanyChangeDateCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<CompanyChangeDateCategory> for i64 {
		fn from(value: CompanyChangeDateCategory) -> Self {
			value.0
		}
	}

	impl From<&CompanyChangeDateCategory> for CompanyChangeDateCategory {
		fn from(value: &CompanyChangeDateCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for CompanyChangeDateCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CompanyChangeDateCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///CompanyLogo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "height",
	///    "id",
	///    "image_id",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CompanyLogo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub url: String,
		pub width: i32,
	}

	impl From<&CompanyLogo> for CompanyLogo {
		fn from(value: &CompanyLogo) -> Self {
			value.clone()
		}
	}

	///Response for a company including external metadata.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Response for a company including external metadata.",
	///  "type": "object",
	///  "required": [
	///    "id",
	///    "name"
	///  ],
	///  "properties": {
	///    "externalMetadata": {
	///      "description": "External metadata for the company.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalMetadata"
	///      }
	///    },
	///    "id": {
	///      "description": "The ID of the company.",
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "name": {
	///      "description": "The name of the company.",
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CompanyResponse {
		///External metadata for the company.
		#[serde(
			rename = "externalMetadata",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_metadata: Vec<ExternalMetadata>,
		///The ID of the company.
		pub id: uuid::Uuid,
		///The name of the company.
		pub name: String,
	}

	impl From<&CompanyResponse> for CompanyResponse {
		fn from(value: &CompanyResponse) -> Self {
			value.clone()
		}
	}

	///CompanyStartDateCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct CompanyStartDateCategory(i64);
	impl ::std::ops::Deref for CompanyStartDateCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<CompanyStartDateCategory> for i64 {
		fn from(value: CompanyStartDateCategory) -> Self {
			value.0
		}
	}

	impl From<&CompanyStartDateCategory> for CompanyStartDateCategory {
		fn from(value: &CompanyStartDateCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for CompanyStartDateCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CompanyStartDateCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///CompanyWebsite
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "id",
	///    "url"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/CompanyWebsiteCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "trusted": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct CompanyWebsite {
		pub category: CompanyWebsiteCategory,
		pub checksum: uuid::Uuid,
		pub id: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub trusted: Option<bool>,
		pub url: String,
	}

	impl From<&CompanyWebsite> for CompanyWebsite {
		fn from(value: &CompanyWebsite) -> Self {
			value.clone()
		}
	}

	///CompanyWebsiteCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    8,
	///    9,
	///    10,
	///    11,
	///    12,
	///    13,
	///    14,
	///    15,
	///    16,
	///    17,
	///    18
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct CompanyWebsiteCategory(i64);
	impl ::std::ops::Deref for CompanyWebsiteCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<CompanyWebsiteCategory> for i64 {
		fn from(value: CompanyWebsiteCategory) -> Self {
			value.0
		}
	}

	impl From<&CompanyWebsiteCategory> for CompanyWebsiteCategory {
		fn from(value: &CompanyWebsiteCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for CompanyWebsiteCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 8_i64, 9_i64, 10_i64, 11_i64, 12_i64,
				13_i64, 14_i64, 15_i64, 16_i64, 17_i64, 18_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CompanyWebsiteCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///Cover
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "game",
	///    "height",
	///    "id",
	///    "image_id",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "game_localization": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Cover {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub game: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub game_localization: Option<i32>,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub url: String,
		pub width: i32,
	}

	impl From<&Cover> for Cover {
		fn from(value: &Cover) -> Self {
			value.clone()
		}
	}

	///Event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "description",
	///    "end_time",
	///    "id",
	///    "name",
	///    "slug",
	///    "start_time",
	///    "time_zone",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "end_time": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "event_logo": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "event_networks": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "games": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "live_stream_url": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "start_time": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "time_zone": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "videos": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Event {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub end_time: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub event_logo: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub event_networks: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub games: Option<Vec<i32>>,
		pub id: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub live_stream_url: Option<String>,
		pub name: String,
		pub slug: String,
		pub start_time: chrono::DateTime<chrono::offset::Utc>,
		pub time_zone: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub videos: Option<Vec<i32>>,
	}

	impl From<&Event> for Event {
		fn from(value: &Event) -> Self {
			value.clone()
		}
	}

	///EventLogo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "event",
	///    "height",
	///    "id",
	///    "image_id",
	///    "updated_at",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "event": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct EventLogo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub event: i32,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
		pub width: i32,
	}

	impl From<&EventLogo> for EventLogo {
		fn from(value: &EventLogo) -> Self {
			value.clone()
		}
	}

	///EventNetwork
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "event",
	///    "id",
	///    "network_type",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "event": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "network_type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct EventNetwork {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub event: i32,
		pub id: i32,
		pub network_type: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&EventNetwork> for EventNetwork {
		fn from(value: &EventNetwork) -> Self {
			value.clone()
		}
	}

	///ExternalGame
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "created_at",
	///    "game",
	///    "id",
	///    "media",
	///    "name",
	///    "uid",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/ExternalGameCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "countries": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "media": {
	///      "$ref": "#/components/schemas/ExternalGameMedia"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "platform": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "uid": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "year": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct ExternalGame {
		pub category: ExternalGameCategory,
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub countries: Option<Vec<i32>>,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub game: i32,
		pub id: i32,
		pub media: ExternalGameMedia,
		pub name: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platform: Option<i32>,
		pub uid: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub year: Option<i32>,
	}

	impl From<&ExternalGame> for ExternalGame {
		fn from(value: &ExternalGame) -> Self {
			value.clone()
		}
	}

	///ExternalGameCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    5,
	///    10,
	///    11,
	///    13,
	///    14,
	///    15,
	///    20,
	///    22,
	///    23,
	///    26,
	///    28,
	///    29,
	///    30,
	///    31,
	///    32,
	///    36,
	///    37,
	///    54,
	///    55
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct ExternalGameCategory(i64);
	impl ::std::ops::Deref for ExternalGameCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ExternalGameCategory> for i64 {
		fn from(value: ExternalGameCategory) -> Self {
			value.0
		}
	}

	impl From<&ExternalGameCategory> for ExternalGameCategory {
		fn from(value: &ExternalGameCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for ExternalGameCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 5_i64, 10_i64, 11_i64, 13_i64, 14_i64, 15_i64, 20_i64, 22_i64, 23_i64,
				26_i64, 28_i64, 29_i64, 30_i64, 31_i64, 32_i64, 36_i64, 37_i64, 54_i64, 55_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ExternalGameCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///ExternalGameMedia
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct ExternalGameMedia(i64);
	impl ::std::ops::Deref for ExternalGameMedia {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ExternalGameMedia> for i64 {
		fn from(value: ExternalGameMedia) -> Self {
			value.0
		}
	}

	impl From<&ExternalGameMedia> for ExternalGameMedia {
		fn from(value: &ExternalGameMedia) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for ExternalGameMedia {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![1_i64, 2_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ExternalGameMedia {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///External metadata for a game/platform/company.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "External metadata for a game/platform/company.",
	///  "type": "object",
	///  "required": [
	///    "matchType",
	///    "providerName"
	///  ],
	///  "properties": {
	///    "automaticMatchReason": {
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/AutomaticMatchReason"
	///        }
	///      ]
	///    },
	///    "comment": {
	///      "description": "Optional Comment about the match.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "failedMatchReason": {
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/FailedMatchReason"
	///        }
	///      ]
	///    },
	///    "manualMatchType": {
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/ManualMatchMode"
	///        }
	///      ]
	///    },
	///    "matchType": {
	///      "$ref": "#/components/schemas/MatchType"
	///    },
	///    "providerId": {
	///      "description": "The ID of the game for this provider.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "providerName": {
	///      "$ref": "#/components/schemas/MetadataProvider"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct ExternalMetadata {
		#[serde(
			rename = "automaticMatchReason",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub automatic_match_reason: Option<AutomaticMatchReason>,
		///Optional Comment about the match.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub comment: Option<String>,
		#[serde(
			rename = "failedMatchReason",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failed_match_reason: Option<FailedMatchReason>,
		#[serde(
			rename = "manualMatchType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub manual_match_type: Option<ManualMatchMode>,
		#[serde(rename = "matchType")]
		pub match_type: MatchType,
		///The ID of the game for this provider.
		#[serde(
			rename = "providerId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub provider_id: Option<String>,
		#[serde(rename = "providerName")]
		pub provider_name: MetadataProvider,
	}

	impl From<&ExternalMetadata> for ExternalMetadata {
		fn from(value: &ExternalMetadata) -> Self {
			value.clone()
		}
	}

	///Reason why an automatic match failed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Reason why an automatic match failed.",
	///  "type": "string",
	///  "enum": [
	///    "NoDirectMatch",
	///    "TooManyMatches"
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(
		:: serde :: Deserialize,
		:: serde :: Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
	)]
	pub enum FailedMatchReason {
		NoDirectMatch,
		TooManyMatches,
	}

	impl From<&FailedMatchReason> for FailedMatchReason {
		fn from(value: &FailedMatchReason) -> Self {
			value.clone()
		}
	}

	impl ToString for FailedMatchReason {
		fn to_string(&self) -> String {
			match *self {
				Self::NoDirectMatch => "NoDirectMatch".to_string(),
				Self::TooManyMatches => "TooManyMatches".to_string(),
			}
		}
	}

	impl std::str::FromStr for FailedMatchReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NoDirectMatch" => Ok(Self::NoDirectMatch),
				"TooManyMatches" => Ok(Self::TooManyMatches),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FailedMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FailedMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FailedMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	///Franchise
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "games": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Franchise {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub games: Option<Vec<i32>>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&Franchise> for Franchise {
		fn from(value: &Franchise) -> Self {
			value.clone()
		}
	}

	///Game
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "age_ratings": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "aggregated_rating": {
	///      "type": [
	///        "number",
	///        "null"
	///      ],
	///      "format": "double"
	///    },
	///    "aggregated_rating_count": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "alternative_names": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "artworks": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "bundles": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "category": {
	///      "$ref": "#/components/schemas/GameCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "collection": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "collections": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "cover": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "dlcs": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "expanded_games": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "expansions": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "external_games": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "first_release_date": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int64"
	///    },
	///    "follows": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "forks": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "franchise": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "franchises": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "game_engines": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "game_localizations": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "game_modes": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "genres": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "hypes": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "involved_companies": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "keywords": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "language_supports": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "multiplayer_modes": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "parent_game": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "platforms": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "player_perspectives": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "ports": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "rating": {
	///      "type": [
	///        "number",
	///        "null"
	///      ],
	///      "format": "double"
	///    },
	///    "rating_count": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "release_dates": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "remakes": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "remasters": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "screenshots": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "similar_games": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "standalone_expansions": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "status": {
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/GameStatus"
	///        }
	///      ]
	///    },
	///    "storyline": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "summary": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "tags": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "themes": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "total_rating": {
	///      "type": [
	///        "number",
	///        "null"
	///      ],
	///      "format": "double"
	///    },
	///    "total_rating_count": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "version_parent": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "version_title": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "videos": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "websites": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Game {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub age_ratings: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub aggregated_rating: Option<f64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub aggregated_rating_count: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alternative_names: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub artworks: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub bundles: Option<Vec<i32>>,
		pub category: GameCategory,
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub collection: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub collections: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cover: Option<i32>,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dlcs: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expanded_games: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expansions: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub external_games: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub first_release_date: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub follows: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub forks: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub franchise: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub franchises: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub game_engines: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub game_localizations: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub game_modes: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub genres: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub hypes: Option<i32>,
		pub id: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub involved_companies: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub keywords: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub language_supports: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub multiplayer_modes: Option<Vec<i32>>,
		pub name: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub parent_game: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platforms: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub player_perspectives: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ports: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rating: Option<f64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rating_count: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub release_dates: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub remakes: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub remasters: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub screenshots: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub similar_games: Option<Vec<i32>>,
		pub slug: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub standalone_expansions: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<GameStatus>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub storyline: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub summary: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tags: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub themes: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub total_rating: Option<f64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub total_rating_count: Option<i32>,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub version_parent: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub version_title: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub videos: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub websites: Option<Vec<i32>>,
	}

	impl From<&Game> for Game {
		fn from(value: &Game) -> Self {
			value.clone()
		}
	}

	///GameCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7,
	///    8,
	///    9,
	///    10,
	///    11,
	///    12,
	///    13,
	///    14
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct GameCategory(i64);
	impl ::std::ops::Deref for GameCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<GameCategory> for i64 {
		fn from(value: GameCategory) -> Self {
			value.0
		}
	}

	impl From<&GameCategory> for GameCategory {
		fn from(value: &GameCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for GameCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64,
				11_i64, 12_i64, 13_i64, 14_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GameCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///GameEngine
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "companies": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "logo": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "platforms": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameEngine {
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub companies: Option<Vec<i32>>,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub description: Option<String>,
		pub id: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub logo: Option<i32>,
		pub name: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platforms: Option<Vec<i32>>,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&GameEngine> for GameEngine {
		fn from(value: &GameEngine) -> Self {
			value.clone()
		}
	}

	///GameEngineLogo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "height",
	///    "id",
	///    "image_id",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameEngineLogo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub url: String,
		pub width: i32,
	}

	impl From<&GameEngineLogo> for GameEngineLogo {
		fn from(value: &GameEngineLogo) -> Self {
			value.clone()
		}
	}

	///GameLocalization
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "game",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "cover": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "region": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameLocalization {
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cover: Option<i32>,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub game: i32,
		pub id: i32,
		pub name: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub region: Option<i32>,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&GameLocalization> for GameLocalization {
		fn from(value: &GameLocalization) -> Self {
			value.clone()
		}
	}

	///Result of a game match.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Result of a game match.",
	///  "type": "object",
	///  "required": [
	///    "gameMatchType"
	///  ],
	///  "properties": {
	///    "externalMetadata": {
	///      "description": "External metadata for the matched game.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalMetadata"
	///      }
	///    },
	///    "gameMatchType": {
	///      "$ref": "#/components/schemas/GameMatchType"
	///    },
	///    "id": {
	///      "description": "If a match was found, the ID of the matched game.",
	///      "type": [
	///        "string",
	///        "null"
	///      ],
	///      "format": "uuid"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameMatchResult {
		///External metadata for the matched game.
		#[serde(
			rename = "externalMetadata",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_metadata: Vec<ExternalMetadata>,
		#[serde(rename = "gameMatchType")]
		pub game_match_type: GameMatchType,
		///If a match was found, the ID of the matched game.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub id: Option<uuid::Uuid>,
	}

	impl From<&GameMatchResult> for GameMatchResult {
		fn from(value: &GameMatchResult) -> Self {
			value.clone()
		}
	}

	///Type of match for this game.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Type of match for this game.",
	///  "type": "string",
	///  "enum": [
	///    "SHA256",
	///    "SHA1",
	///    "MD5",
	///    "FileNameAndSize",
	///    "NoMatch"
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(
		:: serde :: Deserialize,
		:: serde :: Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
	)]
	pub enum GameMatchType {
		#[serde(rename = "SHA256")]
		Sha256,
		#[serde(rename = "SHA1")]
		Sha1,
		#[serde(rename = "MD5")]
		Md5,
		FileNameAndSize,
		NoMatch,
	}

	impl From<&GameMatchType> for GameMatchType {
		fn from(value: &GameMatchType) -> Self {
			value.clone()
		}
	}

	impl ToString for GameMatchType {
		fn to_string(&self) -> String {
			match *self {
				Self::Sha256 => "SHA256".to_string(),
				Self::Sha1 => "SHA1".to_string(),
				Self::Md5 => "MD5".to_string(),
				Self::FileNameAndSize => "FileNameAndSize".to_string(),
				Self::NoMatch => "NoMatch".to_string(),
			}
		}
	}

	impl std::str::FromStr for GameMatchType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SHA256" => Ok(Self::Sha256),
				"SHA1" => Ok(Self::Sha1),
				"MD5" => Ok(Self::Md5),
				"FileNameAndSize" => Ok(Self::FileNameAndSize),
				"NoMatch" => Ok(Self::NoMatch),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for GameMatchType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for GameMatchType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for GameMatchType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	///GameMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameMode {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&GameMode> for GameMode {
		fn from(value: &GameMode) -> Self {
			value.clone()
		}
	}

	///GameStatus
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7,
	///    8
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct GameStatus(i64);
	impl ::std::ops::Deref for GameStatus {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<GameStatus> for i64 {
		fn from(value: GameStatus) -> Self {
			value.0
		}
	}

	impl From<&GameStatus> for GameStatus {
		fn from(value: &GameStatus) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for GameStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GameStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///GameVersion
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "game",
	///    "id",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "features": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "games": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameVersion {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub features: Option<Vec<i32>>,
		pub game: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub games: Option<Vec<i32>>,
		pub id: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&GameVersion> for GameVersion {
		fn from(value: &GameVersion) -> Self {
			value.clone()
		}
	}

	///GameVersionFeature
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "description",
	///    "id",
	///    "position",
	///    "title"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/GameVersionFeatureCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "position": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "title": {
	///      "type": "string"
	///    },
	///    "values": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameVersionFeature {
		pub category: GameVersionFeatureCategory,
		pub checksum: uuid::Uuid,
		pub description: String,
		pub id: i32,
		pub position: i32,
		pub title: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub values: Option<Vec<i32>>,
	}

	impl From<&GameVersionFeature> for GameVersionFeature {
		fn from(value: &GameVersionFeature) -> Self {
			value.clone()
		}
	}

	///GameVersionFeatureCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct GameVersionFeatureCategory(i64);
	impl ::std::ops::Deref for GameVersionFeatureCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<GameVersionFeatureCategory> for i64 {
		fn from(value: GameVersionFeatureCategory) -> Self {
			value.0
		}
	}

	impl From<&GameVersionFeatureCategory> for GameVersionFeatureCategory {
		fn from(value: &GameVersionFeatureCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for GameVersionFeatureCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GameVersionFeatureCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///GameVersionFeatureValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "game",
	///    "game_feature",
	///    "id",
	///    "included_feature"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "game_feature": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "included_feature": {
	///      "$ref": "#/components/schemas/GameVersionFeatureValueEnum"
	///    },
	///    "note": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameVersionFeatureValue {
		pub checksum: uuid::Uuid,
		pub game: i32,
		pub game_feature: i32,
		pub id: i32,
		pub included_feature: GameVersionFeatureValueEnum,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub note: Option<String>,
	}

	impl From<&GameVersionFeatureValue> for GameVersionFeatureValue {
		fn from(value: &GameVersionFeatureValue) -> Self {
			value.clone()
		}
	}

	///GameVersionFeatureValueEnum
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct GameVersionFeatureValueEnum(i64);
	impl ::std::ops::Deref for GameVersionFeatureValueEnum {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<GameVersionFeatureValueEnum> for i64 {
		fn from(value: GameVersionFeatureValueEnum) -> Self {
			value.0
		}
	}

	impl From<&GameVersionFeatureValueEnum> for GameVersionFeatureValueEnum {
		fn from(value: &GameVersionFeatureValueEnum) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for GameVersionFeatureValueEnum {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64, 2_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GameVersionFeatureValueEnum {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///GameVideo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "game",
	///    "id",
	///    "name",
	///    "video_id"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "video_id": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct GameVideo {
		pub checksum: uuid::Uuid,
		pub game: i32,
		pub id: i32,
		pub name: String,
		pub video_id: String,
	}

	impl From<&GameVideo> for GameVideo {
		fn from(value: &GameVideo) -> Self {
			value.clone()
		}
	}

	///Genre
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Genre {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&Genre> for Genre {
		fn from(value: &Genre) -> Self {
			value.clone()
		}
	}

	///InvolvedCompany
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "company",
	///    "created_at",
	///    "developer",
	///    "game",
	///    "id",
	///    "porting",
	///    "publisher",
	///    "supporting",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "company": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "developer": {
	///      "type": "boolean"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "porting": {
	///      "type": "boolean"
	///    },
	///    "publisher": {
	///      "type": "boolean"
	///    },
	///    "supporting": {
	///      "type": "boolean"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct InvolvedCompany {
		pub checksum: uuid::Uuid,
		pub company: i32,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub developer: bool,
		pub game: i32,
		pub id: i32,
		pub porting: bool,
		pub publisher: bool,
		pub supporting: bool,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&InvolvedCompany> for InvolvedCompany {
		fn from(value: &InvolvedCompany) -> Self {
			value.clone()
		}
	}

	///Keyword
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Keyword {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&Keyword> for Keyword {
		fn from(value: &Keyword) -> Self {
			value.clone()
		}
	}

	///Language
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "locale",
	///    "name",
	///    "native_name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "locale": {
	///      "type": "string"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "native_name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Language {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub locale: String,
		pub name: String,
		pub native_name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&Language> for Language {
		fn from(value: &Language) -> Self {
			value.clone()
		}
	}

	///LanguageSupport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "game",
	///    "id",
	///    "language",
	///    "language_support_type",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "language": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "language_support_type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct LanguageSupport {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub game: i32,
		pub id: i32,
		pub language: i32,
		pub language_support_type: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&LanguageSupport> for LanguageSupport {
		fn from(value: &LanguageSupport) -> Self {
			value.clone()
		}
	}

	///LanguageSupportType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct LanguageSupportType {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&LanguageSupportType> for LanguageSupportType {
		fn from(value: &LanguageSupportType) -> Self {
			value.clone()
		}
	}

	///How a game was manually matched.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "How a game was manually matched.",
	///  "type": "string",
	///  "enum": [
	///    "Admin",
	///    "Community",
	///    "Trusted"
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(
		:: serde :: Deserialize,
		:: serde :: Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
	)]
	pub enum ManualMatchMode {
		Admin,
		Community,
		Trusted,
	}

	impl From<&ManualMatchMode> for ManualMatchMode {
		fn from(value: &ManualMatchMode) -> Self {
			value.clone()
		}
	}

	impl ToString for ManualMatchMode {
		fn to_string(&self) -> String {
			match *self {
				Self::Admin => "Admin".to_string(),
				Self::Community => "Community".to_string(),
				Self::Trusted => "Trusted".to_string(),
			}
		}
	}

	impl std::str::FromStr for ManualMatchMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"Admin" => Ok(Self::Admin),
				"Community" => Ok(Self::Community),
				"Trusted" => Ok(Self::Trusted),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ManualMatchMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ManualMatchMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ManualMatchMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	///Match types for a game
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Match types for a game",
	///  "type": "string",
	///  "enum": [
	///    "Automatic",
	///    "Failed",
	///    "Manual",
	///    "None"
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(
		:: serde :: Deserialize,
		:: serde :: Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
	)]
	pub enum MatchType {
		Automatic,
		Failed,
		Manual,
		None,
	}

	impl From<&MatchType> for MatchType {
		fn from(value: &MatchType) -> Self {
			value.clone()
		}
	}

	impl ToString for MatchType {
		fn to_string(&self) -> String {
			match *self {
				Self::Automatic => "Automatic".to_string(),
				Self::Failed => "Failed".to_string(),
				Self::Manual => "Manual".to_string(),
				Self::None => "None".to_string(),
			}
		}
	}

	impl std::str::FromStr for MatchType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"Automatic" => Ok(Self::Automatic),
				"Failed" => Ok(Self::Failed),
				"Manual" => Ok(Self::Manual),
				"None" => Ok(Self::None),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MatchType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MatchType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MatchType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	///Metadata provider for game/platform/company.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Metadata provider for game/platform/company.",
	///  "type": "string",
	///  "enum": [
	///    "IGDB"
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(
		:: serde :: Deserialize,
		:: serde :: Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
	)]
	pub enum MetadataProvider {
		#[serde(rename = "IGDB")]
		Igdb,
	}

	impl From<&MetadataProvider> for MetadataProvider {
		fn from(value: &MetadataProvider) -> Self {
			value.clone()
		}
	}

	impl ToString for MetadataProvider {
		fn to_string(&self) -> String {
			match *self {
				Self::Igdb => "IGDB".to_string(),
			}
		}
	}

	impl std::str::FromStr for MetadataProvider {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IGDB" => Ok(Self::Igdb),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MetadataProvider {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MetadataProvider {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MetadataProvider {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	///MultiplayerMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "campaigncoop",
	///    "checksum",
	///    "dropin",
	///    "game",
	///    "id",
	///    "lancoop",
	///    "offlinecoop",
	///    "onlinecoop",
	///    "platform",
	///    "splitscreen",
	///    "splitscreenonline"
	///  ],
	///  "properties": {
	///    "campaigncoop": {
	///      "type": "boolean"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "dropin": {
	///      "type": "boolean"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "lancoop": {
	///      "type": "boolean"
	///    },
	///    "offlinecoop": {
	///      "type": "boolean"
	///    },
	///    "offlinecoopmax": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "offlinemax": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "onlinecoop": {
	///      "type": "boolean"
	///    },
	///    "onlinecoopmax": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "onlinemax": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "platform": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "splitscreen": {
	///      "type": "boolean"
	///    },
	///    "splitscreenonline": {
	///      "type": "boolean"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct MultiplayerMode {
		pub campaigncoop: bool,
		pub checksum: uuid::Uuid,
		pub dropin: bool,
		pub game: i32,
		pub id: i32,
		pub lancoop: bool,
		pub offlinecoop: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub offlinecoopmax: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub offlinemax: Option<i32>,
		pub onlinecoop: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub onlinecoopmax: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub onlinemax: Option<i32>,
		pub platform: i32,
		pub splitscreen: bool,
		pub splitscreenonline: bool,
	}

	impl From<&MultiplayerMode> for MultiplayerMode {
		fn from(value: &MultiplayerMode) -> Self {
			value.clone()
		}
	}

	///NetworkType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "event_networks": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct NetworkType {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub event_networks: Option<Vec<i32>>,
		pub id: i32,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&NetworkType> for NetworkType {
		fn from(value: &NetworkType) -> Self {
			value.clone()
		}
	}

	///Platform
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "abbreviation": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "alternative_name": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "category": {
	///      "$ref": "#/components/schemas/PlatformCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "generation": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "platform_family": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "platform_logo": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "summary": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "versions": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "websites": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Platform {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub abbreviation: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alternative_name: Option<String>,
		pub category: PlatformCategory,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub generation: Option<i32>,
		pub id: i32,
		pub name: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platform_family: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platform_logo: Option<i32>,
		pub slug: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub summary: Option<String>,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub versions: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub websites: Option<Vec<i32>>,
	}

	impl From<&Platform> for Platform {
		fn from(value: &Platform) -> Self {
			value.clone()
		}
	}

	///PlatformCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct PlatformCategory(i64);
	impl ::std::ops::Deref for PlatformCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PlatformCategory> for i64 {
		fn from(value: PlatformCategory) -> Self {
			value.0
		}
	}

	impl From<&PlatformCategory> for PlatformCategory {
		fn from(value: &PlatformCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for PlatformCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PlatformCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///PlatformFamily
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "id",
	///    "name",
	///    "slug"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformFamily {
		pub checksum: uuid::Uuid,
		pub id: i32,
		pub name: String,
		pub slug: String,
	}

	impl From<&PlatformFamily> for PlatformFamily {
		fn from(value: &PlatformFamily) -> Self {
			value.clone()
		}
	}

	///PlatformLogo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "height",
	///    "id",
	///    "image_id",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformLogo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub url: String,
		pub width: i32,
	}

	impl From<&PlatformLogo> for PlatformLogo {
		fn from(value: &PlatformLogo) -> Self {
			value.clone()
		}
	}

	///Response for a platform including external metadata.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "description": "Response for a platform including external metadata.",
	///  "type": "object",
	///  "required": [
	///    "id",
	///    "name"
	///  ],
	///  "properties": {
	///    "companyId": {
	///      "description": "Optional ID of the company that made the
	/// platform.",
	///      "type": [
	///        "string",
	///        "null"
	///      ],
	///      "format": "uuid"
	///    },
	///    "companyName": {
	///      "description": "Optional name of the company that made the
	/// platform.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "externalMetadata": {
	///      "description": "External metadata for the platform.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalMetadata"
	///      }
	///    },
	///    "id": {
	///      "description": "The ID of the platform.",
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "name": {
	///      "description": "The name of the platform.",
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformResponse {
		///Optional ID of the company that made the platform.
		#[serde(rename = "companyId", default, skip_serializing_if = "Option::is_none")]
		pub company_id: Option<uuid::Uuid>,
		///Optional name of the company that made the platform.
		#[serde(
			rename = "companyName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub company_name: Option<String>,
		///External metadata for the platform.
		#[serde(
			rename = "externalMetadata",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_metadata: Vec<ExternalMetadata>,
		///The ID of the platform.
		pub id: uuid::Uuid,
		///The name of the platform.
		pub name: String,
	}

	impl From<&PlatformResponse> for PlatformResponse {
		fn from(value: &PlatformResponse) -> Self {
			value.clone()
		}
	}

	///PlatformVersion
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "id",
	///    "name",
	///    "slug",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "companies": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "connectivity": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "cpu": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "graphics": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "main_manufacturer": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "media": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "memory": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "os": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "output": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "platform_logo": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "format": "int32"
	///    },
	///    "platform_version_release_dates": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "type": "integer",
	///        "format": "int32"
	///      }
	///    },
	///    "resolutions": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "sound": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "storage": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "summary": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformVersion {
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub companies: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub connectivity: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cpu: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub graphics: Option<String>,
		pub id: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub main_manufacturer: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub media: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub memory: Option<String>,
		pub name: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub os: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub output: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platform_logo: Option<i32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub platform_version_release_dates: Option<Vec<i32>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub resolutions: Option<String>,
		pub slug: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub sound: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub storage: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub summary: Option<String>,
		pub url: String,
	}

	impl From<&PlatformVersion> for PlatformVersion {
		fn from(value: &PlatformVersion) -> Self {
			value.clone()
		}
	}

	///PlatformVersionCompany
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "company",
	///    "developer",
	///    "id",
	///    "manufacturer"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "comment": {
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    },
	///    "company": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "developer": {
	///      "type": "boolean"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "manufacturer": {
	///      "type": "boolean"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformVersionCompany {
		pub checksum: uuid::Uuid,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub comment: Option<String>,
		pub company: i32,
		pub developer: bool,
		pub id: i32,
		pub manufacturer: bool,
	}

	impl From<&PlatformVersionCompany> for PlatformVersionCompany {
		fn from(value: &PlatformVersionCompany) -> Self {
			value.clone()
		}
	}

	///PlatformVersionReleaseDate
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "created_at",
	///    "date",
	///    "human",
	///    "id",
	///    "m",
	///    "platform_version",
	///    "region",
	///    "updated_at",
	///    "y"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/PlatformVersionReleaseDateCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "date": {
	///      "type": "integer",
	///      "format": "int64"
	///    },
	///    "human": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "m": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "platform_version": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "region": {
	///      "$ref": "#/components/schemas/PlatformVersionReleaseDateRegion"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "y": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformVersionReleaseDate {
		pub category: PlatformVersionReleaseDateCategory,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub date: i64,
		pub human: String,
		pub id: i32,
		pub m: i32,
		pub platform_version: i32,
		pub region: PlatformVersionReleaseDateRegion,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub y: i32,
	}

	impl From<&PlatformVersionReleaseDate> for PlatformVersionReleaseDate {
		fn from(value: &PlatformVersionReleaseDate) -> Self {
			value.clone()
		}
	}

	///PlatformVersionReleaseDateCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct PlatformVersionReleaseDateCategory(i64);
	impl ::std::ops::Deref for PlatformVersionReleaseDateCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PlatformVersionReleaseDateCategory> for i64 {
		fn from(value: PlatformVersionReleaseDateCategory) -> Self {
			value.0
		}
	}

	impl From<&PlatformVersionReleaseDateCategory> for PlatformVersionReleaseDateCategory {
		fn from(value: &PlatformVersionReleaseDateCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for PlatformVersionReleaseDateCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PlatformVersionReleaseDateCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///PlatformVersionReleaseDateRegion
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7,
	///    8,
	///    9,
	///    10
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct PlatformVersionReleaseDateRegion(i64);
	impl ::std::ops::Deref for PlatformVersionReleaseDateRegion {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PlatformVersionReleaseDateRegion> for i64 {
		fn from(value: PlatformVersionReleaseDateRegion) -> Self {
			value.0
		}
	}

	impl From<&PlatformVersionReleaseDateRegion> for PlatformVersionReleaseDateRegion {
		fn from(value: &PlatformVersionReleaseDateRegion) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for PlatformVersionReleaseDateRegion {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PlatformVersionReleaseDateRegion {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///PlatformWebsite
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "id",
	///    "url"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/PlatformWebsiteCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "trusted": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlatformWebsite {
		pub category: PlatformWebsiteCategory,
		pub checksum: uuid::Uuid,
		pub id: i32,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub trusted: Option<bool>,
		pub url: String,
	}

	impl From<&PlatformWebsite> for PlatformWebsite {
		fn from(value: &PlatformWebsite) -> Self {
			value.clone()
		}
	}

	///PlatformWebsiteCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    8,
	///    9,
	///    10,
	///    11,
	///    12,
	///    13,
	///    14,
	///    15,
	///    16,
	///    17,
	///    18,
	///    19,
	///    20
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct PlatformWebsiteCategory(i64);
	impl ::std::ops::Deref for PlatformWebsiteCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PlatformWebsiteCategory> for i64 {
		fn from(value: PlatformWebsiteCategory) -> Self {
			value.0
		}
	}

	impl From<&PlatformWebsiteCategory> for PlatformWebsiteCategory {
		fn from(value: &PlatformWebsiteCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for PlatformWebsiteCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 8_i64, 9_i64, 10_i64, 11_i64, 12_i64,
				13_i64, 14_i64, 15_i64, 16_i64, 17_i64, 18_i64, 19_i64, 20_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PlatformWebsiteCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///PlayerPerspective
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PlayerPerspective {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&PlayerPerspective> for PlayerPerspective {
		fn from(value: &PlayerPerspective) -> Self {
			value.clone()
		}
	}

	///PopularityPrimitive
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "calculated_at",
	///    "checksum",
	///    "created_at",
	///    "game_id",
	///    "id",
	///    "popularity_source",
	///    "popularity_type",
	///    "updated_at",
	///    "value"
	///  ],
	///  "properties": {
	///    "calculated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "game_id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "popularity_source": {
	///      "$ref": "#/components/schemas/PopularitySource"
	///    },
	///    "popularity_type": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "value": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PopularityPrimitive {
		pub calculated_at: chrono::DateTime<chrono::offset::Utc>,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub game_id: i32,
		pub id: i32,
		pub popularity_source: PopularitySource,
		pub popularity_type: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub value: String,
	}

	impl From<&PopularityPrimitive> for PopularityPrimitive {
		fn from(value: &PopularityPrimitive) -> Self {
			value.clone()
		}
	}

	///PopularitySource
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    121
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct PopularitySource(i64);
	impl ::std::ops::Deref for PopularitySource {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PopularitySource> for i64 {
		fn from(value: PopularitySource) -> Self {
			value.0
		}
	}

	impl From<&PopularitySource> for PopularitySource {
		fn from(value: &PopularitySource) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for PopularitySource {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![121_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PopularitySource {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///PopularityType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "popularity_source",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "popularity_source": {
	///      "$ref": "#/components/schemas/PopularitySource"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct PopularityType {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub popularity_source: PopularitySource,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&PopularityType> for PopularityType {
		fn from(value: &PopularityType) -> Self {
			value.clone()
		}
	}

	///Region
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "identifier",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "category": {
	///      "type": "string"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "identifier": {
	///      "type": "string"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Region {
		pub category: String,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub identifier: String,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&Region> for Region {
		fn from(value: &Region) -> Self {
			value.clone()
		}
	}

	///ReleaseDate
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "category",
	///    "checksum",
	///    "created_at",
	///    "date",
	///    "game",
	///    "human",
	///    "id",
	///    "m",
	///    "platform",
	///    "region",
	///    "status",
	///    "updated_at",
	///    "y"
	///  ],
	///  "properties": {
	///    "category": {
	///      "$ref": "#/components/schemas/ReleaseDateCategory"
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "date": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "human": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "m": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "platform": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "region": {
	///      "$ref": "#/components/schemas/ReleaseDateRegion"
	///    },
	///    "status": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "y": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct ReleaseDate {
		pub category: ReleaseDateCategory,
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub date: chrono::DateTime<chrono::offset::Utc>,
		pub game: i32,
		pub human: String,
		pub id: i32,
		pub m: i32,
		pub platform: i32,
		pub region: ReleaseDateRegion,
		pub status: i32,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub y: i32,
	}

	impl From<&ReleaseDate> for ReleaseDate {
		fn from(value: &ReleaseDate) -> Self {
			value.clone()
		}
	}

	///ReleaseDateCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    0,
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct ReleaseDateCategory(i64);
	impl ::std::ops::Deref for ReleaseDateCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReleaseDateCategory> for i64 {
		fn from(value: ReleaseDateCategory) -> Self {
			value.0
		}
	}

	impl From<&ReleaseDateCategory> for ReleaseDateCategory {
		fn from(value: &ReleaseDateCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for ReleaseDateCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ReleaseDateCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///ReleaseDateRegion
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    7,
	///    8,
	///    9,
	///    10
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct ReleaseDateRegion(i64);
	impl ::std::ops::Deref for ReleaseDateRegion {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReleaseDateRegion> for i64 {
		fn from(value: ReleaseDateRegion) -> Self {
			value.0
		}
	}

	impl From<&ReleaseDateRegion> for ReleaseDateRegion {
		fn from(value: &ReleaseDateRegion) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for ReleaseDateRegion {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ReleaseDateRegion {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}

	///ReleaseDateStatus
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "description",
	///    "id",
	///    "name",
	///    "updated_at"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "description": {
	///      "type": "string"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct ReleaseDateStatus {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub description: String,
		pub id: i32,
		pub name: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
	}

	impl From<&ReleaseDateStatus> for ReleaseDateStatus {
		fn from(value: &ReleaseDateStatus) -> Self {
			value.clone()
		}
	}

	///Screenshot
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "game",
	///    "height",
	///    "id",
	///    "image_id",
	///    "url",
	///    "width"
	///  ],
	///  "properties": {
	///    "alpha_channel": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "animated": {
	///      "type": [
	///        "boolean",
	///        "null"
	///      ]
	///    },
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "game": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "height": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "image_id": {
	///      "type": "string"
	///    },
	///    "url": {
	///      "type": "string"
	///    },
	///    "width": {
	///      "type": "integer",
	///      "format": "int32"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Screenshot {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub alpha_channel: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub animated: Option<bool>,
		pub checksum: uuid::Uuid,
		pub game: i32,
		pub height: i32,
		pub id: i32,
		pub image_id: String,
		pub url: String,
		pub width: i32,
	}

	impl From<&Screenshot> for Screenshot {
		fn from(value: &Screenshot) -> Self {
			value.clone()
		}
	}

	///Theme
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "object",
	///  "required": [
	///    "checksum",
	///    "created_at",
	///    "id",
	///    "name",
	///    "slug",
	///    "updated_at",
	///    "url"
	///  ],
	///  "properties": {
	///    "checksum": {
	///      "type": "string",
	///      "format": "uuid"
	///    },
	///    "created_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "id": {
	///      "type": "integer",
	///      "format": "int32"
	///    },
	///    "name": {
	///      "type": "string"
	///    },
	///    "slug": {
	///      "type": "string"
	///    },
	///    "updated_at": {
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "url": {
	///      "type": "string"
	///    }
	///  }
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
	pub struct Theme {
		pub checksum: uuid::Uuid,
		pub created_at: chrono::DateTime<chrono::offset::Utc>,
		pub id: i32,
		pub name: String,
		pub slug: String,
		pub updated_at: chrono::DateTime<chrono::offset::Utc>,
		pub url: String,
	}

	impl From<&Theme> for Theme {
		fn from(value: &Theme) -> Self {
			value.clone()
		}
	}

	///WebsiteCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	///{
	///  "type": "integer",
	///  "enum": [
	///    1,
	///    2,
	///    3,
	///    4,
	///    5,
	///    6,
	///    8,
	///    9,
	///    10,
	///    11,
	///    12,
	///    13,
	///    14,
	///    15,
	///    16,
	///    17,
	///    18
	///  ]
	///}
	/// ```
	/// </details>
	#[derive(:: serde :: Serialize, Clone, Debug)]
	pub struct WebsiteCategory(i64);
	impl ::std::ops::Deref for WebsiteCategory {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<WebsiteCategory> for i64 {
		fn from(value: WebsiteCategory) -> Self {
			value.0
		}
	}

	impl From<&WebsiteCategory> for WebsiteCategory {
		fn from(value: &WebsiteCategory) -> Self {
			value.clone()
		}
	}

	impl std::convert::TryFrom<i64> for WebsiteCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
			if ![
				1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 8_i64, 9_i64, 10_i64, 11_i64, 12_i64,
				13_i64, 14_i64, 15_i64, 16_i64, 17_i64, 18_i64,
			]
			.contains(&value)
			{
				Err("invalid value".into())
			} else {
				Ok(Self(value))
			}
		}
	}

	impl<'de> ::serde::Deserialize<'de> for WebsiteCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			Self::try_from(<i64>::deserialize(deserializer)?)
				.map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
		}
	}
}

#[derive(Clone, Debug)]
///Client for api
///
///
///
///Version: 0.1.0
pub struct Client {
	pub(crate) baseurl: String,
	pub(crate) client: reqwest::Client,
}

impl Client {
	/// Create a new client.
	///
	/// `baseurl` is the base URL provided to the internal
	/// `reqwest::Client`, and should include a scheme and hostname,
	/// as well as port and a path stem if applicable.
	pub fn new(baseurl: &str) -> Self {
		#[cfg(not(target_arch = "wasm32"))]
		let client = {
			let dur = std::time::Duration::from_secs(15);
			reqwest::ClientBuilder::new()
				.connect_timeout(dur)
				.timeout(dur)
		};
		#[cfg(target_arch = "wasm32")]
		let client = reqwest::ClientBuilder::new();
		Self::new_with_client(baseurl, client.build().unwrap())
	}

	/// Construct a new client with an existing `reqwest::Client`,
	/// allowing more control over its configuration.
	///
	/// `baseurl` is the base URL provided to the internal
	/// `reqwest::Client`, and should include a scheme and hostname,
	/// as well as port and a path stem if applicable.
	pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
		Self {
			baseurl: baseurl.to_string(),
			client,
		}
	}

	/// Get the base URL to which requests are made.
	pub fn baseurl(&self) -> &String {
		&self.baseurl
	}

	/// Get the internal `reqwest::Client` used to make requests.
	pub fn client(&self) -> &reqwest::Client {
		&self.client
	}

	/// Get the version of this API.
	///
	/// This string is pulled directly from the source OpenAPI
	/// document and may be in any format the API selects.
	pub fn api_version(&self) -> &'static str {
		"0.1.0"
	}
}

#[allow(clippy::all)]
impl Client {
	///Returns all companies and its external metadata mappings
	///
	///Sends a `GET` request to `/api/companies`
	pub async fn get_all_companies<'a>(
		&'a self,
	) -> Result<ResponseValue<Vec<types::CompanyResponse>>, Error<()>> {
		let url = format!("{}/api/companies", self.baseurl,);
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Returns a company and its metadata mappings by id
	///
	///Sends a `GET` request to `/api/companies/{id}`
	pub async fn get_company_by_id<'a>(
		&'a self,
		id: &'a uuid::Uuid,
	) -> Result<ResponseValue<types::CompanyResponse>, Error<()>> {
		let url = format!(
			"{}/api/companies/{}",
			self.baseurl,
			encode_path(&id.to_string()),
		);
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Checks if the service is healthy
	///
	///Sends a `GET` request to `/api/health`
	pub async fn health<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
		let url = format!("{}/api/health", self.baseurl,);
		#[allow(unused_mut)]
		let mut request = self.client.get(url).build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => Ok(ResponseValue::empty(response)),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Identify a game by its file hashes or filename and size, returning the
	/// matched metadata ids, goes in order sha256, sha1, md5 and filename +
	/// size (from most accurate to least accurate)
	///
	///Sends a `GET` request to `/api/identify/ids`
	///
	///Arguments:
	/// - `file_name`: The file name of the game file.
	/// - `file_size`: The size of the game file in bytes.
	/// - `md5`: Optional MD5 hash of the game file.
	/// - `sha1`: Optional SHA1 hash of the game file.
	/// - `sha256`: Optional SHA256 hash of the game file.
	pub async fn identify<'a>(
		&'a self,
		file_name: &'a str,
		file_size: i64,
		md5: Option<&'a str>,
		sha1: Option<&'a str>,
		sha256: Option<&'a str>,
	) -> Result<ResponseValue<types::GameMatchResult>, Error<()>> {
		let url = format!("{}/api/identify/ids", self.baseurl,);
		let mut query = Vec::with_capacity(5usize);
		query.push(("fileName", file_name.to_string()));
		query.push(("fileSize", file_size.to_string()));
		if let Some(v) = &md5 {
			query.push(("md5", v.to_string()));
		}

		if let Some(v) = &sha1 {
			query.push(("sha1", v.to_string()));
		}

		if let Some(v) = &sha256 {
			query.push(("sha256", v.to_string()));
		}

		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for an Age Rating by Id
	///
	///Sends a `GET` request to `/api/igdb/age-rating`
	pub async fn get_age_rating_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::AgeRating>, Error<()>> {
		let url = format!("{}/api/igdb/age-rating", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Age Ratings by Ids
	///
	///Sends a `GET` request to `/api/igdb/age-ratings`
	pub async fn get_age_ratings_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::AgeRating>>, Error<()>> {
		let url = format!("{}/api/igdb/age-ratings", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for an Alternative Name by Id
	///
	///Sends a `GET` request to `/api/igdb/alternative-name`
	pub async fn get_alternative_name_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::AlternativeName>, Error<()>> {
		let url = format!("{}/api/igdb/alternative-name", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Alternative Names by Ids
	///
	///Sends a `GET` request to `/api/igdb/alternative-names`
	pub async fn get_alternative_names_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::AlternativeName>>, Error<()>> {
		let url = format!("{}/api/igdb/alternative-names", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for an Artwork by Id
	///
	///Sends a `GET` request to `/api/igdb/artwork`
	pub async fn get_artwork_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::Artwork>, Error<()>> {
		let url = format!("{}/api/igdb/artwork", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Artworks by Ids
	///
	///Sends a `GET` request to `/api/igdb/artworks`
	pub async fn get_artworks_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::Artwork>>, Error<()>> {
		let url = format!("{}/api/igdb/artworks", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for an Collection by Id
	///
	///Sends a `GET` request to `/api/igdb/collection`
	pub async fn get_collection_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::Collection>, Error<()>> {
		let url = format!("{}/api/igdb/collection", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Collections by Ids
	///
	///Sends a `GET` request to `/api/igdb/collections`
	pub async fn get_collections_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::Collection>>, Error<()>> {
		let url = format!("{}/api/igdb/collections", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for an Cover by Id
	///
	///Sends a `GET` request to `/api/igdb/cover`
	pub async fn get_cover_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::Cover>, Error<()>> {
		let url = format!("{}/api/igdb/cover", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Covers by Ids
	///
	///Sends a `GET` request to `/api/igdb/covers`
	pub async fn get_covers_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::Cover>>, Error<()>> {
		let url = format!("{}/api/igdb/covers", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for an External Game by Id
	///
	///Sends a `GET` request to `/api/igdb/external-game`
	pub async fn get_external_game_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::ExternalGame>, Error<()>> {
		let url = format!("{}/api/igdb/external-game", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for External Games by Ids
	///
	///Sends a `GET` request to `/api/igdb/external-games`
	pub async fn get_external_games_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::ExternalGame>>, Error<()>> {
		let url = format!("{}/api/igdb/external-games", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for a Franchise by Id
	///
	///Sends a `GET` request to `/api/igdb/franchise`
	pub async fn get_franchise_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::Franchise>, Error<()>> {
		let url = format!("{}/api/igdb/franchise", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Franchise by Ids
	///
	///Sends a `GET` request to `/api/igdb/franchises`
	pub async fn get_franchises_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::Franchise>>, Error<()>> {
		let url = format!("{}/api/igdb/franchises", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for a game by its Id
	///
	///Sends a `GET` request to `/api/igdb/game`
	pub async fn get_game_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::Game>, Error<()>> {
		let url = format!("{}/api/igdb/game", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Searches the IGDB API for games by its name
	///
	///Sends a `GET` request to `/api/igdb/game/search`
	pub async fn search_game_by_name<'a>(
		&'a self,
		query: &'a str,
	) -> Result<ResponseValue<Vec<types::Game>>, Error<()>> {
		let url = format!("{}/api/igdb/game/search", self.baseurl,);
		let mut _query = Vec::with_capacity(1usize);
		_query.push(("query", query.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&_query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for games by its Ids
	///
	///Sends a `GET` request to `/api/igdb/games`
	pub async fn get_games_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::Game>>, Error<()>> {
		let url = format!("{}/api/igdb/games", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for a Genre by Id
	///
	///Sends a `GET` request to `/api/igdb/genre`
	pub async fn get_genre_by_id<'a>(
		&'a self,
		id: i32,
	) -> Result<ResponseValue<types::Genre>, Error<()>> {
		let url = format!("{}/api/igdb/genre", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		query.push(("id", id.to_string()));
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Queries the IGDB API for Genres by Ids
	///
	///Sends a `GET` request to `/api/igdb/genres`
	pub async fn get_genres_by_ids<'a>(
		&'a self,
		ids: &'a Vec<i32>,
	) -> Result<ResponseValue<Vec<types::Genre>>, Error<()>> {
		let url = format!("{}/api/igdb/genres", self.baseurl,);
		let mut query = Vec::with_capacity(1usize);
		for id in ids {
			query.push(("ids", id.to_string()));
		}
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.query(&query)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Returns all platforms with its company and its external metadata
	/// mappings
	///
	///Sends a `GET` request to `/api/platforms`
	pub async fn get_all_platforms<'a>(
		&'a self,
	) -> Result<ResponseValue<Vec<types::PlatformResponse>>, Error<()>> {
		let url = format!("{}/api/platforms", self.baseurl,);
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Returns a platform and its metadata mappings by id
	///
	///Sends a `GET` request to `/api/platforms/{id}`
	pub async fn get_platform_by_id<'a>(
		&'a self,
		id: &'a uuid::Uuid,
	) -> Result<ResponseValue<types::PlatformResponse>, Error<()>> {
		let url = format!(
			"{}/api/platforms/{}",
			self.baseurl,
			encode_path(&id.to_string()),
		);
		#[allow(unused_mut)]
		let mut request = self
			.client
			.get(url)
			.header(
				reqwest::header::ACCEPT,
				reqwest::header::HeaderValue::from_static("application/json"),
			)
			.build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => ResponseValue::from_response(response).await,
			404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}

	///Checks if the service is ready
	///
	///Sends a `GET` request to `/api/ready`
	pub async fn ready<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
		let url = format!("{}/api/ready", self.baseurl,);
		#[allow(unused_mut)]
		let mut request = self.client.get(url).build()?;
		let result = self.client.execute(request).await;
		let response = result?;
		match response.status().as_u16() {
			200u16 => Ok(ResponseValue::empty(response)),
			_ => Err(Error::UnexpectedResponse(response)),
		}
	}
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
	#[allow(unused_imports)]
	pub use super::Client;
}
