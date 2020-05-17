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
    pub seq: Option<u64>,
}

#[derive(Debug)]
/// A message that will be recieved from the gateway with a parsed data section
pub struct IncomingGatewayMessage {
    opcode: u8,
    pub data: IncomingGatewayData,
    pub name: Option<String>,
    pub seq: Option<u64>,
}

impl IncomingGatewayMessage {
    pub fn from_json(json: &str) -> serde_json::Result<IncomingGatewayMessage> {
        let unparsed: UnparsedIncomingGatewayMessage = serde_json::from_str(&json)?;

        Ok(IncomingGatewayMessage {
            opcode: unparsed.opcode,
            name: unparsed.name,
            seq: unparsed.seq,
            data: match unparsed.opcode {
                1 => IncomingGatewayData::Heartbeat(serde_json::from_value(unparsed.data)?),
                10 => IncomingGatewayData::Hello(serde_json::from_value(unparsed.data)?),
                11 => IncomingGatewayData::HeartbeatAck,
                _ => IncomingGatewayData::Unknown(unparsed.data),
            },
        })
    }
}

#[derive(Debug)]
pub enum IncomingGatewayData {
    Hello(HelloGatewayData),
    Heartbeat(u64),
    HeartbeatAck,
    Unknown(Value),
}

#[derive(Debug, Deserialize)]
pub struct HelloGatewayData {
    pub heartbeat_interval: u64,
}
