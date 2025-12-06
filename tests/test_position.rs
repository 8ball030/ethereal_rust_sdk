mod common;
use ethereal_rust_sdk::apis::position_api::{
        PositionControllerGetActiveParams, PositionControllerGetByIdParams,
        PositionControllerListBySubaccountIdParams, PositionControllerListFillsByPositionIdParams,
        PositionControllerListLiquidationsBySubaccountIdParams,
    };

#[test]
fn test_get_active() {
    let client = common::create_test_client().unwrap();
    let subaccount_id = client.subaccounts.first().unwrap().id.clone().to_string();
    let product_id = common::get_product(&client).unwrap().id;
    let params = PositionControllerGetActiveParams {
        subaccount_id,
        product_id: product_id.to_string(),
    };
    let result = client.position().get_active(params);
    assert!(result.is_ok());
}

#[test]
fn test_get_by_id() {
    let client = common::create_test_client().unwrap();
    let subaccount_id = client.subaccounts.first().unwrap().id.clone().to_string();
    let product_id = common::get_product(&client).unwrap().id;

    let params = PositionControllerGetActiveParams {
        subaccount_id,
        product_id: product_id.to_string(),
    };
    let result = client.position().get_active(params);
    let params = PositionControllerGetByIdParams {
        id: result.unwrap().id.to_string(),
    };
    let result = client.position().get_by_id(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let subaccount_id = client.subaccounts.first().unwrap().id.clone().to_string();

    let params = PositionControllerListBySubaccountIdParams {
        subaccount_id,
        ..Default::default()
    };
    let result = client.position().list_by_subaccount_id(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_fills_by_position_id() {
    let client = common::create_test_client().unwrap();
    let subaccount_id = client.subaccounts.first().unwrap().id.clone().to_string();
    let product_id = common::get_product(&client).unwrap().id;
    let params = PositionControllerGetActiveParams {
        subaccount_id,
        product_id: product_id.to_string(),
    };
    let result = client.position().get_active(params);
    let params = PositionControllerListFillsByPositionIdParams {
        position_id: result.unwrap().id.to_string(),
        ..Default::default()
    };
    let result = client.position().list_fills_by_position_id(params);
    print!("Result: {result:?}");
    assert!(result.is_ok());
}

#[test]
fn test_list_liquidations_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = PositionControllerListLiquidationsBySubaccountIdParams {
        ..Default::default()
    };
    let result = client.position().list_liquidations_by_subaccount_id(params);
    assert!(result.is_ok());
}
