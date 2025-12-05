mod common;
use ethereal_rust_sdk::apis::subaccount_api::{
    SubaccountControllerGetBySubaccountIdParams, SubaccountControllerListByAccountParams,
    SubaccountControllerListSubaccountBalancesParams,
};

#[test]
#[ignore]
fn test_get_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = SubaccountControllerGetBySubaccountIdParams::default();
    let result = client.subaccount().get_by_subaccount_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_by_account() {
    let client = common::create_test_client().unwrap();
    let params = SubaccountControllerListByAccountParams::default();
    let result = client.subaccount().list_by_account(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_subaccount_balances() {
    let client = common::create_test_client().unwrap();
    let params = SubaccountControllerListSubaccountBalancesParams::default();
    let result = client.subaccount().list_subaccount_balances(params);
    assert!(result.is_ok());
}
