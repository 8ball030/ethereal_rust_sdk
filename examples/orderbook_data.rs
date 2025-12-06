mod common;
use log::info;
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::models::BookDepthMessage;

fn orderbook_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            if let Ok(orderbook) = serde_json::from_value::<BookDepthMessage>(value) {
                info!(
                    "Orderbook Update - Product ID: {:?}, Bids: {:?}, Asks: {:?}",
                    orderbook.product_id, orderbook.bids, orderbook.asks
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let (http_client, mut ws_client) = common::create_test_clients()?;
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params)?;

    ws_client.register_orderbook_callback(orderbook_callback);

    ws_client.connect()?;

    products.data.iter().for_each(|product| {
        ws_client.subscribe_orderbook_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}
