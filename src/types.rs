use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Bytes, Message},
};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type ResponseSender = oneshot::Sender<Bytes>;
pub type ChannelSender = tokio::sync::mpsc::UnboundedSender<Bytes>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = core::result::Result<T, Error>;

/// ErrorResponseError : Error code and message.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseError {
    #[serde(rename = "code")]
    pub code: u64,
    #[serde(rename = "message")]
    pub message: String,
}

impl ErrorResponseError {
    /// Error code and message.
    pub fn new(code: u64, message: String) -> ErrorResponseError {
        ErrorResponseError { code, message }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RpcErrorResponse {
    /// Your request id, or null if not supplied.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseError>,
}

impl RpcErrorResponse {
    pub fn new() -> RpcErrorResponse {
        RpcErrorResponse {
            id: None,
            error: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RpcMessage {
    pub id: Option<u64>,
    pub result: Value,
    pub error: Option<RpcErrorResponse>,
}

pub enum InternalCommand {
    Send(Message),
    Close,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum ExternalEvent {
    Connected,
    Disconnected,
    Exited,
}

#[derive(Debug, Serialize)]
pub struct ProductSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}
#[derive(Debug, Serialize)]
pub struct SubaccountSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(rename = "subaccountId")]
    pub subaccount_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SubscribeResponse {
    Ok { id: u64, result: Vec<String> },
    Err { id: u64, error: RpcErrorResponse },
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("RPC error: {0:?}")]
    Rpc(RpcErrorResponse),

    #[error("transport error")]
    Transport(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("JSON parse error")]
    Parse(#[from] serde_json::Error),

    #[error("oneshot receive error")]
    Recv(#[from] oneshot::error::RecvError),
}
