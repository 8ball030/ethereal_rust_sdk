mod common;
use ethereal_rust_sdk::apis::whitelist_api::WhitelistControllerIsWhitelistedParams;

#[tokio::test]
async fn test_is_whitelisted() {
    let client = common::create_test_client().await.unwrap();
    let params = WhitelistControllerIsWhitelistedParams {
        address: "0xdeadbeef00000000000000000000000000000000".to_string(),
    };
    let result = client.whitelist().is_whitelisted(params).await;
    assert!(result.is_ok());
}
