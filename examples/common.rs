use ethereal_rust_sdk::{
    async_client::client::HttpClient, enums::Environment, utils::create_client, ws_client::WsClient,
};

pub async fn create_test_clients() -> anyhow::Result<(HttpClient, WsClient)> {
    let env = Environment::Testnet;
    let private_key = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a";
    let (http_client, ws_client) = create_client(env, private_key, None).await?;
    Ok((http_client, ws_client))
}

#[allow(dead_code)]
pub fn main() {
    // This file is only for common functions used in tests and examples.
}
