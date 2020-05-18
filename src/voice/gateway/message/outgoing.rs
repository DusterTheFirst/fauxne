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
    Identify {
        token: String,
        properties: IdentifyProperties,
        intents: u16,
    },
    Heartbeat(Option<u64>),
    HeartbeatAck,
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
