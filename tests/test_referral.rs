mod common;
use ethereal_rust_sdk::apis::referral_api::{
    ReferralControllerActivateParams, ReferralControllerClaimCodeParams,
    ReferralControllerGetCodeUsageParams, ReferralControllerGetSummaryParams,
    ReferralControllerListReferralsParams,
};

#[tokio::test]
#[ignore]
async fn test_activate() {
    let client = common::create_test_client().await.unwrap();
    let params = ReferralControllerActivateParams::default();
    let result = client.referral().activate(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_claim_code() {
    let client = common::create_test_client().await.unwrap();
    let params = ReferralControllerClaimCodeParams::default();
    let result = client.referral().claim_code(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_get_code_usage() {
    let client = common::create_test_client().await.unwrap();
    let params = ReferralControllerGetCodeUsageParams::default();
    let result = client.referral().get_code_usage(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_get_summary() {
    let client = common::create_test_client().await.unwrap();
    let params = ReferralControllerGetSummaryParams::default();
    let result = client.referral().get_summary(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_list_referrals() {
    let client = common::create_test_client().await.unwrap();
    let params = ReferralControllerListReferralsParams::default();
    let result = client.referral().list_referrals(params).await;
    assert!(result.is_ok());
}
