mod common;
use ethereal_rust_sdk::models::{submit_order_limit_dto_data, OrderSide, OrderType};
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = common::create_test_clients().await?;

    println!("Creating order...");

    let ticker = "BTC-USD";
    let quantity = dec!(0.001);
    let price = dec!(80000.0);
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;

    let expires_at = None;
    let time_in_force = submit_order_limit_dto_data::TimeInForce::Gtd;

    // We have a few more options when creating an order now.
    let mut post_only = false;
    let mut reduce_only = false;

    let order = client
        .submit_order(
            ticker,
            quantity,
            price,
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();
    println!("Order submitted: {order:?}");

    // We can also create orders with post only flag
    println!("Creating post only reduce only order...");
    post_only = true;
    reduce_only = false;
    let order = client
        .submit_order(
            ticker,
            quantity,
            price,
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();
    println!("Post and reduce only order submitted: {order:?}");

    println!("Creating order with expires_at...");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    let expires_at = Some(now + 60); // Expires in 60 seconds
    let order = client
        .submit_order(
            ticker,
            quantity,
            price,
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();
    println!("Order with expires_at submitted: {order:?}");

    println!("Fetching all current orders to cancel...");
    let orders = client.get_open_orders().await?;
    let cancel_result = client
        .cancel_orders(orders.iter().map(|order| order.id.to_string()).collect())
        .await?;
    println!("Cancel result: {cancel_result:?}");
    Ok(())
}
