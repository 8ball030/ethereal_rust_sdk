mod common;
use ethereal_rust_sdk::ws_client::run_forever;
use log::info;

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::models::TradeStreamMessage;

fn trade_fill_callback(trade: TradeStreamMessage) {
    for fill in trade.data {
        info!(
            "Trade Fill - Product ID: {:?}, Price: {:?}, Quantity: {:?}",
            trade.product_id, fill.price, fill.filled
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients().await?;
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params).await?;

    ws_client.register_trade_fill_callback(trade_fill_callback);

    products.data.iter().for_each(|product| {
        ws_client.subscribe_trade_fill_data(&product.id.to_string());
    });
    ws_client.connect().await?;
    run_forever().await;
    Ok(())
}
