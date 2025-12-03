use rust_socketio::client::RawClient;
use rust_socketio::Payload;

use ethereal_streamer::async_client::get_products;
use ethereal_streamer::enums::Environment;
use ethereal_streamer::models::TradeStreamMessage;
use ethereal_streamer::ws_client::WsClient;

fn trade_fill_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            match serde_json::from_value::<TradeStreamMessage>(value) {
                Err(e) => {
                    eprintln!("Failed to deserialize TradeStreamMessage: {e}");
                    return;
                }
                Ok(trade) => {
                    // Successfully deserialized, proceed
                    for fill in trade.data {
                        println!(
                            "Trade Fill - Product ID: {:?}, Price: {:?}, Quantity: {:?}",
                            trade.product_id, fill.price, fill.filled
                        );
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting products...");
    let env = Environment::Production;
    let products = get_products(env.clone())?;
    println!("Starting WS Client.");

    let mut ws_client = WsClient::new(env);

    println!("Registering trade fill callback...");
    ws_client.register_trade_fill_callback(trade_fill_callback);

    println!("Connecting WS Client...");
    ws_client.connect()?;

    println!("Subscribing to trade fill data for products...");
    products.iter().for_each(|product| {
        ws_client.subscribe_trade_fill_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}
