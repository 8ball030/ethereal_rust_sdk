mod common;
use ethereal_rust_sdk::ws_client::run_forever;
use log::info;

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::models::BookDepthMessage;

fn orderbook_callback(raw_data: BookDepthMessage) {
    info!(
        "Orderbook Update - Product ID: {:?}, Bids: {:?}, Asks: {:?}",
        raw_data.product_id, raw_data.bids, raw_data.asks
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let (http_client, mut ws_client) = common::create_test_clients().await?;
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params).await?;

    ws_client.register_orderbook_callback(orderbook_callback);

    products.data.iter().for_each(|product| {
        ws_client.subscribe_orderbook_data(&product.id.to_string());
    });
    ws_client.connect().await?;
    run_forever().await;
    Ok(())
}
