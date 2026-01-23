mod common;

#[tokio::test]
async fn test_is_maintenance() {
    let client = common::create_test_client().await.unwrap();
    let result = client.maintenance().is_maintenance().await;
    assert!(result.is_ok());
}
