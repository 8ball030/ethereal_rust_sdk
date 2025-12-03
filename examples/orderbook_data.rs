use ethereal_streamer::models::BookDepthMessage;
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

use ethereal_streamer::enums::Environment;
use ethereal_streamer::ws_client::WsClient;

fn orderbook_callback(market_price: Payload, _socket: RawClient) {
    if let Payload::Text(values) = market_price {
        for value in values {
            if let Ok(orderbook) = serde_json::from_value::<BookDepthMessage>(value) {
                println!(
                    "Orderbook Update - Product ID: {:?}, Bids: {:?}, Asks: {:?}",
                    orderbook.product_id, orderbook.bids, orderbook.asks
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting products...");
    let products = ethereal_streamer::async_client::get_products()?;
    println!("Starting WS Client.");

    let mut ws_client = WsClient::new(Environment::Testnet);

    println!("Registering orderbook callback...");
    ws_client.register_orderbook_callback(orderbook_callback);

    println!("Connecting WS Client...");
    ws_client.connect()?;

    println!("Subscribing to orderbook data for products...");
    products.iter().for_each(|product| {
        ws_client.subscribe_orderbook_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}
