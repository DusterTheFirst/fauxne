use super::model::{id::SequenceID, user::User};
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
                        IncomingGatewayData::Dispatch(Event::from_event_value(event, data))
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
    Ready(GatewayReady),
}

impl Event {
    fn from_event_value(event: &str, value: Value) -> Self {
        match event {
            // "READY" => Event::Ready("".into()),
            _ => Event::Unknown(event.into(), value),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GatewayReady {
    user: User,
    session_id: String,
}
