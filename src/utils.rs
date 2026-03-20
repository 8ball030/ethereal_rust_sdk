// use rust_socketio::{asynchronous::Client, Payload};

use crate::{async_client::client::HttpClient, enums::Environment, ws_client::WsClient};

pub async fn create_client(
    env: Environment,
    private_key: &str,
    owner_address: Option<String>,
) -> anyhow::Result<(HttpClient, WsClient)> {
    let http_client = HttpClient::new(env, private_key, owner_address).await;
    let ws_client = WsClient::new(env);
    Ok((http_client, ws_client))
}
