mod common;
use ethereal_rust_sdk::apis::funding_api::{
    FundingControllerGetProjectedFundingRateParams, FundingControllerListByProductIdParams,
    FundingControllerListProjectedRatesParams,
};

#[test]
fn test_get_projected_funding_rate() {
    let client = common::create_test_client().unwrap();
    let product = common::get_product(&client).unwrap();
    let params = FundingControllerGetProjectedFundingRateParams {
        product_id: product.id.to_string(),
    };
    let result = client.funding().get_projected_funding_rate(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_by_product_id() {
    let client = common::create_test_client().unwrap();
    let product = common::get_product(&client).unwrap();
    let params = FundingControllerListByProductIdParams {
        product_id: product.id.to_string(),
        range: "DAY".to_string(),
        ..Default::default()
    };
    let result = client.funding().list_by_product_id(params);
    assert!(result.is_ok());
}

#[test]
fn test_list_projected_rates() {
    let client = common::create_test_client().unwrap();
    let product = common::get_product(&client).unwrap();
    let params = FundingControllerListProjectedRatesParams {
        product_ids: vec![product.id],
    };
    let result = client.funding().list_projected_rates(params);
    assert!(result.is_ok());
}
