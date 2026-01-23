mod common;

#[tokio::test]
async fn test_get_system_time() {
    let client = common::create_test_client().await.unwrap();
    let result = client.time().get_system_time().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_post_system_time() {
    let client = common::create_test_client().await.unwrap();
    let result = client.time().post_system_time().await;
    assert!(result.is_ok());
}
