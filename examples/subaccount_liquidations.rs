use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::SubaccountLiquidation;
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::ws_client::WsClient;

use log::{error, info};
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn liquidation_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            if let Ok(liquidation) = serde_json::from_value::<SubaccountLiquidation>(value.clone())
            {
                info!(
                    "Subaccount liquidated - ID: {}, Liquidated At: {}",
                    liquidation.subaccount_id, liquidation.liquidated_at
                );
            } else {
                error!("Failed to deserialize liquidation data: {value}");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let sender_address = std::env::var("SENDER_ADDRESS").unwrap_or_else(|_| {
        panic!("SENDER_ADDRESS environment variable is not set");
    });
    let env = Environment::Testnet;
    let http_client = HttpClient::new(env);
    let params = SubaccountControllerListByAccountParams {
        sender: sender_address,
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;

    let mut ws_client = WsClient::new(env);

    ws_client.register_subaccount_liquidation_callback(liquidation_callback);
    ws_client.connect()?;
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_subaccount_liquidation(&subaccount.id.to_string());
    });
    ws_client.run_forever();

    Ok(())
}
