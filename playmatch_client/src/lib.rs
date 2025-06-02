mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
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
    ///`AgeRating`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content_descriptions: ::std::option::Option<::std::vec::Vec<i64>>,
        pub id: i32,
        pub rating: AgeRatingEnum,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rating_cover_url: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub synopsis: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&AgeRating> for AgeRating {
        fn from(value: &AgeRating) -> Self {
            value.clone()
        }
    }
    ///`AgeRatingCategory`
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
    #[serde(transparent)]
    pub struct AgeRatingCategory(i64);
    impl ::std::ops::Deref for AgeRatingCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<AgeRatingCategory> for i64 {
        fn from(value: AgeRatingCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&AgeRatingCategory> for AgeRatingCategory {
        fn from(value: &AgeRatingCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for AgeRatingCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AgeRatingCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`AgeRatingContentCategory`
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
    #[serde(transparent)]
    pub struct AgeRatingContentCategory(i64);
    impl ::std::ops::Deref for AgeRatingContentCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<AgeRatingContentCategory> for i64 {
        fn from(value: AgeRatingContentCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&AgeRatingContentCategory> for AgeRatingContentCategory {
        fn from(value: &AgeRatingContentCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for AgeRatingContentCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`AgeRatingContentDescription`
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
        pub checksum: ::uuid::Uuid,
        pub description: ::std::string::String,
        pub id: i32,
    }
    impl ::std::convert::From<&AgeRatingContentDescription> for AgeRatingContentDescription {
        fn from(value: &AgeRatingContentDescription) -> Self {
            value.clone()
        }
    }
    ///`AgeRatingEnum`
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
    #[serde(transparent)]
    pub struct AgeRatingEnum(i64);
    impl ::std::ops::Deref for AgeRatingEnum {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<AgeRatingEnum> for i64 {
        fn from(value: AgeRatingEnum) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&AgeRatingEnum> for AgeRatingEnum {
        fn from(value: &AgeRatingEnum) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for AgeRatingEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`AlternativeName`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        pub game: i32,
        pub id: i32,
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&AlternativeName> for AlternativeName {
        fn from(value: &AlternativeName) -> Self {
            value.clone()
        }
    }
    ///`Artwork`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub game: i32,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&Artwork> for Artwork {
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
    impl ::std::convert::From<&Self> for AutomaticMatchReason {
        fn from(value: &AutomaticMatchReason) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for AutomaticMatchReason {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AlternativeName => write!(f, "AlternativeName"),
                Self::DirectName => write!(f, "DirectName"),
                Self::ViaChild => write!(f, "ViaChild"),
                Self::ViaParent => write!(f, "ViaParent"),
            }
        }
    }
    impl ::std::str::FromStr for AutomaticMatchReason {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "AlternativeName" => Ok(Self::AlternativeName),
                "DirectName" => Ok(Self::DirectName),
                "ViaChild" => Ok(Self::ViaChild),
                "ViaParent" => Ok(Self::ViaParent),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AutomaticMatchReason {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for AutomaticMatchReason {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for AutomaticMatchReason {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`Character`
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
        pub akas: ::std::vec::Vec<::std::string::String>,
        pub checksum: ::uuid::Uuid,
        pub country_name: ::std::string::String,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub games: ::std::vec::Vec<i32>,
        pub gender: CharacterGender,
        pub id: i32,
        pub mug_shot: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub species: CharacterSpecies,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&Character> for Character {
        fn from(value: &Character) -> Self {
            value.clone()
        }
    }
    ///`CharacterGender`
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
    #[serde(transparent)]
    pub struct CharacterGender(i64);
    impl ::std::ops::Deref for CharacterGender {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<CharacterGender> for i64 {
        fn from(value: CharacterGender) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CharacterGender> for CharacterGender {
        fn from(value: &CharacterGender) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for CharacterGender {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CharacterGender {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`CharacterSpecies`
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
    #[serde(transparent)]
    pub struct CharacterSpecies(i64);
    impl ::std::ops::Deref for CharacterSpecies {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<CharacterSpecies> for i64 {
        fn from(value: CharacterSpecies) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CharacterSpecies> for CharacterSpecies {
        fn from(value: &CharacterSpecies) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for CharacterSpecies {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![1_i64, 2_i64, 3_i64, 4_i64, 5_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CharacterSpecies {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`Collection`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub as_child_relations: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub as_parent_relations: ::std::option::Option<::std::vec::Vec<i32>>,
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub games: ::std::vec::Vec<i32>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&Collection> for Collection {
        fn from(value: &Collection) -> Self {
            value.clone()
        }
    }
    ///`CollectionMembership`
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
        pub checksum: ::uuid::Uuid,
        pub collection: i32,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub game: i32,
        pub id: i32,
        #[serde(rename = "type")]
        pub type_: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&CollectionMembership> for CollectionMembership {
        fn from(value: &CollectionMembership) -> Self {
            value.clone()
        }
    }
    ///`CollectionMembershipType`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub id: i32,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&CollectionMembershipType> for CollectionMembershipType {
        fn from(value: &CollectionMembershipType) -> Self {
            value.clone()
        }
    }
    ///`CollectionRelation`
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
        pub checksum: ::uuid::Uuid,
        pub child_collection: i32,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub parent_collection: i32,
        #[serde(rename = "type")]
        pub type_: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&CollectionRelation> for CollectionRelation {
        fn from(value: &CollectionRelation) -> Self {
            value.clone()
        }
    }
    ///`CollectionRelationType`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub id: i32,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&CollectionRelationType> for CollectionRelationType {
        fn from(value: &CollectionRelationType) -> Self {
            value.clone()
        }
    }
    ///`CollectionType`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub id: i32,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&CollectionType> for CollectionType {
        fn from(value: &CollectionType) -> Self {
            value.clone()
        }
    }
    ///`Company`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "change_date_category",
    ///    "checksum",
    ///    "country",
    ///    "created_at",
    ///    "description",
    ///    "developed",
    ///    "id",
    ///    "logo",
    ///    "name",
    ///    "published",
    ///    "slug",
    ///    "start_date_category",
    ///    "updated_at",
    ///    "url",
    ///    "websites"
    ///  ],
    ///  "properties": {
    ///    "change_date": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "change_date_category": {
    ///      "$ref": "#/components/schemas/CompanyChangeDateCategory"
    ///    },
    ///    "changed_company_id": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
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
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
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
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub change_date: ::std::option::Option<i64>,
        pub change_date_category: CompanyChangeDateCategory,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub changed_company_id: ::std::option::Option<i32>,
        pub checksum: ::uuid::Uuid,
        pub country: i32,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub developed: ::std::vec::Vec<i32>,
        pub id: i32,
        pub logo: i32,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<i32>,
        pub published: ::std::vec::Vec<i32>,
        pub slug: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start_date: ::std::option::Option<i64>,
        pub start_date_category: CompanyStartDateCategory,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
        pub websites: ::std::vec::Vec<i32>,
    }
    impl ::std::convert::From<&Company> for Company {
        fn from(value: &Company) -> Self {
            value.clone()
        }
    }
    ///`CompanyChangeDateCategory`
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
    #[serde(transparent)]
    pub struct CompanyChangeDateCategory(i64);
    impl ::std::ops::Deref for CompanyChangeDateCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<CompanyChangeDateCategory> for i64 {
        fn from(value: CompanyChangeDateCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CompanyChangeDateCategory> for CompanyChangeDateCategory {
        fn from(value: &CompanyChangeDateCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for CompanyChangeDateCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompanyChangeDateCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`CompanyLogo`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&CompanyLogo> for CompanyLogo {
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
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub external_metadata: ::std::vec::Vec<ExternalMetadata>,
        ///The ID of the company.
        pub id: ::uuid::Uuid,
        ///The name of the company.
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&CompanyResponse> for CompanyResponse {
        fn from(value: &CompanyResponse) -> Self {
            value.clone()
        }
    }
    ///`CompanyStartDateCategory`
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
    #[serde(transparent)]
    pub struct CompanyStartDateCategory(i64);
    impl ::std::ops::Deref for CompanyStartDateCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<CompanyStartDateCategory> for i64 {
        fn from(value: CompanyStartDateCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CompanyStartDateCategory> for CompanyStartDateCategory {
        fn from(value: &CompanyStartDateCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for CompanyStartDateCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompanyStartDateCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`CompanyWebsite`
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
        pub checksum: ::uuid::Uuid,
        pub id: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub trusted: ::std::option::Option<bool>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&CompanyWebsite> for CompanyWebsite {
        fn from(value: &CompanyWebsite) -> Self {
            value.clone()
        }
    }
    ///`CompanyWebsiteCategory`
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
    #[serde(transparent)]
    pub struct CompanyWebsiteCategory(i64);
    impl ::std::ops::Deref for CompanyWebsiteCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<CompanyWebsiteCategory> for i64 {
        fn from(value: CompanyWebsiteCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CompanyWebsiteCategory> for CompanyWebsiteCategory {
        fn from(value: &CompanyWebsiteCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for CompanyWebsiteCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`Cover`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub game: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub game_localization: ::std::option::Option<i32>,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&Cover> for Cover {
        fn from(value: &Cover) -> Self {
            value.clone()
        }
    }
    ///`Event`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub end_time: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub event_logo: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub event_networks: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub games: ::std::option::Option<::std::vec::Vec<i32>>,
        pub id: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub live_stream_url: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub time_zone: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub videos: ::std::option::Option<::std::vec::Vec<i32>>,
    }
    impl ::std::convert::From<&Event> for Event {
        fn from(value: &Event) -> Self {
            value.clone()
        }
    }
    ///`EventLogo`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub event: i32,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&EventLogo> for EventLogo {
        fn from(value: &EventLogo) -> Self {
            value.clone()
        }
    }
    ///`EventNetwork`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub event: i32,
        pub id: i32,
        pub network_type: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&EventNetwork> for EventNetwork {
        fn from(value: &EventNetwork) -> Self {
            value.clone()
        }
    }
    ///`ExternalGame`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub countries: ::std::option::Option<::std::vec::Vec<i32>>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub game: i32,
        pub id: i32,
        pub media: ExternalGameMedia,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platform: ::std::option::Option<i32>,
        pub uid: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub year: ::std::option::Option<i32>,
    }
    impl ::std::convert::From<&ExternalGame> for ExternalGame {
        fn from(value: &ExternalGame) -> Self {
            value.clone()
        }
    }
    ///`ExternalGameCategory`
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
    #[serde(transparent)]
    pub struct ExternalGameCategory(i64);
    impl ::std::ops::Deref for ExternalGameCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<ExternalGameCategory> for i64 {
        fn from(value: ExternalGameCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ExternalGameCategory> for ExternalGameCategory {
        fn from(value: &ExternalGameCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for ExternalGameCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`ExternalGameMedia`
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
    #[serde(transparent)]
    pub struct ExternalGameMedia(i64);
    impl ::std::ops::Deref for ExternalGameMedia {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<ExternalGameMedia> for i64 {
        fn from(value: ExternalGameMedia) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ExternalGameMedia> for ExternalGameMedia {
        fn from(value: &ExternalGameMedia) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for ExternalGameMedia {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![1_i64, 2_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExternalGameMedia {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/AutomaticMatchReason"
    ///            }
    ///          ]
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
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/FailedMatchReason"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "manualMatchType": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/ManualMatchMode"
    ///            }
    ///          ]
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
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub automatic_match_reason: ::std::option::Option<AutomaticMatchReason>,
        ///Optional Comment about the match.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "failedMatchReason",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub failed_match_reason: ::std::option::Option<FailedMatchReason>,
        #[serde(
            rename = "manualMatchType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub manual_match_type: ::std::option::Option<ManualMatchMode>,
        #[serde(rename = "matchType")]
        pub match_type: MatchType,
        ///The ID of the game for this provider.
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "providerName")]
        pub provider_name: MetadataProvider,
    }
    impl ::std::convert::From<&ExternalMetadata> for ExternalMetadata {
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
    impl ::std::convert::From<&Self> for FailedMatchReason {
        fn from(value: &FailedMatchReason) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for FailedMatchReason {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NoDirectMatch => write!(f, "NoDirectMatch"),
                Self::TooManyMatches => write!(f, "TooManyMatches"),
            }
        }
    }
    impl ::std::str::FromStr for FailedMatchReason {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NoDirectMatch" => Ok(Self::NoDirectMatch),
                "TooManyMatches" => Ok(Self::TooManyMatches),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FailedMatchReason {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for FailedMatchReason {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FailedMatchReason {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`Franchise`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub games: ::std::option::Option<::std::vec::Vec<i32>>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&Franchise> for Franchise {
        fn from(value: &Franchise) -> Self {
            value.clone()
        }
    }
    ///`Game`
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
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/GameStatus"
    ///            }
    ///          ]
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub age_ratings: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub aggregated_rating: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub aggregated_rating_count: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alternative_names: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub artworks: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bundles: ::std::option::Option<::std::vec::Vec<i32>>,
        pub category: GameCategory,
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub collection: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub collections: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cover: ::std::option::Option<i32>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dlcs: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expanded_games: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expansions: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub external_games: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub first_release_date: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub follows: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub forks: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub franchise: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub franchises: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub game_engines: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub game_localizations: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub game_modes: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub genres: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hypes: ::std::option::Option<i32>,
        pub id: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub involved_companies: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub keywords: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub language_supports: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub multiplayer_modes: ::std::option::Option<::std::vec::Vec<i32>>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parent_game: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platforms: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub player_perspectives: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ports: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rating: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rating_count: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub release_dates: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub remakes: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub remasters: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub screenshots: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub similar_games: ::std::option::Option<::std::vec::Vec<i32>>,
        pub slug: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub standalone_expansions: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<GameStatus>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub storyline: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub summary: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tags: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub themes: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_rating: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_rating_count: ::std::option::Option<i32>,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_parent: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub videos: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub websites: ::std::option::Option<::std::vec::Vec<i32>>,
    }
    impl ::std::convert::From<&Game> for Game {
        fn from(value: &Game) -> Self {
            value.clone()
        }
    }
    ///`GameCategory`
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
    #[serde(transparent)]
    pub struct GameCategory(i64);
    impl ::std::ops::Deref for GameCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<GameCategory> for i64 {
        fn from(value: GameCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&GameCategory> for GameCategory {
        fn from(value: &GameCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for GameCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`GameEngine`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub companies: ::std::option::Option<::std::vec::Vec<i32>>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        pub id: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub logo: ::std::option::Option<i32>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platforms: ::std::option::Option<::std::vec::Vec<i32>>,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&GameEngine> for GameEngine {
        fn from(value: &GameEngine) -> Self {
            value.clone()
        }
    }
    ///`GameEngineLogo`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&GameEngineLogo> for GameEngineLogo {
        fn from(value: &GameEngineLogo) -> Self {
            value.clone()
        }
    }
    ///`GameLocalization`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cover: ::std::option::Option<i32>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub game: i32,
        pub id: i32,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub region: ::std::option::Option<i32>,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&GameLocalization> for GameLocalization {
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
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub external_metadata: ::std::vec::Vec<ExternalMetadata>,
        #[serde(rename = "gameMatchType")]
        pub game_match_type: GameMatchType,
        ///If a match was found, the ID of the matched game.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::uuid::Uuid>,
    }
    impl ::std::convert::From<&GameMatchResult> for GameMatchResult {
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
    impl ::std::convert::From<&Self> for GameMatchType {
        fn from(value: &GameMatchType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for GameMatchType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Sha256 => write!(f, "SHA256"),
                Self::Sha1 => write!(f, "SHA1"),
                Self::Md5 => write!(f, "MD5"),
                Self::FileNameAndSize => write!(f, "FileNameAndSize"),
                Self::NoMatch => write!(f, "NoMatch"),
            }
        }
    }
    impl ::std::str::FromStr for GameMatchType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
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
    impl ::std::convert::TryFrom<&str> for GameMatchType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for GameMatchType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for GameMatchType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`GameMode`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&GameMode> for GameMode {
        fn from(value: &GameMode) -> Self {
            value.clone()
        }
    }
    ///`GameStatus`
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
    #[serde(transparent)]
    pub struct GameStatus(i64);
    impl ::std::ops::Deref for GameStatus {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<GameStatus> for i64 {
        fn from(value: GameStatus) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&GameStatus> for GameStatus {
        fn from(value: &GameStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for GameStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GameStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`GameVersion`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub features: ::std::option::Option<::std::vec::Vec<i32>>,
        pub game: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub games: ::std::option::Option<::std::vec::Vec<i32>>,
        pub id: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&GameVersion> for GameVersion {
        fn from(value: &GameVersion) -> Self {
            value.clone()
        }
    }
    ///`GameVersionFeature`
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
        pub checksum: ::uuid::Uuid,
        pub description: ::std::string::String,
        pub id: i32,
        pub position: i32,
        pub title: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub values: ::std::option::Option<::std::vec::Vec<i32>>,
    }
    impl ::std::convert::From<&GameVersionFeature> for GameVersionFeature {
        fn from(value: &GameVersionFeature) -> Self {
            value.clone()
        }
    }
    ///`GameVersionFeatureCategory`
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
    #[serde(transparent)]
    pub struct GameVersionFeatureCategory(i64);
    impl ::std::ops::Deref for GameVersionFeatureCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<GameVersionFeatureCategory> for i64 {
        fn from(value: GameVersionFeatureCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&GameVersionFeatureCategory> for GameVersionFeatureCategory {
        fn from(value: &GameVersionFeatureCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for GameVersionFeatureCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GameVersionFeatureCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`GameVersionFeatureValue`
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
        pub checksum: ::uuid::Uuid,
        pub game: i32,
        pub game_feature: i32,
        pub id: i32,
        pub included_feature: GameVersionFeatureValueEnum,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub note: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&GameVersionFeatureValue> for GameVersionFeatureValue {
        fn from(value: &GameVersionFeatureValue) -> Self {
            value.clone()
        }
    }
    ///`GameVersionFeatureValueEnum`
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
    #[serde(transparent)]
    pub struct GameVersionFeatureValueEnum(i64);
    impl ::std::ops::Deref for GameVersionFeatureValueEnum {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<GameVersionFeatureValueEnum> for i64 {
        fn from(value: GameVersionFeatureValueEnum) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&GameVersionFeatureValueEnum> for GameVersionFeatureValueEnum {
        fn from(value: &GameVersionFeatureValueEnum) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for GameVersionFeatureValueEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GameVersionFeatureValueEnum {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`GameVideo`
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
        pub checksum: ::uuid::Uuid,
        pub game: i32,
        pub id: i32,
        pub name: ::std::string::String,
        pub video_id: ::std::string::String,
    }
    impl ::std::convert::From<&GameVideo> for GameVideo {
        fn from(value: &GameVideo) -> Self {
            value.clone()
        }
    }
    ///`Genre`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&Genre> for Genre {
        fn from(value: &Genre) -> Self {
            value.clone()
        }
    }
    ///`InvolvedCompany`
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
        pub checksum: ::uuid::Uuid,
        pub company: i32,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub developer: bool,
        pub game: i32,
        pub id: i32,
        pub porting: bool,
        pub publisher: bool,
        pub supporting: bool,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&InvolvedCompany> for InvolvedCompany {
        fn from(value: &InvolvedCompany) -> Self {
            value.clone()
        }
    }
    ///`Keyword`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&Keyword> for Keyword {
        fn from(value: &Keyword) -> Self {
            value.clone()
        }
    }
    ///`Language`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub locale: ::std::string::String,
        pub name: ::std::string::String,
        pub native_name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&Language> for Language {
        fn from(value: &Language) -> Self {
            value.clone()
        }
    }
    ///`LanguageSupport`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub game: i32,
        pub id: i32,
        pub language: i32,
        pub language_support_type: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&LanguageSupport> for LanguageSupport {
        fn from(value: &LanguageSupport) -> Self {
            value.clone()
        }
    }
    ///`LanguageSupportType`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&LanguageSupportType> for LanguageSupportType {
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
    impl ::std::convert::From<&Self> for ManualMatchMode {
        fn from(value: &ManualMatchMode) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for ManualMatchMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => write!(f, "Admin"),
                Self::Community => write!(f, "Community"),
                Self::Trusted => write!(f, "Trusted"),
            }
        }
    }
    impl ::std::str::FromStr for ManualMatchMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Admin" => Ok(Self::Admin),
                "Community" => Ok(Self::Community),
                "Trusted" => Ok(Self::Trusted),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ManualMatchMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ManualMatchMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ManualMatchMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MatchRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "manual_match_type",
    ///    "provider",
    ///    "provider_id"
    ///  ],
    ///  "properties": {
    ///    "comment": {
    ///      "description": "Optional comment about the match.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "manual_match_type": {
    ///      "$ref": "#/components/schemas/ManualMatchMode"
    ///    },
    ///    "md5": {
    ///      "description": "MD5 hash of the game file.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "Name of game or file.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "provider": {
    ///      "$ref": "#/components/schemas/MetadataProvider"
    ///    },
    ///    "provider_id": {
    ///      "description": "ID of the game file in the metadata provider.",
    ///      "type": "string"
    ///    },
    ///    "sha1": {
    ///      "description": "SHA1 hash of the game file.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "sha256": {
    ///      "description": "SHA256 hash of the game file.",
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
    pub struct MatchRequest {
        ///Optional comment about the match.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        pub manual_match_type: ManualMatchMode,
        ///MD5 hash of the game file.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub md5: ::std::option::Option<::std::string::String>,
        ///Name of game or file.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        pub provider: MetadataProvider,
        ///ID of the game file in the metadata provider.
        pub provider_id: ::std::string::String,
        ///SHA1 hash of the game file.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha1: ::std::option::Option<::std::string::String>,
        ///SHA256 hash of the game file.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha256: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&MatchRequest> for MatchRequest {
        fn from(value: &MatchRequest) -> Self {
            value.clone()
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
    impl ::std::convert::From<&Self> for MatchType {
        fn from(value: &MatchType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for MatchType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Automatic => write!(f, "Automatic"),
                Self::Failed => write!(f, "Failed"),
                Self::Manual => write!(f, "Manual"),
                Self::None => write!(f, "None"),
            }
        }
    }
    impl ::std::str::FromStr for MatchType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Automatic" => Ok(Self::Automatic),
                "Failed" => Ok(Self::Failed),
                "Manual" => Ok(Self::Manual),
                "None" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MatchType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MatchType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MatchType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
    impl ::std::convert::From<&Self> for MetadataProvider {
        fn from(value: &MetadataProvider) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for MetadataProvider {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Igdb => write!(f, "IGDB"),
            }
        }
    }
    impl ::std::str::FromStr for MetadataProvider {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IGDB" => Ok(Self::Igdb),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MetadataProvider {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MetadataProvider {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MetadataProvider {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`MultiplayerMode`
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
        pub checksum: ::uuid::Uuid,
        pub dropin: bool,
        pub game: i32,
        pub id: i32,
        pub lancoop: bool,
        pub offlinecoop: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offlinecoopmax: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offlinemax: ::std::option::Option<i32>,
        pub onlinecoop: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub onlinecoopmax: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub onlinemax: ::std::option::Option<i32>,
        pub platform: i32,
        pub splitscreen: bool,
        pub splitscreenonline: bool,
    }
    impl ::std::convert::From<&MultiplayerMode> for MultiplayerMode {
        fn from(value: &MultiplayerMode) -> Self {
            value.clone()
        }
    }
    ///`NetworkType`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub event_networks: ::std::option::Option<::std::vec::Vec<i32>>,
        pub id: i32,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&NetworkType> for NetworkType {
        fn from(value: &NetworkType) -> Self {
            value.clone()
        }
    }
    ///`Platform`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub abbreviation: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alternative_name: ::std::option::Option<::std::string::String>,
        pub category: PlatformCategory,
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub generation: ::std::option::Option<i32>,
        pub id: i32,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platform_family: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platform_logo: ::std::option::Option<i32>,
        pub slug: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub summary: ::std::option::Option<::std::string::String>,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub versions: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub websites: ::std::option::Option<::std::vec::Vec<i32>>,
    }
    impl ::std::convert::From<&Platform> for Platform {
        fn from(value: &Platform) -> Self {
            value.clone()
        }
    }
    ///`PlatformCategory`
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
    #[serde(transparent)]
    pub struct PlatformCategory(i64);
    impl ::std::ops::Deref for PlatformCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<PlatformCategory> for i64 {
        fn from(value: PlatformCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PlatformCategory> for PlatformCategory {
        fn from(value: &PlatformCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for PlatformCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlatformCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`PlatformFamily`
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
        pub checksum: ::uuid::Uuid,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
    }
    impl ::std::convert::From<&PlatformFamily> for PlatformFamily {
        fn from(value: &PlatformFamily) -> Self {
            value.clone()
        }
    }
    ///`PlatformLogo`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&PlatformLogo> for PlatformLogo {
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
        #[serde(
            rename = "companyId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub company_id: ::std::option::Option<::uuid::Uuid>,
        ///Optional name of the company that made the platform.
        #[serde(
            rename = "companyName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub company_name: ::std::option::Option<::std::string::String>,
        ///External metadata for the platform.
        #[serde(
            rename = "externalMetadata",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub external_metadata: ::std::vec::Vec<ExternalMetadata>,
        ///The ID of the platform.
        pub id: ::uuid::Uuid,
        ///The name of the platform.
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&PlatformResponse> for PlatformResponse {
        fn from(value: &PlatformResponse) -> Self {
            value.clone()
        }
    }
    ///`PlatformVersion`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub companies: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub connectivity: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cpu: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub graphics: ::std::option::Option<::std::string::String>,
        pub id: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub main_manufacturer: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub media: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub memory: ::std::option::Option<::std::string::String>,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub os: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub output: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platform_logo: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub platform_version_release_dates: ::std::option::Option<::std::vec::Vec<i32>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub resolutions: ::std::option::Option<::std::string::String>,
        pub slug: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sound: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub storage: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub summary: ::std::option::Option<::std::string::String>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&PlatformVersion> for PlatformVersion {
        fn from(value: &PlatformVersion) -> Self {
            value.clone()
        }
    }
    ///`PlatformVersionCompany`
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
        pub checksum: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub comment: ::std::option::Option<::std::string::String>,
        pub company: i32,
        pub developer: bool,
        pub id: i32,
        pub manufacturer: bool,
    }
    impl ::std::convert::From<&PlatformVersionCompany> for PlatformVersionCompany {
        fn from(value: &PlatformVersionCompany) -> Self {
            value.clone()
        }
    }
    ///`PlatformVersionReleaseDate`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub date: i64,
        pub human: ::std::string::String,
        pub id: i32,
        pub m: i32,
        pub platform_version: i32,
        pub region: PlatformVersionReleaseDateRegion,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub y: i32,
    }
    impl ::std::convert::From<&PlatformVersionReleaseDate> for PlatformVersionReleaseDate {
        fn from(value: &PlatformVersionReleaseDate) -> Self {
            value.clone()
        }
    }
    ///`PlatformVersionReleaseDateCategory`
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
    #[serde(transparent)]
    pub struct PlatformVersionReleaseDateCategory(i64);
    impl ::std::ops::Deref for PlatformVersionReleaseDateCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<PlatformVersionReleaseDateCategory> for i64 {
        fn from(value: PlatformVersionReleaseDateCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PlatformVersionReleaseDateCategory>
        for PlatformVersionReleaseDateCategory
    {
        fn from(value: &PlatformVersionReleaseDateCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for PlatformVersionReleaseDateCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlatformVersionReleaseDateCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`PlatformVersionReleaseDateRegion`
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
    #[serde(transparent)]
    pub struct PlatformVersionReleaseDateRegion(i64);
    impl ::std::ops::Deref for PlatformVersionReleaseDateRegion {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<PlatformVersionReleaseDateRegion> for i64 {
        fn from(value: PlatformVersionReleaseDateRegion) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PlatformVersionReleaseDateRegion> for PlatformVersionReleaseDateRegion {
        fn from(value: &PlatformVersionReleaseDateRegion) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for PlatformVersionReleaseDateRegion {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`PlatformWebsite`
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
        pub checksum: ::uuid::Uuid,
        pub id: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub trusted: ::std::option::Option<bool>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&PlatformWebsite> for PlatformWebsite {
        fn from(value: &PlatformWebsite) -> Self {
            value.clone()
        }
    }
    ///`PlatformWebsiteCategory`
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
    #[serde(transparent)]
    pub struct PlatformWebsiteCategory(i64);
    impl ::std::ops::Deref for PlatformWebsiteCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<PlatformWebsiteCategory> for i64 {
        fn from(value: PlatformWebsiteCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PlatformWebsiteCategory> for PlatformWebsiteCategory {
        fn from(value: &PlatformWebsiteCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for PlatformWebsiteCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`PlayerPerspective`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&PlayerPerspective> for PlayerPerspective {
        fn from(value: &PlayerPerspective) -> Self {
            value.clone()
        }
    }
    ///`PopularityPrimitive`
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
        pub calculated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub game_id: i32,
        pub id: i32,
        pub popularity_source: PopularitySource,
        pub popularity_type: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&PopularityPrimitive> for PopularityPrimitive {
        fn from(value: &PopularityPrimitive) -> Self {
            value.clone()
        }
    }
    ///`PopularitySource`
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
    #[serde(transparent)]
    pub struct PopularitySource(i64);
    impl ::std::ops::Deref for PopularitySource {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<PopularitySource> for i64 {
        fn from(value: PopularitySource) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PopularitySource> for PopularitySource {
        fn from(value: &PopularitySource) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for PopularitySource {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![121_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PopularitySource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`PopularityType`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub popularity_source: PopularitySource,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&PopularityType> for PopularityType {
        fn from(value: &PopularityType) -> Self {
            value.clone()
        }
    }
    ///`Region`
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
        pub category: ::std::string::String,
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub identifier: ::std::string::String,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&Region> for Region {
        fn from(value: &Region) -> Self {
            value.clone()
        }
    }
    ///`ReleaseDate`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub date: ::chrono::DateTime<::chrono::offset::Utc>,
        pub game: i32,
        pub human: ::std::string::String,
        pub id: i32,
        pub m: i32,
        pub platform: i32,
        pub region: ReleaseDateRegion,
        pub status: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub y: i32,
    }
    impl ::std::convert::From<&ReleaseDate> for ReleaseDate {
        fn from(value: &ReleaseDate) -> Self {
            value.clone()
        }
    }
    ///`ReleaseDateCategory`
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
    #[serde(transparent)]
    pub struct ReleaseDateCategory(i64);
    impl ::std::ops::Deref for ReleaseDateCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<ReleaseDateCategory> for i64 {
        fn from(value: ReleaseDateCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReleaseDateCategory> for ReleaseDateCategory {
        fn from(value: &ReleaseDateCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for ReleaseDateCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReleaseDateCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`ReleaseDateRegion`
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
    #[serde(transparent)]
    pub struct ReleaseDateRegion(i64);
    impl ::std::ops::Deref for ReleaseDateRegion {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<ReleaseDateRegion> for i64 {
        fn from(value: ReleaseDateRegion) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReleaseDateRegion> for ReleaseDateRegion {
        fn from(value: &ReleaseDateRegion) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for ReleaseDateRegion {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }
    ///`ReleaseDateStatus`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub description: ::std::string::String,
        pub id: i32,
        pub name: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&ReleaseDateStatus> for ReleaseDateStatus {
        fn from(value: &ReleaseDateStatus) -> Self {
            value.clone()
        }
    }
    ///`Screenshot`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub alpha_channel: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub animated: ::std::option::Option<bool>,
        pub checksum: ::uuid::Uuid,
        pub game: i32,
        pub height: i32,
        pub id: i32,
        pub image_id: ::std::string::String,
        pub url: ::std::string::String,
        pub width: i32,
    }
    impl ::std::convert::From<&Screenshot> for Screenshot {
        fn from(value: &Screenshot) -> Self {
            value.clone()
        }
    }
    ///`Theme`
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
        pub checksum: ::uuid::Uuid,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub name: ::std::string::String,
        pub slug: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub url: ::std::string::String,
    }
    impl ::std::convert::From<&Theme> for Theme {
        fn from(value: &Theme) -> Self {
            value.clone()
        }
    }
    ///Result of a manual game match.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result of a manual game match.",
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "externalMetadata": {
    ///      "description": "External metadata for the matched game.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExternalMetadata"
    ///      }
    ///    },
    ///    "id": {
    ///      "description": "ID of the game.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdatedMatchResult {
        ///External metadata for the matched game.
        #[serde(
            rename = "externalMetadata",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub external_metadata: ::std::vec::Vec<ExternalMetadata>,
        ///ID of the game.
        pub id: ::uuid::Uuid,
    }
    impl ::std::convert::From<&UpdatedMatchResult> for UpdatedMatchResult {
        fn from(value: &UpdatedMatchResult) -> Self {
            value.clone()
        }
    }
    ///`WebsiteCategory`
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
    #[serde(transparent)]
    pub struct WebsiteCategory(i64);
    impl ::std::ops::Deref for WebsiteCategory {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl ::std::convert::From<WebsiteCategory> for i64 {
        fn from(value: WebsiteCategory) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&WebsiteCategory> for WebsiteCategory {
        fn from(value: &WebsiteCategory) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::TryFrom<i64> for WebsiteCategory {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
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
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
#[allow(elided_named_lifetimes)]
impl Client {
    ///Returns all companies and its external metadata mappings
    ///
    ///Sends a `GET` request to `/api/companies`
    pub async fn get_all_companies<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::CompanyResponse>>, Error<()>> {
        let url = format!("{}/api/companies", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
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
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::CompanyResponse>, Error<()>> {
        let url = format!(
            "{}/api/companies/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("fileName", &file_name))
            .query(&progenitor_client::QueryParam::new("fileSize", &file_size))
            .query(&progenitor_client::QueryParam::new("md5", &md5))
            .query(&progenitor_client::QueryParam::new("sha1", &sha1))
            .query(&progenitor_client::QueryParam::new("sha256", &sha256))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::AgeRating>>, Error<()>> {
        let url = format!("{}/api/igdb/age-ratings", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::AlternativeName>>, Error<()>> {
        let url = format!("{}/api/igdb/alternative-names", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Artwork>>, Error<()>> {
        let url = format!("{}/api/igdb/artworks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Collection>>, Error<()>> {
        let url = format!("{}/api/igdb/collections", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Cover>>, Error<()>> {
        let url = format!("{}/api/igdb/covers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::ExternalGame>>, Error<()>> {
        let url = format!("{}/api/igdb/external-games", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Franchise>>, Error<()>> {
        let url = format!("{}/api/igdb/franchises", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Queries the IGDB API for a game by its Id or Slug
    ///
    ///Sends a `GET` request to `/api/igdb/game`
    pub async fn get_game_by_id<'a>(
        &'a self,
        id: Option<i32>,
        slug: Option<&'a str>,
    ) -> Result<ResponseValue<types::Game>, Error<()>> {
        let url = format!("{}/api/igdb/game", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("slug", &slug))
            .headers(header_map)
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
    ) -> Result<ResponseValue<::std::vec::Vec<types::Game>>, Error<()>> {
        let url = format!("{}/api/igdb/game/search", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("query", &query))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Game>>, Error<()>> {
        let url = format!("{}/api/igdb/games", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
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
        ids: &'a ::std::vec::Vec<i32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Genre>>, Error<()>> {
        let url = format!("{}/api/igdb/genres", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("ids", &ids))
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Endpoint is currently private as community suggestions are still being
    /// worked on, manually match a game by its file hashes or filename,
    /// returning the matched game ExternalMetadata
    ///
    ///Sends a `POST` request to `/api/match`
    ///
    ///Arguments:
    /// - `body`:
    pub async fn match_game<'a>(
        &'a self,
        body: &'a types::MatchRequest,
    ) -> Result<ResponseValue<::std::vec::Vec<types::UpdatedMatchResult>>, Error<()>> {
        let url = format!("{}/api/match", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    ///Returns all platforms with its company and its external metadata
    /// mappings
    ///
    ///Sends a `GET` request to `/api/platforms`
    pub async fn get_all_platforms<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::PlatformResponse>>, Error<()>> {
        let url = format!("{}/api/platforms", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
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
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::PlatformResponse>, Error<()>> {
        let url = format!(
            "{}/api/platforms/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(self.api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
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
