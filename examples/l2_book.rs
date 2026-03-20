mod common;

use ethereal_rust_sdk::{
    apis::product_api::ProductControllerListParams, models::L2BookMessage,
    ws_client::ConnectionState,
};
use log::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Info).expect("log");
    let (http_client, ws_client) = common::create_test_clients().await.unwrap();
    info!("HTTP client and WS client created");
    let params = ProductControllerListParams::default();
    info!("Fetching products with params: {params:?}");
    let products = http_client.product().list(params).await.unwrap().data;

    async fn l2_book_callback(msg: L2BookMessage) {
        info!("Received book message: {:?}", msg);
    }

    let tickers = products
        .iter()
        .map(|p| p.ticker.clone())
        .collect::<Vec<_>>();

    ws_client
        .subscriptions()
        .l2_book(tickers, l2_book_callback)
        .await
        .unwrap();

    info!("Starting event loop...");
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
