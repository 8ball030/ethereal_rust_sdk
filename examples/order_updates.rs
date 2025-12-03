use ethereal_streamer::async_client::get_subaccounts;
use ethereal_streamer::enums::Environment;
use ethereal_streamer::models::PageOfOrderFillDtos;
use ethereal_streamer::ws_client::WsClient;

use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn order_fill_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            if let Ok(page) = serde_json::from_value::<PageOfOrderFillDtos>(value.clone()) {
                for fill in page.data {
                    println!(
                        "Order Fill - ID: {}, Product ID: {}, Price: {}, Side: {:?}",
                        fill.id, fill.product_id, fill.price, fill.side,
                    );
                }
            } else {
                eprintln!("Failed to deserialize order fill data: {value}");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("We retrieve the sender address from the environment variable SENDER_ADDRESS");
    // We raise if the variable is not set
    let sender_address = std::env::var("SENDER_ADDRESS").unwrap_or_else(|_| {
        panic!("SENDER_ADDRESS environment variable is not set");
    });
    println!("Getting subaccounts...");
    let env = Environment::Testnet;
    let subaccounts = get_subaccounts(env.clone(), sender_address.as_str())?;

    println!("Subaccounts: {subaccounts:?}");

    let mut ws_client = WsClient::new(env);
    println!("Connecting WS Client...");

    ws_client.register_order_fill_callback(order_fill_callback);
    ws_client.connect()?;
    println!("Subscribing to transfer events for subaccounts...");
    subaccounts.iter().for_each(|subaccount| {
        ws_client.subscribe_order_fill(&subaccount.id.to_string());
    });
    ws_client.run_forever();

    Ok(())
}
