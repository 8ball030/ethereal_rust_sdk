mod common;
use ethereal_rust_sdk::apis::funding_api::{
    FundingControllerGetProjectedFundingRateParams, FundingControllerListByProductIdParams, FundingControllerListProjectedRatesParams,
};

#[test]
#[ignore]
fn test_get_projected_funding_rate() {
    let client = common::create_test_client().unwrap();
    let params = FundingControllerGetProjectedFundingRateParams::default();
    let result = client.funding().get_projected_funding_rate(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_by_product_id() {
    let client = common::create_test_client().unwrap();
    let params = FundingControllerListByProductIdParams::default();
    let result = client.funding().list_by_product_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_projected_rates() {
    let client = common::create_test_client().unwrap();
    let params = FundingControllerListProjectedRatesParams::default();
    let result = client.funding().list_projected_rates(params);
    assert!(result.is_ok());
}
