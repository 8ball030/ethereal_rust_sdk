use rust_socketio::client::RawClient;
use rust_socketio::Payload;

use ethereal_streamer::async_client::get_products;
use ethereal_streamer::enums::Environment;
use ethereal_streamer::models::MarketPriceDto;
use ethereal_streamer::ws_client::WsClient;

fn market_data_callback(market_price: Payload, _socket: RawClient) {
    if let Payload::Text(values) = market_price {
        for value in values {
            if let Ok(market_price) = serde_json::from_value::<MarketPriceDto>(value) {
                println!(
                    "Market Price Update - Product ID: {:?}, Best Bid: {:?}, Best Ask: {:?}",
                    market_price.product_id,
                    market_price.best_bid_price,
                    market_price.best_ask_price
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting products...");
    let env = Environment::Testnet;
    let products = get_products(env.clone())?;
    println!("Starting WS Client.");
    let mut ws_client = WsClient::new(env);

    println!("Registering market data callback...");
    ws_client.register_market_price_callback(market_data_callback);

    println!("Connecting WS Client...");
    ws_client.connect()?;

    println!("Subscribing to market data for products...");
    products.iter().for_each(|product| {
        ws_client.subscribe_market_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}
