mod common;

use ethereal_rust_sdk::apis::token_api::{
    TokenControllerGetByIdParams, TokenControllerInitiateWithdrawParams, TokenControllerListParams,
    TokenControllerListTransfersParams, TokenControllerListWithdrawsParams,
};

#[tokio::test]
#[ignore]
async fn test_get_by_id() {
    let client = common::create_test_client().await.unwrap();
    let params = TokenControllerGetByIdParams {
        id: "1".to_string(),
    };
    let result = client.token().get_by_id(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_initiate_withdraw() {
    let client = common::create_test_client().await.unwrap();
    let params = TokenControllerInitiateWithdrawParams::default();
    let result = client.token().initiate_withdraw(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list() {
    let client = common::create_test_client().await.unwrap();
    let params = TokenControllerListParams::default();
    let result = client.token().list(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_transfers() {
    let client = common::create_test_client().await.unwrap();
    let params = TokenControllerListTransfersParams {
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };

    let result = client.token().list_transfers(params).await;
    println!("Result: {result:?}");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_withdraws() {
    let client = common::create_test_client().await.unwrap();
    let params = TokenControllerListWithdrawsParams {
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client.token().list_withdraws(params).await;
    assert!(result.is_ok());
}
