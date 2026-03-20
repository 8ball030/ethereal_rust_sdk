mod common;
use ethereal_rust_sdk::models::OrderFillMessage;

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
    let subaccount_ids = common::get_subaccount_ids(&http_client).await?;

    ws_client
        .subscriptions()
        .order_fill(subaccount_ids, order_fill_callback)
        .await?;

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
    Ok(())
}
