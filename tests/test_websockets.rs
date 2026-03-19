mod common;

#[tokio::test]
async fn test_create_test_ws_client() {
    let ws_client = common::create_test_ws_client().await;
    assert!(ws_client.is_ok());
}
