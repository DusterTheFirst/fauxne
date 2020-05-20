use super::model::{id::{UserID, SequenceID, MessageID, GuildID, ChannelID}, user::User};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
/// A message that will be recieved from the gateway
struct UnparsedIncomingGatewayMessage {
    #[serde(rename = "op")]
    opcode: u8,
    #[serde(rename = "d")]
    data: Value,
    #[serde(rename = "t")]
    pub name: Option<String>,
    #[serde(rename = "s")]
    pub seq: Option<SequenceID>,
}

#[derive(Debug)]
/// A message that will be recieved from the gateway with a parsed data section
pub struct IncomingGatewayMessage {
    opcode: u8,
    pub data: IncomingGatewayData,
    pub name: Option<String>,
    pub seq: Option<SequenceID>,
}

impl IncomingGatewayMessage {
    pub fn from_json(json: &str) -> serde_json::Result<IncomingGatewayMessage> {
        let UnparsedIncomingGatewayMessage {
            data,
            name,
            opcode,
            seq,
        } = serde_json::from_str(&json)?;

        Ok(IncomingGatewayMessage {
            opcode,
            seq,
            data: match opcode {
                0 => match &name {
                    Some(event) => {
                        IncomingGatewayData::Dispatch(Event::from_event_value(event, data)?)
                    }
                    None => IncomingGatewayData::Unknown(data),
                },
                1 => IncomingGatewayData::Heartbeat(serde_json::from_value(data)?),
                7 => IncomingGatewayData::Reconnect,
                9 => IncomingGatewayData::InvalidSession(serde_json::from_value(data)?),
                10 => IncomingGatewayData::Hello(serde_json::from_value(data)?),
                11 => IncomingGatewayData::HeartbeatAck,
                _ => IncomingGatewayData::Unknown(data),
            },
            name,
        })
    }
}

#[derive(Debug)]
pub enum IncomingGatewayData {
    Dispatch(Event),
    Heartbeat(SequenceID),
    HeartbeatAck,
    Hello(HelloGatewayData),
    InvalidSession(bool),
    Reconnect,
    Unknown(Value),
}

#[derive(Debug, Deserialize)]
pub struct HelloGatewayData {
    pub heartbeat_interval: u64,
}

#[derive(Debug)]
pub enum Event {
    Unknown(String, Value),
    Muted(String),
    Ready(GatewayReady),
    CallUpdate(CallUpdate),
    VoiceStateUpdate(VoiceStateUpdate),
    CallDelete(CallDelete)
}

impl Event {
    fn from_event_value(event: &str, value: Value) -> serde_json::Result<Self> {
        Ok(match event {
            "READY" => Event::Ready(serde_json::from_value(value)?),
            "SESSIONS_REPLACE" | "PRESENCE_UPDATE" | "MESSAGE_CREATE" | "MESSAGE_UPDATE" | "CALL_CREATE" => Event::Muted(event.into()),
            "CALL_UPDATE"=> Event::CallUpdate(serde_json::from_value(value)?),
            "VOICE_STATE_UPDATE" => Event::VoiceStateUpdate(serde_json::from_value(value)?),
            "CALL_DELETE" => Event::CallDelete(serde_json::from_value(value)?),
            _ => Event::Unknown(event.into(), value),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CallUpdate {
    pub channel_id: Option<ChannelID>,
    pub guild_id: Option<GuildID>,
    pub message_id: Option<MessageID>,
    pub region: String,
    pub ringing: Box<[UserID]>
}

#[derive(Debug, Deserialize)]
pub struct VoiceStateUpdate {
    pub channel_id: Option<ChannelID>,
    pub deaf: bool,
    pub guild_id: Option<GuildID>,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_video: bool,
    pub session_id: String,
    pub suppress: bool,
    pub user_id: UserID
}

#[derive(Debug, Deserialize)]
pub struct CallDelete {
    pub channel_id: ChannelID
}

#[derive(Debug, Deserialize)]
pub struct GatewayReady {
    pub user: User,
    pub session_id: String,
}
