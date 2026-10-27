mod common;
use ethereal_rust_sdk::apis::usde_api::UsdeControllerListTransfersParams;

#[tokio::test]
async fn test_list_transfers() {
    let client = common::create_test_client().await.unwrap();
    let params = UsdeControllerListTransfersParams {
        address: client.address.clone(),
        ..Default::default()
    };
    let result = client.usde().list_transfers(params).await;
    assert!(result.is_ok());
}
