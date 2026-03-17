use std::ops::Sub;

use bytes::Bytes;
use serde::Serialize;
use tokio::sync::oneshot;

use crate::enums::Channels;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: Channels,
    pub symbol: String,
}
impl Into<Bytes> for ProductSubscriptionMessage {
    fn into(self) -> Bytes {
        let msg: SubscriptionMessage<ProductSubscriptionMessage> = SubscriptionMessage {
            event: "subscribe".to_string(),
            data: self,
        };
        let json = serde_json::to_string(&msg).expect("json");
        Bytes::from(json)
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: Channels,
    pub subaccount_id: String,
}
impl Into<Bytes> for SubaccountSubscriptionMessage {
    fn into(self) -> Bytes {
        let msg: SubscriptionMessage<SubaccountSubscriptionMessage> = SubscriptionMessage {
            event: "subscribe".to_string(),
            data: self,
        };
        let json = serde_json::to_string(&msg).expect("json");
        Bytes::from(json)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMessage<T>{
    pub event: String,
    pub data: T,
}

pub type ResponseSender = oneshot::Sender<String>;
