mod common;
use ethereal_rust_sdk::apis::order_api::OrderControllerListBySubaccountIdParams;
use ethereal_rust_sdk::models::submit_order_created_dto::Result;
use ethereal_rust_sdk::models::{submit_order_limit_dto_data, OrderSide, OrderType};

#[tokio::test]
async fn test_simple_order() {
    let client = common::create_test_client()
        .await
        .expect("Failed to create client");

    println!("Creating order...");

    let ticker = "BTC-USD";
    let quantity = 0.001;
    let price = 40000.0;
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;

    let expires_at = None;
    let time_in_force = submit_order_limit_dto_data::TimeInForce::Gtd;

    // We have a few more options when creating an order now.
    let post_only = false;
    let reduce_only = false;

    let order = client
        .submit_order(
            ticker,
            quantity,
            Some(price),
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();

    assert!(order.result == Result::Ok);
    assert!(order.filled == "0");
}

#[tokio::test]
async fn test_cancel_all_orders() {
    let client = common::create_test_client()
        .await
        .expect("Failed to create client");

    let params = OrderControllerListBySubaccountIdParams {
        subaccount_id: client.subaccounts[0].id.to_string(),
        is_working: Some(true),
        ..Default::default()
    };
    let orders = client
        .order()
        .list_by_subaccount_id(params)
        .await
        .expect("Failed to list orders");
    if orders.data.is_empty() {
        return;
    }
    let order_ids = orders
        .data
        .iter()
        .map(|order| order.id.clone().to_string())
        .collect();

    let result = client.cancel_orders(order_ids).await;

    assert!(result.is_ok());
}
