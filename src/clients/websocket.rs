use crate::clients::websocket::ReceiveResult::Raw;
use crate::error::TonApiResult;
use crate::net::ws::{
    WebSocketFacade, WebSocketFacadeConfig, WebSocketReadFacade, WebSocketSplitFacade,
    WebSocketWriteFacade,
};
use crate::server::Server;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
use tonlib_core::TonAddress;

static TONAPI_MAINNET_BASE_URL: &str = "wss://tonapi.io/v2/websocket";
static TONAPI_TESTNET_BASE_URL: &str = "wss://testnet.tonapi.io/v2/websocket";

pub struct WebSocketConfig {
    pub server: Server,
}

pub async fn new_websocket_client<F: WebSocketFacade + WebSocketSplitFacade>(
    config: WebSocketConfig,
) -> TonApiResult<(
    WebSocketSplitReadClient<F::Read>,
    WebSocketSplitWriteClient<F::Write>,
)> {
    let facade = F::connect(WebSocketFacadeConfig {
        url: match config.server {
            Server::MainNet => TONAPI_MAINNET_BASE_URL
                .parse()
                .expect("TONAPI_MAINNET_BASE_URL invalid"),
            Server::TestNet => TONAPI_TESTNET_BASE_URL
                .parse()
                .expect("TONAPI_TESTNET_BASE_URL invalid"),
            Server::Custom(url) => url,
        },
    })
    .await?;
    let (read, write) = facade.split().unwrap();
    Ok((
        WebSocketSplitReadClient { read_facade: read },
        WebSocketSplitWriteClient {
            write_facade: write,
            increment: AtomicU64::new(0),
        },
    ))
}

pub struct WebSocketSplitReadClient<F: WebSocketReadFacade> {
    read_facade: F,
}

impl<F: WebSocketReadFacade> WebSocketSplitReadClient<F> {
    pub async fn recv(&mut self) -> TonApiResult<Option<ReceiveResult>> {
        if let Some(message_body) = self.read_facade.recv().await? {
            match serde_json::from_str::<WebSocketMessage>(&message_body) {
                Ok(message) => Ok(Some(ReceiveResult::Message(message))),
                Err(_) => Ok(Some(Raw(message_body))),
            }
        } else {
            Ok(None)
        }
    }
}

pub struct WebSocketSplitWriteClient<F: WebSocketWriteFacade> {
    increment: AtomicU64,
    write_facade: F,
}

impl<F: WebSocketWriteFacade> WebSocketSplitWriteClient<F> {
    pub async fn execute(&mut self, method: impl WebSocketMethod) -> TonApiResult<()> {
        let method_body = WebSocketMethodRequestBody {
            id: self.next_id(),
            jsonrpc: method.jsonrpc(),
            method: method.method(),
            params: method.params(),
        };
        self.send(method_body).await?;
        Ok(())
    }

    pub async fn send(&mut self, message: impl Serialize) -> TonApiResult<()> {
        let message_str = serde_json::to_string(&message).unwrap();
        self.write_facade.send(message_str).await?;
        Ok(())
    }

    pub fn next_id(&mut self) -> u64 {
        self.increment
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug)]
/// Receive message types
pub enum ReceiveResult {
    /// Parsed websocket message. When crate could identify message
    Message(WebSocketMessage),
    /// Raw websocket message. When crate could not identify message
    Raw(String),
}

#[derive(Debug, Deserialize)]
/// Parameters of trace websocket event
pub struct TraceParams {
    pub accounts: Vec<String>,
    pub hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum WebSocketMessage {
    SubscribeTrace {
        id: Option<u64>,
        jsonrpc: String,
        result: String,
    },
    Trace {
        jsonrpc: String,
        params: TraceParams,
    },
}

#[derive(Serialize)]
pub struct WebSocketMethodRequestBody {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
}

pub trait WebSocketMethod {
    fn jsonrpc(&self) -> String {
        "2.0".to_string()
    }
    fn method(&self) -> String;
    fn params(&self) -> Vec<String>;
}

pub struct SubscribeTrace {
    pub accounts: Vec<TonAddress>,
}

impl WebSocketMethod for SubscribeTrace {
    fn method(&self) -> String {
        "subscribe_trace".to_string()
    }

    fn params(&self) -> Vec<String> {
        let result = self.accounts.iter().map(|a| a.to_string()).collect();
        println!("{:?}", result);
        result
    }
}
