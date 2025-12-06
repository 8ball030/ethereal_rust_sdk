mod common;
use ethereal_rust_sdk::apis::product_api::{
    ProductControllerGetByIdParams, ProductControllerGetMarketLiquidityParams,
    ProductControllerGetMarketPriceParams, ProductControllerListParams,
};
use ethereal_rust_sdk::sync_client::client::HttpClient;
use uuid::Uuid;

fn client_and_first_product() -> (HttpClient, Uuid) {
    let client = common::create_test_client().unwrap();

    let products = client
        .product()
        .list(ProductControllerListParams::default())
        .expect("product().list should succeed in test env");

    let product_id = products
        .data
        .first()
        .map(|p| p.id)
        .expect("at least one product should exist in test env");

    (client, product_id)
}

#[test]
fn test_get_by_id() {
    let (client, product_id) = client_and_first_product();

    let params = ProductControllerGetByIdParams {
        id: product_id.to_string(),
    };

    let result = client.product().get_by_id(params);
    assert!(result.is_ok(), "get_by_id failed: {:?}", result.err());
}

#[test]
fn test_get_market_liquidity() {
    let (client, product_id) = client_and_first_product();

    let params = ProductControllerGetMarketLiquidityParams {
        product_id: product_id.to_string(),
    };

    let result = client.product().get_market_liquidity(params);
    assert!(
        result.is_ok(),
        "get_market_liquidity failed: {:?}",
        result.err()
    );
}

#[test]
fn test_get_market_price() {
    let (client, product_id) = client_and_first_product();

    let params = ProductControllerGetMarketPriceParams {
        product_ids: vec![product_id],
    };

    let result = client.product().get_market_price(params);
    assert!(
        result.is_ok(),
        "get_market_price failed: {:?}",
        result.err()
    );
}

#[test]
fn test_list() {
    let client = common::create_test_client().unwrap();
    let result = client
        .product()
        .list(ProductControllerListParams::default());
    assert!(result.is_ok(), "list failed: {:?}", result.err());
}
