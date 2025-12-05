mod common;
use ethereal_rust_sdk::apis::product_api::{
    ProductControllerGetByIdParams, ProductControllerGetMarketLiquidityParams, ProductControllerGetMarketPriceParams, ProductControllerListParams,
};

#[test]
#[ignore]
fn test_get_by_id() {
    let client = common::create_test_client().unwrap();
    let params = ProductControllerGetByIdParams::default();
    let result = client.product().get_by_id(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_market_liquidity() {
    let client = common::create_test_client().unwrap();
    let params = ProductControllerGetMarketLiquidityParams::default();
    let result = client.product().get_market_liquidity(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_market_price() {
    let client = common::create_test_client().unwrap();
    let params = ProductControllerGetMarketPriceParams::default();
    let result = client.product().get_market_price(params);
    assert!(result.is_ok());
}

#[test]
fn test_list() {
    let client = common::create_test_client().unwrap();
    let params = ProductControllerListParams::default();
    let result = client.product().list(params);
    assert!(result.is_ok());
}
