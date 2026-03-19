mod common;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::models::TransferDto;

use ethereal_rust_sdk::ws_client::run_forever;
use log::info;

async fn transfer_callback(raw_data: TransferDto) {
    info!(
        "Transfer Event - ID: {:?}, Status: {:?}, Token Name: {:?}, Amount: {:?}, Token Address: {:?}",
        raw_data.id, raw_data.status, raw_data.token_name, raw_data.amount, raw_data.token_address
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients().await?;
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params).await?;

    ws_client.register_transfer_callback(transfer_callback);
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_transfer_events(&subaccount.id.to_string());
    });
    ws_client.connect().await?;
    run_forever().await;

    Ok(())
}
