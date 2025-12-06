mod common;

#[test]
fn test_get_config() {
    let client = common::create_test_client().unwrap();
    let result = client.rpc().get_config();
    assert!(result.is_ok());
}
