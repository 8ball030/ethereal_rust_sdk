mod common;
use ethereal_rust_sdk::apis::product_api::{
    ProductControllerGetByIdParams, ProductControllerGetMarketLiquidityParams,
    ProductControllerGetMarketPriceParams, ProductControllerListParams,
};
use ethereal_rust_sdk::sync_client::client::HttpClient;
use uuid::Uuid;

async fn client_and_first_product() -> (HttpClient, Uuid) {
    let client = common::create_test_client().await.unwrap();

    let product = common::get_product(&client).await.unwrap();

    (client, product.id)
}

#[tokio::test]
async fn test_get_by_id() {
    let (client, product_id) = client_and_first_product().await;

    let params = ProductControllerGetByIdParams {
        id: product_id.to_string(),
    };

    let result = client.product().get_by_id(params).await;
    assert!(result.is_ok(), "get_by_id failed: {:?}", result.err());
}

#[tokio::test]
async fn test_get_market_liquidity() {
    let (client, product_id) = client_and_first_product().await;

    let params = ProductControllerGetMarketLiquidityParams {
        product_id: product_id.to_string(),
    };

    let result = client.product().get_market_liquidity(params).await;
    assert!(
        result.is_ok(),
        "get_market_liquidity failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_get_market_price() {
    let (client, product_id) = client_and_first_product().await;

    let params = ProductControllerGetMarketPriceParams {
        product_ids: vec![product_id],
    };

    let result = client.product().get_market_price(params).await;
    assert!(
        result.is_ok(),
        "get_market_price failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_list() {
    let client = common::create_test_client().await.unwrap();
    let result = client
        .product()
        .list(ProductControllerListParams::default())
        .await;
    assert!(result.is_ok(), "list failed: {:?}", result.err());
}
