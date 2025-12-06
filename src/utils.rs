use crate::{enums::Environment, sync_client::client::HttpClient, ws_client::WsClient};

pub fn create_client(
    env: Environment,
    private_key: &str,
) -> anyhow::Result<(HttpClient, WsClient)> {
    let http_client = HttpClient::new(env, private_key);
    let ws_client = WsClient::new(env);
    Ok((http_client, ws_client))
}
