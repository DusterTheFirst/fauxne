use super::id::Snowflake;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct Emoji {
    /// The name of the emoji
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id of the emoji
    pub id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this emoji is animated
    pub animated: Option<bool>,
}

impl Emoji {
    pub fn named(name: &str) -> Self {
        Emoji {
            name: name.into(),
            id: None,
            animated: None,
        }
    }

    pub fn custom(name: &str, id: Snowflake, animated: bool) -> Self {
        Emoji {
            name: name.into(),
            id: Some(id),
            animated: Some(animated),
        }
    }
}
