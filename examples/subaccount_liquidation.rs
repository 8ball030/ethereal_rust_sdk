mod common;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::models::SubaccountLiquidationMessage;

use ethereal_rust_sdk::ws_client::ConnectionState;
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
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params).await?;
    let subaccount_ids = subaccounts
        .data
        .iter()
        .map(|subaccount| subaccount.id.to_string())
        .collect::<Vec<_>>();

    ws_client
        .subscriptions()
        .subaccount_liquidation(subaccount_ids, subaccount_liquidation_callback)
        .await?;

    info!("Starting event loop...");
    ws_client.wait_for_connection().await;
    loop {
        match ws_client.run_till_event().await {
            ConnectionState::Connected => {
                info!("WebSocket connected");
                ws_client.resubscribe_all().await.unwrap();
            }
            ConnectionState::Disconnected => {
                info!("WebSocket disconnected");
            }
            ConnectionState::Exited => {
                info!("WebSocket exited");
                break;
            }
            ConnectionState::Reconnecting => {
                info!("WebSocket reconnecting...");
            }
        }
    }

    Ok(())
}
