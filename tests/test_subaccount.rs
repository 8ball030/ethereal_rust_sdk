mod common;
use ethereal_rust_sdk::apis::subaccount_api::{
    SubaccountControllerGetBySubaccountIdParams, SubaccountControllerListByAccountParams,
    SubaccountControllerListSubaccountBalancesParams,
};

#[tokio::test]
async fn test_get_by_subaccount_id() {
    let client = common::create_test_client().await.unwrap();
    let params = SubaccountControllerGetBySubaccountIdParams {
        id: client.subaccounts.first().unwrap().id.to_string(),
    };
    let result = client.subaccount().get_by_subaccount_id(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_by_account() {
    let client = common::create_test_client().await.unwrap();
    let params = SubaccountControllerListByAccountParams {
        sender: client.address.clone(),
        ..Default::default()
    };
    let result = client.subaccount().list_by_account(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_subaccount_balances() {
    let client = common::create_test_client().await.unwrap();
    let params = SubaccountControllerListSubaccountBalancesParams {
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client.subaccount().list_subaccount_balances(params).await;
    assert!(result.is_ok());
}
