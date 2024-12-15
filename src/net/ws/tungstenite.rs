use crate::error::TonApiResult;
use crate::net::ws::{
    WebSocketFacade, WebSocketFacadeConfig, WebSocketReadFacade, WebSocketSplitFacade,
    WebSocketWriteFacade,
};
use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub struct TungsteniteFacade(WebSocketStream<MaybeTlsStream<TcpStream>>);

#[async_trait::async_trait]
impl WebSocketFacade for TungsteniteFacade {
    async fn connect(config: WebSocketFacadeConfig) -> TonApiResult<Self> {
        let (socket, _response) = tokio_tungstenite::connect_async(config.url.to_string())
            .await
            .unwrap();

        Ok(Self(socket))
    }
}

impl WebSocketSplitFacade for TungsteniteFacade {
    type Read = TungsteniteReadFacade;
    type Write = TungsteniteWriteFacade;

    fn split(self) -> TonApiResult<(Self::Read, Self::Write)> {
        let (write, read) = self.0.split();
        Ok((TungsteniteReadFacade(read), TungsteniteWriteFacade(write)))
    }
}

pub struct TungsteniteReadFacade(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>);

#[async_trait::async_trait]
impl WebSocketReadFacade for TungsteniteReadFacade {
    async fn recv(&mut self) -> TonApiResult<Option<String>> {
        let msg = self.0.next().await.unwrap().unwrap();
        match msg {
            Message::Text(text) => Ok(Some(text)),
            Message::Binary(_) => Ok(Some("binary".to_string())),
            Message::Ping(_) => Ok(Some("ping".to_string())),
            Message::Pong(_) => Ok(Some("pong".to_string())),
            Message::Close(_) => Ok(Some("close".to_string())),
            Message::Frame(_) => Ok(Some("frame".to_string())),
        }
    }
}

pub struct TungsteniteWriteFacade(SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>);

#[async_trait::async_trait]
impl WebSocketWriteFacade for TungsteniteWriteFacade {
    async fn send(&mut self, message: String) -> TonApiResult<()> {
        self.0.send(Message::Text(message)).await.unwrap();
        Ok(())
    }
}
