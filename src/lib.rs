//! [WORK IN PROGRESS] tonapi.rs rust SDK

mod clients;
mod error;
mod net;
mod server;

#[cfg(feature = "ws")]
/// WebSocket tonapi module
pub mod ws {
    pub use crate::clients::websocket::{
        new_websocket_client, WebSocketConfig, WebSocketSplitReadClient, WebSocketSplitWriteClient,
    };
    pub use crate::net::ws::{
        WebSocketFacade, WebSocketFacadeConfig, WebSocketReadFacade, WebSocketSplitFacade,
        WebSocketWriteFacade,
    };

    /// Methods for websocket API
    pub mod methods {
        pub use crate::clients::websocket::{
            SubscribeTrace, WebSocketMethod, WebSocketMethodRequestBody,
        };
    }

    #[cfg(feature = "tungstenite-ws")]
    pub use crate::net::ws::tungstenite::{
        TungsteniteFacade, TungsteniteReadFacade, TungsteniteWriteFacade,
    };
}

/// Common
pub mod common {
    pub use crate::error::{TonApiError, TonApiResult};
    pub use crate::server::Server;
}
