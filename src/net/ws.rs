use crate::error::TonApiResult;
use url::Url;

#[cfg(feature = "tungstenite-ws")]
pub mod tungstenite;

pub struct WebSocketFacadeConfig {
    pub url: Url,
}

#[async_trait::async_trait]
/// Base trait for websocket clients support
pub trait WebSocketFacade {
    /// Connection to websocket
    async fn connect(config: WebSocketFacadeConfig) -> TonApiResult<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
pub trait WebSocketSplitFacade {
    type Read: WebSocketReadFacade;
    type Write: WebSocketWriteFacade;

    fn split(self) -> TonApiResult<(Self::Read, Self::Write)>;
}

#[async_trait::async_trait]
pub trait WebSocketWriteFacade {
    /// Sending message
    async fn send(&mut self, message: String) -> TonApiResult<()>;
}

#[async_trait::async_trait]
pub trait WebSocketReadFacade {
    /// Awaiting new message
    async fn recv(&mut self) -> TonApiResult<Option<String>>;
}
