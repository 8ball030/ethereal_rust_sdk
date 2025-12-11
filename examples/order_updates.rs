mod common;
use ethereal_rust_sdk::apis::position_api::PositionControllerGetActiveParams;
use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::models::PageOfOrderDtos;

use log::{error, info};
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn order_update_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            match serde_json::from_value::<PageOfOrderDtos>(value.clone()) {
                Ok(page) => {
                    for fill in page.data {
                        info!(
                            "Order update - ID: {}, Product ID: {}, Price: {}, Side: {:?} Quantity: {:?}",
                            fill.id, fill.product_id, fill.price, fill.side, fill.filled
                        );
                    }
                }
                Err(err) => {
                    error!("Failed to deserialize order data: {value}, error: {err}");
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, mut ws_client) = common::create_test_clients()?;
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params).unwrap();

    products
        .data
        .first()
        .expect("No products found in test account");

    let product_id = &products.data.first().unwrap().id;
    let params = PositionControllerGetActiveParams {
        subaccount_id: subaccounts.data.first().unwrap().id.to_string(),
        product_id: product_id.to_string(),
    };
    println!("Params: {params:?}");

    ws_client.register_order_update_callback(order_update_callback);
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_order_update(&subaccount.id.to_string());
    });
    ws_client.connect()?;
    ws_client.run_forever();

    Ok(())
}
