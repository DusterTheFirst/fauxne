use super::gateway::{
    intents::Intent,
    message::outgoing::{IdentifyProperties, OutgoingGatewayData, OutgoingGatewayMessage},
    message::{
        incoming::{Event, IncomingGatewayData, IncomingGatewayMessage},
        model::{
            activity::{Activity, Status},
            text::Emoji,
        },
    },
};
use crate::voice::gateway::message::model::user::User;
use async_native_tls::TlsStream;
use async_std::{
    net::TcpStream,
    sync::{Mutex, RwLock},
    task,
};
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
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
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
    last_seq: AtomicU64,
    heartbeating: AtomicBool,
    user: RwLock<Option<User>>,
    session_id: RwLock<Option<String>>,
}

impl VoiceClient {
    const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=6&encoding=json";

    pub async fn connect() -> Result<Arc<Self>> {
        let (stream, _) = connect_async(Self::GATEWAY).await?;

        let (tx, rx) = stream.split();

        Ok(Arc::new(VoiceClient {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
            last_seq: AtomicU64::new(0),
            heartbeating: AtomicBool::new(false),
            user: RwLock::new(None),
            session_id: RwLock::new(None),
        }))
    }

    pub async fn disconnect(self: Arc<Self>) -> Result<()> {
        self.tx
            .lock()
            .await
            .send(Message::Close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "".into(),
            })))
            .await?;

        Ok(())
    }

    pub async fn login(self: Arc<Self>, token: &str) -> Result<()> {
        self.clone().identify(token).await?;

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
                                self.last_seq.store(seq.into(), Ordering::Release);
                            }

                            if let Err(e) = self.clone().handle_message(incoming).await {
                                error!(
                                    "VoiceClient encountered an error handling a message: {:?}",
                                    e
                                );
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

    async fn handle_message(self: Arc<Self>, message: IncomingGatewayMessage) -> Result<()> {
        match message.data {
            IncomingGatewayData::Hello(hello) => {
                if self.heartbeating.load(Ordering::Relaxed) {
                    warn!("Server sent multiple hello packets, ignoring: {:?} ", hello);
                }
                debug!(
                    "Starting heartbeating with an interval of {}ms",
                    hello.heartbeat_interval
                );

                self.heartbeating.store(true, Ordering::Relaxed);
                self.clone().run_heartbeat(hello.heartbeat_interval);
            }
            IncomingGatewayData::Heartbeat(seq) => {
                trace!("Server sent Heartbeat({:?}), responding", seq);
                self.send(OutgoingGatewayData::HeartbeatAck).await?;
            }
            IncomingGatewayData::HeartbeatAck => {
                trace!("Server sent Heartbeat ACK");
            }
            IncomingGatewayData::Dispatch(event) => match event {
                Event::Ready(data) => {
                    info!("Gateway Ready!");
                    debug!("{:#?}", data);

                    self.session_id.write().await.replace(data.session_id);
                    self.user.write().await.replace(data.user);

                    self.send(OutgoingGatewayData::PresenceUpdate {
                        afk: false,
                        game: Some(Activity::custom("Waiting for a call", Emoji::named("☎️"))),
                        since: Some(0),
                        status: Status::Online,
                    })
                    .await?;
                }
                Event::CallUpdate(call) => {
                    info!("Call has been updated: {:#?}", call);

                    if let Some(user) = self.user.read().await.as_ref() {
                        if call.ringing.contains(&user.id) {
                            info!("BEING CALLED!!!!");

                            self.clone()
                                .send(OutgoingGatewayData::VoiceStateUpdate {
                                    channel_id: call.channel_id,
                                    guild_id: call.guild_id,
                                    self_deaf: false,
                                    self_mute: false,
                                })
                                .await?;
                        }
                    } else {
                        warn!("Call update event has been sent before the client has recieved the ready packet. Something might be wrong");
                    }
                }
                Event::CallDelete(call) => info!("Call has been deleted: {:#?}", call),
                Event::VoiceStateUpdate(state) => {
                    info!("Voice state has been updated: {:#?}", state)
                }
                Event::Muted(event) => trace!("Ignored muted event: {:?}", event),
                Event::Unknown(event, data) => warn!(
                    "Server dispatched an unknown event: \"{}\"\n{}",
                    event,
                    serde_json::to_string_pretty(&data)?
                ),
            },
            IncomingGatewayData::InvalidSession(resumable) => {
                warn!("Invalid session, Resumable: {}", resumable);
                todo!();
            }
            IncomingGatewayData::Reconnect => {
                warn!("Gateway sent reconnect event");
                todo!();
            }
            IncomingGatewayData::Unknown(_) => {
                warn!("Server sent unknown data: {:?}", message);
            }
        }

        Ok(())
    }

    fn run_heartbeat(self: Arc<Self>, interval: u64) {
        task::spawn(async move {
            loop {
                task::sleep(Duration::from_millis(interval)).await;

                // Retry until heartbeat sent
                loop {
                    let seq_id = self.last_seq.load(Ordering::Acquire);

                    trace!("Sending heartbeat with seq {:?}", seq_id);

                    if let Err(e) = self
                        .send(OutgoingGatewayData::Heartbeat(if seq_id == 0 {
                            None
                        } else {
                            Some(seq_id)
                        }))
                        .await
                    {
                        error!("Failed to send heartbeat, retrying: {:?}", e);
                    } else {
                        break;
                    }
                }
            }
        });
    }

    async fn identify(self: Arc<Self>, token: &str) -> Result<()> {
        self.send(OutgoingGatewayData::Identify {
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
            intents: Intent::DIRECT_MESSAGES,
        })
        .await?;

        Ok(())
    }

    async fn send(self: &Arc<Self>, data: OutgoingGatewayData) -> Result<()> {
        if let OutgoingGatewayData::Heartbeat(_) = data {
        } else {
            debug!("Sending: {:#?}", &data);
        }

        let message = OutgoingGatewayMessage::from(data);

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
