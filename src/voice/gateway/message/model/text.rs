use super::id::Snowflake;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Emoji {
    /// The name of the emoji
    name: String,
    /// The id of the emoji
    id: Option<Snowflake>,
    /// Whether this emoji is animated
    animated: Option<bool>,
}
