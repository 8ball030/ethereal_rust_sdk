mod common;

#[tokio::test]
async fn test_get_config() {
    let client = common::create_test_client().await.unwrap();
    let result = client.rpc().get_config().await;
    assert!(result.is_ok());
}
