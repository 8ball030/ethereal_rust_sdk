mod common;
use ethereal_rust_sdk::models::OrderUpdateMessage;

use log::info;

async fn order_update_callback(raw_data: OrderUpdateMessage) {
    let data = raw_data.data;
    let orders = data.d;

    for order in orders {
        info!(
            "Order update @ ID: {}, Symbol: {}, Price: {:?}, Side: {:?} Quantity: {:?} Status: {:?}",
            order.id, order.s, order.px, order.sd, order.qty, order.st
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
        .order_update(subaccount_ids, order_update_callback)
        .await?;

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
    Ok(())
}
