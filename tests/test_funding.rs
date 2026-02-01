mod common;
use ethereal_rust_sdk::apis::funding_api::{
    FundingControllerGetProjectedFundingRateParams, FundingControllerListByProductIdParams,
    FundingControllerListProjectedRatesParams,
};

#[tokio::test]
async fn test_get_projected_funding_rate() {
    let client = common::create_test_client().await.unwrap();
    let product = common::get_product(&client).await.unwrap();
    let params = FundingControllerGetProjectedFundingRateParams {
        product_id: product.id.to_string(),
    };
    let result = client.funding().get_projected_funding_rate(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_by_product_id() {
    let client = common::create_test_client().await.unwrap();
    let product = common::get_product(&client).await.unwrap();
    let params = FundingControllerListByProductIdParams {
        product_id: product.id.to_string(),
        range: "DAY".to_string(),
        ..Default::default()
    };
    let result = client.funding().list_by_product_id(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_projected_rates() {
    let client = common::create_test_client().await.unwrap();
    let product = common::get_product(&client).await.unwrap();
    let params = FundingControllerListProjectedRatesParams {
        product_ids: vec![product.id],
    };
    let result = client.funding().list_projected_rates(params).await;
    assert!(result.is_ok());
}
