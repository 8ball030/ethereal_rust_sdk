mod common;
use ethereal_rust_sdk::models::{OrderSide, OrderType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = common::create_test_clients()?;

    println!("Creating order...");

    let ticker = "BTC-USD";
    let quantity = 0.001;
    let price = 80000.0;
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;

    let order = client
        .submit_order(ticker, quantity, Some(price), side, r#type)
        .unwrap();
    println!("Order submitted: {order:?}");

    println!("Fetching all current orders to cancel...");
    let orders = client.get_open_orders().unwrap();
    let cancel_result = client
        .cancel_orders(orders.iter().map(|order| order.id.to_string()).collect())
        .unwrap();
    println!("Cancel result: {cancel_result:?}");
    Ok(())
}
