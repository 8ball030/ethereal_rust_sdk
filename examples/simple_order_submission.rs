mod common;
use ethereal_rust_sdk::models::{OrderSide, OrderType};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (client, _) = common::create_test_clients()?;

    println!("Fetching all current orders...");
    let orders = client.get_open_orders().unwrap();
    for order in orders {
        println!("Order ID: {}, Status: {:?}", order.id, order.status);
        if order.status == OrderStatus.OPEN {
            println!("Cancelling order ID: {}", order.id);
            let cancel_result = client.cancel_order(&order.id);
            match cancel_result {
                Ok(_) => println!("Order ID: {} cancelled successfully.", order.id),
                Err(e) => println!("Failed to cancel order ID: {}. Error: {}", order.id, e),
            }
        }
    }

    println!("Creating order...");

    let ticker = "BTC-USD";
    let quantity = 0.001;
    let price = 80000.0;
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;


    let order = client
        .submit_order(
            ticker, 
            quantity, 
            Some(price), 
            side, 
            r#type)
        .unwrap();
    println!("Order submitted: {:?}", order);
    Ok(())
}
