use ethereal_rust_sdk::async_client::get_subaccounts;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::SubaccountLiquidation;
use ethereal_rust_sdk::ws_client::WsClient;

use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn liquidation_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            if let Ok(liquidation) = serde_json::from_value::<SubaccountLiquidation>(value.clone())
            {
                println!(
                    "Subaccount liquidated - ID: {}, Liquidated At: {}",
                    liquidation.subaccount_id, liquidation.liquidated_at
                );
            } else {
                eprintln!("Failed to deserialize liquidation data: {value}");
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

    ws_client.register_subaccount_liquidation_callback(liquidation_callback);
    ws_client.connect()?;
    println!("Subscribing to transfer events for subaccounts...");
    subaccounts.iter().for_each(|subaccount| {
        ws_client.subscribe_subaccount_liquidation(&subaccount.id.to_string());
    });
    ws_client.run_forever();

    Ok(())
}
