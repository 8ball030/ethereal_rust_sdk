/// Example WebSocket client that connects to Bybit's public trade stream
mod common;
use std::time::Duration;

use ethereal_rust_sdk::{
    apis::product_api::ProductControllerListParams, enums::Environment, models::TickerMessage,
};
use log::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Debug).expect("log");

    let env = Environment::Testnet;
    println!("Environment set to {env:?}");
    let (http_client, ws_client) = common::create_test_clients().await.unwrap();
    println!("HTTP client and WS client created");
    let params = ProductControllerListParams::default();
    println!("Fetching products with params: {params:?}");
    let products = http_client.product().list(params).await.unwrap().data;

    println!("Producsts: {products:?}");

    async fn ticker_callback(msg: TickerMessage) {
        info!("Received ticker message: {:?}", msg);
    }

    let tickers = products
        .iter()
        .map(|p| p.ticker.clone())
        .collect::<Vec<_>>();

    ws_client
        .subscriptions()
        .ticker(tickers, ticker_callback)
        .await
        .unwrap();

    println!("Starting event loop...");
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // Connect to the WebSocket server with fast compression enabled
    // let mut client = WebSocket::connect(env.get_server_url().parse().unwrap())
    //     .with_options(Options::default().with_high_compression())
    //     .await
    //     .expect("connection");

    // for product in products.iter() {
    //     let text = ProductSubscriptionMessage {
    //         msg_type: Channels::Ticker,
    //         symbol: product.display_ticker.clone().replace("-", ""),
    //     };

    //     let _ = client.send(Frame::text(text)).await;
    // }

    // // Set up an interval to send pings every 3 seconds
    // let mut ival = interval(Duration::from_secs(3));

    // loop {
    //     tokio::select! {
    //         // Send a ping on each tick
    //         _ = ival.tick() => {
    //             log::debug!("Tick");
    //             let _ = client.send(Frame::ping("idk")).await;
    //         }
    //         // Handle incoming frames
    //         frame = client.next() => {
    //             if frame.is_none() {
    //                 log::debug!("Disconnected");
    //                 break;
    //             }

    //             let frame = frame.unwrap();
    //             let (opcode, _is_fin, body) = frame.into_parts();
    //             match opcode {
    //                 OpCode::Text => {
    //                     let text = std::str::from_utf8(&body).expect("utf8");
    //                     log::info!("{text}");
    //                     let _: serde_json::Value = serde_json::from_str(text).expect("serde");
    //                 }
    //                 OpCode::Pong => {
    //                     let data = std::str::from_utf8(&body).unwrap();
    //                     log::debug!("Pong: {data}");
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }
}
