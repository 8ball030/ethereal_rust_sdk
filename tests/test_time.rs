mod common;

#[test]
fn test_get_system_time() {
    let client = common::create_test_client().unwrap();
    let result = client.time().get_system_time();
    assert!(result.is_ok());
}

#[test]
fn test_post_system_time() {
    let client = common::create_test_client().unwrap();
    let result = client.time().post_system_time();
    assert!(result.is_ok());
}
