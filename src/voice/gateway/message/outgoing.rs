use super::{
    super::intents::Intent,
    model::{
        activity::{Activity, Status},
        id::{ChannelID, GuildID, SequenceID},
    },
};
use serde::Serialize;

#[derive(Debug, Serialize)]
/// A message that will be sent to the gateway
pub struct OutgoingGatewayMessage {
    #[serde(rename = "op")]
    opcode: u8,
    #[serde(rename = "d")]
    pub data: OutgoingGatewayData,
}

impl OutgoingGatewayMessage {
    /// Convert the gateway message to a json object
    pub fn to_json(self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
    /// Convert the gateway message to a pretty json object
    pub fn to_json_pretty(self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }
}

impl From<OutgoingGatewayData> for OutgoingGatewayMessage {
    fn from(data: OutgoingGatewayData) -> Self {
        OutgoingGatewayMessage {
            opcode: match &data {
                OutgoingGatewayData::Heartbeat(_) => 1,
                OutgoingGatewayData::Identify { .. } => 2,
                OutgoingGatewayData::PresenceUpdate { .. } => 3,
                OutgoingGatewayData::VoiceStateUpdate { .. } => 4,
                OutgoingGatewayData::Resume { .. } => 6,
                OutgoingGatewayData::HeartbeatAck => 11,
            },
            data,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
/// Data that will be sent to the gateway
pub enum OutgoingGatewayData {
    Heartbeat(Option<u64>),
    HeartbeatAck,
    Identify {
        intents: Intent,
        properties: IdentifyProperties,
        token: String,
    },
    PresenceUpdate {
        /// Unix time (in milliseconds) of when the client went idle, or `None` if the client is not idle
        since: Option<u64>,
        /// `None`, or the user's new activity
        game: Option<Activity>,
        /// The user's new [Status]
        status: Status,
        /// Whether or not the client is afk
        afk: bool,
    },
    Resume {
        token: String,
        session_id: String,
        seq: SequenceID,
    },
    VoiceStateUpdate {
        guild_id: Option<GuildID>,
        channel_id: Option<ChannelID>,
        self_mute: bool,
        self_deaf: bool,
    },
}

#[derive(Debug, Serialize)]
pub struct IdentifyProperties {
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
    pub device: String,
}
