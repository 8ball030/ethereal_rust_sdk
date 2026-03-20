use ethereal_rust_sdk::{
    apis::{
        product_api::ProductControllerListParams,
        subaccount_api::SubaccountControllerListByAccountParams,
    },
    async_client::client::HttpClient,
    enums::Environment,
    utils::create_client,
    ws_client::{ConnectionState, WsClient},
};
use log::info;

#[allow(dead_code)]
pub async fn create_test_clients() -> anyhow::Result<(HttpClient, WsClient)> {
    let env = Environment::Testnet;
    let private_key = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a";
    let (http_client, ws_client) = create_client(env, private_key, None).await?;
    Ok((http_client, ws_client))
}

#[allow(dead_code)]
pub async fn get_subaccount_ids(http_client: &HttpClient) -> anyhow::Result<Vec<String>> {
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params).await?;
    let subaccount_ids = subaccounts
        .data
        .iter()
        .map(|subaccount| subaccount.id.to_string())
        .collect::<Vec<_>>();
    Ok(subaccount_ids)
}

#[allow(dead_code)]
pub async fn get_product_tickers(http_client: &HttpClient) -> anyhow::Result<Vec<String>> {
    let products = http_client
        .product()
        .list(ProductControllerListParams::default())
        .await?;
    let tickers = products
        .data
        .iter()
        .map(|product| product.ticker.clone())
        .collect::<Vec<_>>();
    Ok(tickers)
}
#[allow(dead_code)]
pub async fn run_forever(ws_client: &WsClient) {
    ws_client.wait_for_connection().await;
    loop {
        match ws_client.run_till_event().await {
            ConnectionState::Connected => {
                info!("WebSocket connected");
                ws_client.resubscribe_all().await.unwrap();
            }
            ConnectionState::Disconnected => {
                info!("WebSocket disconnected");
            }
            ConnectionState::Exited => {
                info!("WebSocket exited");
                break;
            }
            ConnectionState::Reconnecting => {
                info!("WebSocket reconnecting...");
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    // This file is only for common functions used in tests and examples.
}
