mod common;
use ethereal_rust_sdk::apis::position_api::{
    PositionControllerGetActiveParams, PositionControllerGetByIdParams, PositionControllerListBySubaccountIdParams, PositionControllerListFillsByPositionIdParams,
    PositionControllerListLiquidationsBySubaccountIdParams,
};

#[test]
#[ignore]
fn test_get_active() {
    let client = common::create_test_client().unwrap();
    let params = PositionControllerGetActiveParams::default();
    let result = client.position().get_active(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_by_id() {
    let client = common::create_test_client().unwrap();
    let params = PositionControllerGetByIdParams::default();
    let result = client.position().get_by_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = PositionControllerListBySubaccountIdParams::default();
    let result = client.position().list_by_subaccount_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_fills_by_position_id() {
    let client = common::create_test_client().unwrap();
    let params = PositionControllerListFillsByPositionIdParams::default();
    let result = client.position().list_fills_by_position_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_liquidations_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = PositionControllerListLiquidationsBySubaccountIdParams::default();
    let result = client.position().list_liquidations_by_subaccount_id(params);
    assert!(result.is_ok());
}
