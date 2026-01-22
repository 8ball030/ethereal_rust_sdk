mod common;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::models::PageOfOrderFillDtos;

use log::info;

fn order_fill_callback(raw_data: PageOfOrderFillDtos) {
    for fill in raw_data.data {
        info!(
            "Order fill - ID: {}, Product ID: {}, Price: {}, Side: {:?} Quantity: {:?}",
            fill.id, fill.product_id, fill.price, fill.side, fill.filled
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients()?;
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;

    ws_client.register_order_fill_callback(order_fill_callback);
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_order_fill(&subaccount.id.to_string());
    });
    ws_client.connect()?;
    ws_client.run_forever();

    Ok(())
}
