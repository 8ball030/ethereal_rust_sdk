mod common;

#[test]
fn test_is_maintenance() {
    let client = common::create_test_client().unwrap();
    let result = client.maintenance().is_maintenance();
    assert!(result.is_ok());
}
