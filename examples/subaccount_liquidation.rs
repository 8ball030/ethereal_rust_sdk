mod common;
use ethereal_rust_sdk::models::SubaccountLiquidationMessage;

use log::info;

async fn subaccount_liquidation_callback(raw_data: SubaccountLiquidationMessage) {
    let data = raw_data.data.d;
    for liq in data {
        info!(
            "Subaccount liquidated on Symbol: {}, Price: {}, Quantity: {}",
            liq.s, liq.px, liq.sz
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, ws_client) = common::create_test_clients().await?;
    let subaccount_ids = common::get_subaccount_ids(&http_client).await?;
    ws_client
        .subscriptions()
        .subaccount_liquidation(subaccount_ids, subaccount_liquidation_callback)
        .await?;

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
    Ok(())
}
