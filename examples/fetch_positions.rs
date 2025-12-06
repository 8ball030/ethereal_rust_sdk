use ethereal_rust_sdk::apis::{
    position_api::PositionControllerListBySubaccountIdParams,
    subaccount_api::SubaccountControllerListByAccountParams,
};

mod common;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, _) = common::create_test_clients()?;
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;

    let positions = http_client.position().list_by_subaccount_id(
        PositionControllerListBySubaccountIdParams {
            subaccount_id: subaccounts.data.first().unwrap().id.to_string(),
            ..Default::default()
        },
    )?;
    println!("Positions: {positions:#?}");

    Ok(())
}
