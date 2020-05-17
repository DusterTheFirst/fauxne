use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use async_std::task;
use async_tungstenite::{
    async_std::connect_async,
    stream::Stream,
    tungstenite::{self, Message},
    WebSocketStream,
};
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::time::Duration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred in serde_json")]
    JSON(#[from] serde_json::Error),
    #[error("An error occurred dealing with websockets")]
    WS(#[from] tungstenite::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct VoiceClient {
    stream: WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>,
}

impl VoiceClient {
    const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=6&encoding=json";

    pub async fn connect() -> Result<Self> {
        let (stream, _) = connect_async(Self::GATEWAY).await?;

        Ok(VoiceClient { stream })
    }

    pub async fn login(&mut self, token: &str) -> Result<()> {
        self.send_gateway_message(
            OutgoingGatewayData::Identify {
                token: token.into(),
                properties: IdentifyProperties {
                    os: if cfg!(target_os = "macos") {
                        "macos"
                    } else if cfg!(target_os = "windows") {
                        "windows"
                    } else if cfg!(target_os = "linux") {
                        "linux"
                    } else {
                        "unknown"
                    }
                    .into(),
                    browser: env!("CARGO_PKG_NAME").into(),
                    device: env!("CARGO_PKG_NAME").into(),
                },
                intents: intents::DIRECT_MESSAGES,
            }
            .into(),
        )
        .await?;

        while let Some(Ok(response)) = self.stream.next().await {
            match response {
                Message::Text(json) => {
                    trace!("Recieved: {:?}", json);

                    match serde_json::from_str::<IncomingGatewayMessage>(&json) {
                        Ok(incoming) => match incoming.data {
                            IncomingGatewayData::Hello { heartbeat_interval } => {
                                debug!(
                                    "Starting heartbeating with an interval of {}ms",
                                    heartbeat_interval
                                );

                                task::spawn(async move {
                                    loop {

                                        task::sleep(Duration::from_millis(heartbeat_interval));
                                    }
                                });
                            }
                        },
                        Err(e) => error!("Failed to parse incoming json: {:?}\n{:?}", e, json),
                    };
                }
                Message::Binary(_) => {
                    warn!("Discord gateway sent binary data that could not be understood")
                }
                Message::Ping(_) => trace!("Ping!"),
                Message::Pong(_) => trace!("Pong!"),
                Message::Close(frame) => {
                    if let Some(frame) = frame {
                        error!("Gateway closed: {:?}", frame);
                    }

                    break;
                }
            }
        }

        Ok(())
    }

    async fn send_gateway_message(&mut self, message: OutgoingGatewayMessage) -> Result<()> {
        debug!("Sending: {:#?}", &message);

        let content = serde_json::to_string(&message)?;

        trace!("Sending JSON: {:?}", &content);

        self.stream.send(Message::Text(content)).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct OutgoingGatewayMessage {
    #[serde(rename = "op")]
    pub opcode: u8,
    #[serde(rename = "d")]
    pub data: OutgoingGatewayData,
}

#[derive(Debug, Deserialize)]
struct IncomingGatewayMessage {
    #[serde(rename = "op")]
    pub opcode: u8,
    #[serde(rename = "d")]
    pub data: IncomingGatewayData,
    #[serde(rename = "t")]
    pub name: Option<String>,
    #[serde(rename = "s")]
    pub seq: Option<u64>,
}

impl Into<OutgoingGatewayMessage> for OutgoingGatewayData {
    fn into(self) -> OutgoingGatewayMessage {
        match self {
            OutgoingGatewayData::Identify { .. } => OutgoingGatewayMessage {
                opcode: 2,
                data: self,
            },
        }
    }
}

// FIXME: BETTER PAYLOAD (DE)SERIALIZATION
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum OutgoingGatewayData {
    Identify {
        token: String,
        properties: IdentifyProperties,
        intents: u16,
    },
    Heartbeat(u64),
    HeartbeatAck
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum IncomingGatewayData {
    Hello { heartbeat_interval: u32 },
    Heartbeat(u64),
    HeartbeatAck
}

#[derive(Debug, Serialize)]
struct IdentifyProperties {
    #[serde(rename = "$os")]
    os: String,
    #[serde(rename = "$browser")]
    browser: String,
    #[serde(rename = "$device")]
    device: String,
}

/// The bitfield of intents to use
#[allow(unused)]
mod intents {
    /// - GUILD_CREATE
    /// - GUILD_UPDATE
    /// - GUILD_DELETE
    /// - GUILD_ROLE_CREATE
    /// - GUILD_ROLE_UPDATE
    /// - GUILD_ROLE_DELETE
    /// - CHANNEL_CREATE
    /// - CHANNEL_UPDATE
    /// - CHANNEL_DELETE
    /// - CHANNEL_PINS_UPDATE
    pub const GUILDS: u16 = 1 << 0;

    /// - GUILD_MEMBER_ADD
    /// - GUILD_MEMBER_UPDATE
    /// - GUILD_MEMBER_REMOVE
    pub const GUILD_MEMBERS: u16 = 1 << 1;

    /// - GUILD_BAN_ADD
    /// - GUILD_BAN_REMOVE
    pub const GUILD_BANS: u16 = 1 << 2;

    /// - GUILD_EMOJIS_UPDATE
    pub const GUILD_EMOJIS: u16 = 1 << 3;

    /// - GUILD_INTEGRATIONS_UPDATE
    pub const GUILD_INTEGRATIONS: u16 = 1 << 4;

    /// - WEBHOOKS_UPDATE
    pub const GUILD_WEBHOOKS: u16 = 1 << 5;

    /// - INVITE_CREATE
    /// - INVITE_DELETE
    pub const GUILD_INVITES: u16 = 1 << 6;

    /// - VOICE_STATE_UPDATE
    pub const GUILD_VOICE_STATES: u16 = 1 << 7;

    /// - PRESENCE_UPDATE
    pub const GUILD_PRESENCES: u16 = 1 << 8;

    /// - MESSAGE_CREATE
    /// - MESSAGE_UPDATE
    /// - MESSAGE_DELETE
    /// - MESSAGE_DELETE_BULK
    pub const GUILD_MESSAGES: u16 = 1 << 9;

    /// - MESSAGE_REACTION_ADD
    /// - MESSAGE_REACTION_REMOVE
    /// - MESSAGE_REACTION_REMOVE_ALL
    /// - MESSAGE_REACTION_REMOVE_EMOJI
    pub const GUILD_MESSAGE_REACTIONS: u16 = 1 << 10;

    /// - TYPING_START
    pub const GUILD_MESSAGE_TYPING: u16 = 1 << 11;

    /// - CHANNEL_CREATE
    /// - MESSAGE_CREATE
    /// - MESSAGE_UPDATE
    /// - MESSAGE_DELETE
    /// - CHANNEL_PINS_UPDATE
    pub const DIRECT_MESSAGES: u16 = 1 << 12;

    /// - MESSAGE_REACTION_ADD
    /// - MESSAGE_REACTION_REMOVE
    /// - MESSAGE_REACTION_REMOVE_ALL
    /// - MESSAGE_REACTION_REMOVE_EMOJI
    pub const DIRECT_MESSAGE_REACTIONS: u16 = 1 << 13;

    /// - TYPING_START
    pub const DIRECT_MESSAGE_TYPING: u16 = 1 << 14;
}
