use super::text::Emoji;
use serde::Serialize;
use serde_repr::Serialize_repr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
#[non_exhaustive]
/// The user's activity
pub struct Activity {
    /// The activity's name
    pub name: String,
    #[serde(rename = "type")]
    /// The activity's type
    pub activity_type: ActivityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The stream url, it is validated when type is `Streaming`
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unix timestamp of when the activity was added to the user's session
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's current party status
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The emoji used for a custom status
    pub emoji: Option<Emoji>,
}

impl Activity {
    fn created_at() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    pub fn game(name: &str) -> Self {
        Activity {
            activity_type: ActivityType::Game,
            created_at: Some(Self::created_at()),
            emoji: None,
            name: name.into(),
            state: None,
            url: None,
        }
    }

    pub fn listening(name: &str) -> Self {
        Activity {
            activity_type: ActivityType::Listening,
            created_at: Some(Self::created_at()),
            emoji: None,
            name: name.into(),
            state: None,
            url: None,
        }
    }

    pub fn streaming(name: &str, url: &str) -> Self {
        Activity {
            activity_type: ActivityType::Streaming,
            created_at: Some(Self::created_at()),
            emoji: None,
            name: name.into(),
            state: None,
            url: Some(url.into()),
        }
    }

    pub fn custom(status: &str, emoji: Emoji) -> Self {
        Activity {
            activity_type: ActivityType::Custom,
            created_at: Some(Self::created_at()),
            emoji: Some(emoji),
            name: "Custom Status".into(),
            state: Some(status.into()),
            url: None,
        }
    }
}

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
/// The activity's type
pub enum ActivityType {
    /// `Playing {name}`
    Game = 0,
    /// `Streaming {details}`
    Streaming = 1,
    /// `Listening to {name}`
    Listening = 2,
    /// `{emoji} {name}`
    Custom = 4,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Online
    Online,
    /// Do Not Disturb
    DnD,
    // AFK
    Idle,
    /// Invisible and shown as offline
    Invisible,
    /// Offline
    Offline,
}
