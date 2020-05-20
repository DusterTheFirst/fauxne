use super::{
    id::{ApplicationID, Snowflake},
    text::Emoji,
};
use bitflags::bitflags;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
/// The user's activity
pub struct Activity {
    /// The activity's name
    name: String,
    #[serde(rename = "type")]
    /// The activity's type
    activity_type: ActivityType,
    /// The stream url, it is validated when type is `Streaming`
    url: Option<String>,
    /// Unix timestamp of when the activity was added to the user's session
    created_at: u64,
    /// Unix timestamps for start and/or end of the game
    timestamps: Option<ActivityTimestamps>,
    /// The application id for the game
    application_id: Option<ApplicationID>,
    /// What the player is currently doing
    details: Option<String>,
    /// The user's current party status
    state: Option<String>,
    /// The emoji used for a custom status
    emoji: Option<Emoji>,
    /// Information for the current party of the player
    party: Option<Party>,
    /// Images for the presence and their hover texts
    assets: Option<Assets>,
    /// Whether or not the activity is an instanced game session
    instance: bool,
    /// Activity flags, describes what the payload includes
    flags: ActivityFlags,
}
#[derive(Debug, Serialize)]
/// Information for the current party of the player
pub struct Party {
    /// The id of the party
    id: String,
    /// Used to show the party's current and maximum size
    size: Option<(u8, u8)>,
}

#[derive(Debug, Serialize)]
/// Images for the presence and their hover texts
pub struct Assets {
    /// The id for a large asset of the activity, usually a snowflake
    large_image: Option<Snowflake>,
    /// Text displayed when hovering over the large image of the activity
    large_text: Option<String>,
    ///	The id for a small asset of the activity, usually a snowflake
    small_image: Option<Snowflake>,
    /// Text displayed when hovering over the small image of the activity
    small_text: Option<String>,
}

bitflags! {
    /// Activity flags, describes what the payload includes
    pub struct ActivityFlags: u8 {
        const INSTANCE	    = 1 << 0;
        const JOIN	        = 1 << 1;
        const SPECTATE	    = 1 << 2;
        const JOIN_REQUEST	= 1 << 3;
        const SYNC	        = 1 << 4;
        const PLAY	        = 1 << 5;
    }
}

impl Serialize for ActivityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.bits)
    }
}

#[derive(Debug, Serialize)]
/// Unix timestamps for start and/or end of the game
pub struct ActivityTimestamps {
    /// Unix time (in milliseconds) of when the activity started
    start: Option<u64>,
    /// Unix time (in milliseconds) of when the activity ends
    end: Option<u64>,
}

#[derive(Debug, Serialize)]
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
