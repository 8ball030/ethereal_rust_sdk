use bytes::Bytes;
use serde::Serialize;
use tokio::sync::oneshot;

use crate::channels::Channels;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: Channels,
    pub symbol: String,
}
impl From<ProductSubscriptionMessage> for Bytes {
    fn from(val: ProductSubscriptionMessage) -> Self {
        let msg: SubscriptionMessage<ProductSubscriptionMessage> = SubscriptionMessage {
            event: "subscribe".to_string(),
            data: val,
        };
        let json = serde_json::to_string(&msg).expect("json");
        Bytes::from(json)
    }
}
impl ProductSubscriptionMessage {
    pub fn get_channel_id(&self) -> String {
        format!("{:?}:{}", self.msg_type, self.symbol)
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountSubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: Channels,
    pub subaccount_id: String,
}
impl SubaccountSubscriptionMessage {
    pub fn get_channel_id(&self) -> String {
        format!("{:?}:{}", self.msg_type, self.subaccount_id)
    }
}
impl From<SubaccountSubscriptionMessage> for Bytes {
    fn from(val: SubaccountSubscriptionMessage) -> Self {
        let msg: SubscriptionMessage<SubaccountSubscriptionMessage> = SubscriptionMessage {
            event: "subscribe".to_string(),
            data: val,
        };
        let json = serde_json::to_string(&msg).expect("json");
        Bytes::from(json)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMessage<T> {
    pub event: String,
    pub data: T,
}

pub type ResponseSender = oneshot::Sender<Bytes>;
