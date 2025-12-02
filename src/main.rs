use rust_socketio::{ClientBuilder, Payload, RawClient, TransportType};
use serde::Serialize;
use std::result::Result::Ok;
use std::thread;
use std::time::Duration;

mod channels;
mod models;

use models::market_price_dto::MarketPriceDto;
use models::page_of_product_dtos::PageOfProductDtos;
use models::product_dto::ProductDto;

use crate::channels::public_channels::{BOOK_DEPTH, MARKET_PRICE};

const SERVER_URL: &str = "wss://ws.etherealtest.net";
const API_URL: &str = "https://api.etherealtest.net";

#[derive(Debug, Serialize)]
struct SubscriptionMessage {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "productId")]
    product_id: String,
}

async fn get_products() -> Result<Vec<ProductDto>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{API_URL}/v1/product")).send().await?;
    println!("Fetching products");
    let product_response: PageOfProductDtos = response.json().await?;
    Ok(product_response.data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to Ethereal WebSocket server at {SERVER_URL}");

    // Fetch products first
    let products = get_products().await?;
    println!("Fetched {} products", products.len());

    // Build socket with event handlers
    thread::spawn(move || {
        // Build socket with event handlers
        let _socket = ClientBuilder::new(SERVER_URL)
            .transport_type(TransportType::Websocket)
            .namespace("/v1/stream")
            .on("open", move |_payload: Payload, socket: RawClient| {
                println!("Connected to {SERVER_URL}");

                // Subscribe to all products
                for product in &products {
                    // Subscribe to BookDepth
                    let book_depth_msg = SubscriptionMessage {
                        msg_type: BOOK_DEPTH.to_string(),
                        product_id: product.id.to_string(),
                    };

                    if let Ok(json_msg) = serde_json::to_value(&book_depth_msg) {
                        let _ = socket.emit("subscribe", Payload::from(json_msg.to_string()));
                        println!("Subscribed BookDepth:{}", product.id);
                    }

                    // Subscribe to MarketPrice
                    let market_price_msg = SubscriptionMessage {
                        msg_type: MARKET_PRICE.to_string(),
                        product_id: product.id.to_string(),
                    };

                    if let Ok(json_msg) = serde_json::to_value(&market_price_msg) {
                        let _ = socket.emit("subscribe", Payload::from(json_msg.to_string()));
                        println!("Subscribed MarketPrice:{}", product.id);
                    }
                }
            })
            .on("connecting", |_payload: Payload, _socket: RawClient| {
                println!("Attempting connection...");
            })
            .on("disconnect", |_payload: Payload, _socket: RawClient| {
                println!("Disconnected");
            })
            .on("error", |payload: Payload, _socket: RawClient| {
                println!("Error encountered: {payload:?}");
            })
            .on(
                BOOK_DEPTH.to_string(),
                |payload: Payload, _socket: RawClient| {
                    match payload {
                        Payload::Text(values) => {
                            if let Some(s) = values.first() {
                                println!("[BookDepth] Received: {s}");
                            }
                        }
                        Payload::Binary(bin) => println!("[BookDepth] Received bytes: {bin:#?}"),
                        _ => {} // Payload::String(_) => println!("[BookDepth] Received a string payload"),
                    }
                },
            )
            .on(MARKET_PRICE.to_string(), |payload: Payload, _| {
                if let Payload::Text(values) = payload {
                    for value in values {
                        if let Ok(market_price) = serde_json::from_value::<MarketPriceDto>(value) {
                            println!("MarketPrice: {market_price:?}");
                        }
                    }
                }
            })
            .connect()
            .expect("Connection failed");
        println!("Connection established!");
    });

    // Keep main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(0)).await;
    }
}
