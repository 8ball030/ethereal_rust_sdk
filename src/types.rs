use serde::Serialize;

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
