mod common;
use ethereal_rust_sdk::apis::funding_api::{
    FundingControllerListByProductIdParams, FundingControllerListProjectedRatesParams,
};

#[tokio::test]
async fn test_list_by_product_id() {
    let client = common::create_test_client().await.unwrap();
    let product = common::get_product(&client).await.unwrap();
    let end_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let params = FundingControllerListByProductIdParams {
        product_id: product.id.to_string(),
        start_time: end_time - 86_400_000.0,
        end_time,
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
