mod common;
use ethereal_rust_sdk::apis::linked_signer_api::{
    LinkedSignerControllerGetAccountQuotaParams, LinkedSignerControllerGetSignerParams,
    LinkedSignerControllerLinkSignerParams, LinkedSignerControllerListBySubaccountIdParams,
    LinkedSignerControllerRefreshSignerParams, LinkedSignerControllerRevokeSignerParams,
};

#[test]
#[ignore]
fn test_get_account_quota() {
    let client = common::create_test_client().unwrap();
    let params = LinkedSignerControllerGetAccountQuotaParams::default();
    let result = client.linked_signer().get_account_quota(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_signer() {
    let client = common::create_test_client().unwrap();
    let params = LinkedSignerControllerGetSignerParams::default();
    let result = client.linked_signer().get_signer(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_link_signer() {
    let client = common::create_test_client().unwrap();
    let params = LinkedSignerControllerLinkSignerParams::default();
    let result = client.linked_signer().link_signer(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_by_subaccount_id() {
    let client = common::create_test_client().unwrap();
    let params = LinkedSignerControllerListBySubaccountIdParams::default();
    let result = client.linked_signer().list_by_subaccount_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_refresh_signer() {
    let client = common::create_test_client().unwrap();
    let params = LinkedSignerControllerRefreshSignerParams::default();
    let result = client.linked_signer().refresh_signer(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_revoke_signer() {
    let client = common::create_test_client().unwrap();
    let params = LinkedSignerControllerRevokeSignerParams::default();
    let result = client.linked_signer().revoke_signer(params);
    assert!(result.is_ok());
}
