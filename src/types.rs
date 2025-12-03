use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}
