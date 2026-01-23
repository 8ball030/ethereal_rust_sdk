mod common;
use ethereal_rust_sdk::apis::linked_signer_api::{
    LinkedSignerControllerGetAccountQuotaParams, LinkedSignerControllerGetSignerParams,
    LinkedSignerControllerLinkSignerParams, LinkedSignerControllerListBySubaccountIdParams,
    LinkedSignerControllerRefreshSignerParams, LinkedSignerControllerRevokeSignerParams,
};

#[tokio::test]
#[ignore]
async fn test_get_account_quota() {
    let client = common::create_test_client().await.unwrap();
    let params = LinkedSignerControllerGetAccountQuotaParams::default();
    let result = client.linked_signer().get_account_quota(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_get_signer() {
    let client = common::create_test_client().await.unwrap();
    let params = LinkedSignerControllerGetSignerParams::default();
    let result = client.linked_signer().get_signer(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_link_signer() {
    let client = common::create_test_client().await.unwrap();
    let params = LinkedSignerControllerLinkSignerParams::default();
    let result = client.linked_signer().link_signer(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_revoke_signer() {
    let client = common::create_test_client().await.unwrap();
    let params = LinkedSignerControllerRevokeSignerParams::default();
    let result = client.linked_signer().revoke_signer(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_list_by_subaccount_id() {
    let client = common::create_test_client().await.unwrap();
    let params = LinkedSignerControllerListBySubaccountIdParams::default();
    let result = client.linked_signer().list_by_subaccount_id(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_refresh_signer() {
    let client = common::create_test_client().await.unwrap();
    let params = LinkedSignerControllerRefreshSignerParams::default();
    let result = client.linked_signer().refresh_signer(params).await;
    assert!(result.is_ok());
}
