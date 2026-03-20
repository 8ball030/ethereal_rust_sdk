mod common;

use ethereal_rust_sdk::models::TickerMessage;
use log::info;

async fn ticker_callback(msg: TickerMessage) {
    info!("Received ticker message: {:?}", msg);
}

#[tokio::main]
async fn main() {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Info).expect("log");
    let (http_client, ws_client) = common::create_test_clients().await.unwrap();
    info!("HTTP client and WS client created");
    let tickers = common::get_product_tickers(&http_client).await.unwrap();

    ws_client
        .subscriptions()
        .ticker(tickers, ticker_callback)
        .await
        .unwrap();

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
}
