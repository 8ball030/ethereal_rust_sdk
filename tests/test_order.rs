mod common;
use ethereal_rust_sdk::apis::order_api::{
    OrderControllerCancelParams, OrderControllerDryRunParams, OrderControllerGetByIdParams,
    OrderControllerListBySubaccountIdParams, OrderControllerListFillsBySubaccountIdParams,
    OrderControllerListTradesParams, OrderControllerSubmitParams,
};

#[test]
#[ignore]
fn test_cancel() {
    let client = common::create_test_client().unwrap();
    let params = OrderControllerCancelParams::default();
    let result = client.order().cancel(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_dry_run() {
    let client = common::create_test_client().unwrap();
    let params = OrderControllerDryRunParams::default();
    let result = client.order().dry_run(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_by_id() {
    let client = common::create_test_client().unwrap();
    let params = OrderControllerGetByIdParams::default();
    let result = client.order().get_by_id(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = OrderControllerListBySubaccountIdParams {
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client.order().list_by_subaccount_id(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_fills_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = OrderControllerListFillsBySubaccountIdParams {
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client.order().list_fills_by_subaccount_id(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_trades() {
    let client = common::create_test_client().unwrap();
    let product = common::get_product(&client).unwrap();
    let params = OrderControllerListTradesParams {
        product_id: product.id.clone().to_string(),
        ..Default::default()
    };
    let result = client.order().list_trades(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_submit() {
    let client = common::create_test_client().unwrap();
    let params = OrderControllerSubmitParams::default();
    let result = client.order().submit(params);
    assert!(result.is_ok());
}
