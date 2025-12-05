use log::{error, info};
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::TradeStreamMessage;
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::ws_client::WsClient;

fn trade_fill_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            match serde_json::from_value::<TradeStreamMessage>(value) {
                Err(e) => {
                    error!("Failed to deserialize TradeStreamMessage: {e}");
                    return;
                }
                Ok(trade) => {
                    // Successfully deserialized, proceed
                    for fill in trade.data {
                        info!(
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
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let env = Environment::Mainnet;

    let http_client = HttpClient::new(env.clone());
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params)?;
    let mut ws_client = WsClient::new(env);

    ws_client.register_trade_fill_callback(trade_fill_callback);
    ws_client.connect()?;

    products.data.iter().for_each(|product| {
        ws_client.subscribe_trade_fill_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}
