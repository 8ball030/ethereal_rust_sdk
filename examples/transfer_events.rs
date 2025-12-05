use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::TransferDto;
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::ws_client::WsClient;

use log::info;
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn transfer_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            if let Ok(transfer) = serde_json::from_value::<TransferDto>(value) {
                info!(
                    "Transfer Event - ID: {:?}, Status: {:?}, Token Name: {:?}, Amount: {:?}, Token Address: {:?}",
                    transfer.id,
                    transfer.status,
                    transfer.token_name,
                    transfer.amount,
                    transfer.token_address
                );
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

    ws_client.register_transfer_callback(transfer_callback);
    ws_client.connect()?;
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_transfer_events(&subaccount.id.to_string());
    });
    ws_client.run_forever();

    Ok(())
}
