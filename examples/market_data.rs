use ethereal_rust_sdk::ws_client::ConnectionState;
use log::{error, info};
mod common;

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::models::MarketPriceDto;

async fn market_data_callback(market_price: MarketPriceDto) {
    info!(
        "Market Price Update - Product ID: {:?}, Best Bid: {:?}, Best Ask: {:?}",
        market_price.product_id, market_price.best_bid_price, market_price.best_ask_price
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients().await?;
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params).await.unwrap().data;

    ws_client.register_market_data_callback(market_data_callback);

    for product in products.iter() {
        ws_client.subscribe_market_data(&product.id.to_string());
    }

    ws_client.connect().await?;
    loop {
        match ws_client.run_till_event().await {
            ConnectionState::Connected => {
                info!("Called detects connected")
            }
            ConnectionState::Disconnected => {
                error!("State is disconncted!");
                break;
            }
            ConnectionState::Reconnecting => {
                error!("Client trying to reconnect!")
            }
        }
    }
    Ok(())
}
