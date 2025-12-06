use log::info;
use rust_socketio::client::RawClient;
use rust_socketio::Payload;
mod common;

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::models::MarketPriceDto;

fn market_data_callback(market_price: Payload, _socket: RawClient) {
    if let Payload::Text(values) = market_price {
        for value in values {
            if let Ok(market_price) = serde_json::from_value::<MarketPriceDto>(value.clone()) {
                info!(
                    "Market Price Update - Product ID: {:?}, Best Bid: {:?}, Best Ask: {:?}",
                    market_price.product_id,
                    market_price.best_bid_price,
                    market_price.best_ask_price
                );
            } else {
                info!("Failed to deserialize MarketPriceDto: {value}");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients()?;
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params)?;

    ws_client.register_market_price_callback(market_data_callback);
    ws_client.connect()?;

    products.data.iter().for_each(|product| {
        println!(
            "Subscribing to market data for market: {} id: {}",
            product.ticker, product.id
        );
        ws_client.subscribe_market_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}
