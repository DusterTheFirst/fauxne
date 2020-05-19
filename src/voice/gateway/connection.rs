use async_native_tls::TlsStream;
use async_std::{net::TcpStream, prelude::FutureExt};
use async_tungstenite::{stream::Stream, tungstenite::Message, WebSocketStream};
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    future::{self, Either},
    sink::SinkExt,
    stream::StreamExt,
};
use std::time::Duration;

type GatewayStream = WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;

pub struct GatewayConnection {
    rx: UnboundedReceiver<Message>,
    pub stream: GatewayStream,
    tx: UnboundedSender<Message>,
}

impl GatewayConnection {
    pub fn new(
        stream: GatewayStream,
    ) -> (Self, UnboundedReceiver<Message>, UnboundedSender<Message>) {
        let (to_user, from_forwarder) = mpsc::unbounded();
        let (to_forwarder, from_user) = mpsc::unbounded();

        (
            Self {
                rx: from_user,
                stream,
                tx: to_user,
            },
            from_forwarder,
            to_forwarder,
        )
    }

    pub async fn run(mut self) {
        const TIMEOUT: Duration = Duration::from_secs(90);

        debug!("[GatewayConnection] Starting driving loop");
        loop {
            match future::select(self.rx.next(), self.stream.next().timeout(TIMEOUT)).await {
                Either::Left((Some(msg), _)) => {
                    trace!("[GatewayConnection] Sending msg: {}", msg);
                    if let Err(err) = self.stream.send(msg).await {
                        warn!("[GatewayConnection] Got error when sending: {}", err);
                        break;
                    }
                }
                Either::Left((None, _)) => {
                    warn!("[GatewayConnection] Got None, closing stream");
                    let _ = self.stream.close(None).await;

                    break;
                }
                Either::Right((Ok(Some(Ok(msg))), _)) => {
                    if self.tx.unbounded_send(msg).is_err() {
                        break;
                    }
                }
                Either::Right((Ok(Some(Err(err))), _)) => {
                    warn!("[GatewayConnection] Got error: {}, closing tx", err);
                    self.tx.close_channel();
                    break;
                }
                Either::Right((Ok(None), _)) => {
                    warn!("[GatewayConnection] Got None, closing tx");
                    self.tx.close_channel();
                    break;
                }
                Either::Right((Err(why), _)) => {
                    warn!("[GatewayConnection] Error: {}", why);
                    self.tx.close_channel();
                    break;
                }
            }
        }
        warn!("[GatewayConnection] Leaving loop");
    }
}
