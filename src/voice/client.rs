use super::gateway::{
    intents,
    message::incoming::{IncomingGatewayData, IncomingGatewayMessage},
    message::outgoing::{IdentifyProperties, OutgoingGatewayData, OutgoingGatewayMessage},
};
use async_native_tls::TlsStream;
use async_std::{net::TcpStream, sync::Mutex, task};
use async_tungstenite::{
    async_std::connect_async,
    stream::Stream,
    tungstenite::{
        self,
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    WebSocketStream,
};
use futures::prelude::*;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    sync::Arc,
    time::Duration,
};
use stream::{SplitSink, SplitStream};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred in serde_json")]
    JSON(#[from] serde_json::Error),
    #[error("An error occurred dealing with websockets")]
    WS(#[from] tungstenite::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

type GatewayTX = SplitSink<GatewayStream, Message>;
type GatewayRX = SplitStream<GatewayStream>;
type GatewayStream = WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;

pub struct VoiceClient {
    tx: Mutex<GatewayTX>,
    rx: Mutex<GatewayRX>,
}

impl VoiceClient {
    const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=6&encoding=json";

    pub async fn connect() -> Result<Arc<Self>> {
        let (stream, _) = connect_async(Self::GATEWAY).await?;

        let (tx, rx) = stream.split();

        Ok(Arc::new(VoiceClient {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }))
    }

    pub async fn disconnect(self: Arc<Self>) -> Result<()> {
        self.tx.lock().await.send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "".into()
        }))).await?;

        Ok(())
    }

    pub async fn login(self: Arc<Self>, token: &str) -> Result<()> {
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
                    browser: "Discord iOS".into(),
                    device: env!("CARGO_PKG_NAME").into(),
                },
                intents: intents::DIRECT_MESSAGES,
            }
            .into(),
        )
        .await?;

        let last_seq = Arc::new(AtomicU64::new(0));

        loop {
            let mut stream = self.rx.lock().await;
            let next = stream.next().await;
            let message = match next {
                Some(Ok(m)) => m.clone(),
                Some(Err(e)) => {
                    error!("Encountered error reading gateway stream.");

                    return Err(e.into());
                }
                None => break,
            };

            match message {
                Message::Text(json) => {
                    trace!("Recieved: {:?}", json);

                    match IncomingGatewayMessage::from_json(&json) {
                        Ok(incoming) => {
                            if let Some(seq) = incoming.seq {
                                last_seq.store(seq, Ordering::Release);
                            }

                            match incoming.data {
                                IncomingGatewayData::Hello(hello) => {
                                    debug!(
                                        "Starting heartbeating with an interval of {}ms",
                                        hello.heartbeat_interval
                                    );

                                    let seq_id = last_seq.clone();

                                    let heartbeat_self = self.clone();

                                    task::spawn(async move {
                                        loop {
                                            task::sleep(Duration::from_millis(
                                                hello.heartbeat_interval,
                                            ))
                                            .await;

                                            let seq_id = seq_id.load(Ordering::Acquire);

                                            debug!("Sending heartbeat with seq {:?}", seq_id);

                                            if let Err(e) = heartbeat_self.send_gateway_message(
                                                OutgoingGatewayData::Heartbeat(if seq_id == 0 {
                                                    None
                                                } else {
                                                    Some(seq_id)
                                                })
                                                .into(),
                                            )
                                            .await
                                            {
                                                error!("Failed to send heartbeat: {:?}", e);
                                            };
                                        }
                                    });
                                }
                                IncomingGatewayData::Heartbeat(nonce) => {
                                    trace!(
                                        "Server sent Heartbeat, responding with nonce: {}",
                                        nonce
                                    );
                                    todo!();
                                }
                                IncomingGatewayData::HeartbeatAck => {
                                    trace!("Server sent Heartbeat ACK");
                                }
                                IncomingGatewayData::Unknown(_) => {
                                    warn!("Server sent unknown data: {:?}", incoming);
                                }
                            }
                        }
                        Err(e) => error!("Failed to parse incoming json: {:?}\n{:?}", e, json),
                    }
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

    async fn send_gateway_message(self: &Arc<Self>, message: OutgoingGatewayMessage) -> Result<()> {
        debug!("Sending: {:#?}", &message);

        let content = if cfg!(debug_assertions) {
            message.to_json_pretty()?
        } else {
            message.to_json()?
        };

        trace!("Sending JSON: {}", &content);

        self.tx.lock().await.send(Message::Text(content)).await?;

        Ok(())
    }
}
