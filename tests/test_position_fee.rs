mod common;
use ethereal_rust_sdk::apis::position_fee_api::{
    PositionFeeControllerListByProductIdParams, PositionFeeControllerListProjectedFeesParams,
};

#[tokio::test]
async fn test_list_by_product_id() {
    let client = common::create_test_client().await.unwrap();
    let product = common::get_product(&client).await.unwrap();
    let end_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let params = PositionFeeControllerListByProductIdParams {
        product_id: product.id.to_string(),
        start_time: end_time - 86_400_000.0,
        end_time,
        ..Default::default()
    };
    let result = client.position_fee().list_by_product_id(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_projected_fees() {
    let client = common::create_test_client().await.unwrap();
    let product = common::get_product(&client).await.unwrap();
    let params = PositionFeeControllerListProjectedFeesParams {
        product_ids: vec![product.id],
    };
    let result = client.position_fee().list_projected_fees(params).await;
    assert!(result.is_ok());
}
