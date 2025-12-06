mod common;
use ethereal_rust_sdk::models::{OrderSide, OrderType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create wallet
    println!("Creating order...");

    let ticker = "BTC-USD";
    let quantity = 0.001;
    let price = 80000.0;
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;

    let (client, _) = common::create_test_clients()?;

    let order = client
        .submit_order(ticker, quantity, Some(price), side, r#type)
        .unwrap();
    println!("Order submitted: {order:?}");

    Ok(())
}
