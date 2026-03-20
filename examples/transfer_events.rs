mod common;
use ethereal_rust_sdk::models::TokenTransferMessage;

use log::info;

async fn token_transfer_callback(raw_data: TokenTransferMessage) {
    let data = raw_data.data;
    info!(
        "Token transfer @ ID: {}, Symbol: {}, Quantity: {:?} Type: {:?} Status: {:?}",
        data.id, data.t_name, data.amt, data.typ, data.st
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, ws_client) = common::create_test_clients().await?;
    let subaccount_ids = common::get_subaccount_ids(&http_client).await?;

    ws_client
        .subscriptions()
        .token_transfer(subaccount_ids, token_transfer_callback)
        .await?;

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
    Ok(())
}
