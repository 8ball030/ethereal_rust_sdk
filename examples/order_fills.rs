mod common;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::models::OrderFillMessage;

use ethereal_rust_sdk::ws_client::ConnectionState;
use log::info;

async fn order_fill_callback(raw_data: OrderFillMessage) {
    let fill = raw_data.data;
    let orders = fill.d;

    for fill in orders {
        info!(
            "Order fill @ ID: {}, Symbol: {}, Price: {}, Side: {:?} Quantity: {:?}",
            fill.id, fill.s, fill.px, fill.sd, fill.sz
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
        .order_fill(subaccount_ids, order_fill_callback)
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
