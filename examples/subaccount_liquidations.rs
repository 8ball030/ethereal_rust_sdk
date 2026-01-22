mod common;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::models::SubaccountLiquidation;

use log::info;

fn liquidation_callback(liquidation: SubaccountLiquidation) {
    info!(
        "Subaccount liquidated - ID: {}, Liquidated At: {}",
        liquidation.subaccount_id, liquidation.liquidated_at
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients()?;
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;

    ws_client.register_subaccount_liquidation_callback(liquidation_callback);
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_subaccount_liquidation(&subaccount.id.to_string());
    });
    ws_client.connect()?;
    ws_client.run_forever();

    Ok(())
}
