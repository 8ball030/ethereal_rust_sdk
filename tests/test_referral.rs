mod common;
use ethereal_rust_sdk::apis::referral_api::{
    ReferralControllerActivateParams, ReferralControllerClaimCodeParams, ReferralControllerGetCodeUsageParams, ReferralControllerGetSummaryParams, ReferralControllerListReferralsParams,
};

#[test]
#[ignore]
fn test_activate() {
    let client = common::create_test_client().unwrap();
    let params = ReferralControllerActivateParams::default();
    let result = client.referral().activate(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_claim_code() {
    let client = common::create_test_client().unwrap();
    let params = ReferralControllerClaimCodeParams::default();
    let result = client.referral().claim_code(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_code_usage() {
    let client = common::create_test_client().unwrap();
    let params = ReferralControllerGetCodeUsageParams::default();
    let result = client.referral().get_code_usage(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_summary() {
    let client = common::create_test_client().unwrap();
    let params = ReferralControllerGetSummaryParams::default();
    let result = client.referral().get_summary(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_referrals() {
    let client = common::create_test_client().unwrap();
    let params = ReferralControllerListReferralsParams::default();
    let result = client.referral().list_referrals(params);
    assert!(result.is_ok());
}
