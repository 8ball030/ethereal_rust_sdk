use rust_socketio::{ClientBuilder, Error, Payload, RawClient, TransportType};
use std::{
    result::Result::Ok,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::enums::Environment;
use rust_socketio::client::Client;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubscriptionMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}

const MARKET_PRICE: &str = "MarketPrice";

const SERVER_URL: &str = "wss://ws.etherealtest.net";

fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Production => "wss://ws.etherealstreamer.com",
        Environment::Testnet => "wss://ws.etherealtest.net",
    }
}

#[derive(Clone)]
pub struct WsClient {
    client_builder: ClientBuilder,
    client: Option<Client>,
}
impl WsClient {
    pub fn new(environment: Environment) -> Self {
        let url = get_server_url(&environment);
        let client_builder = ClientBuilder::new(url)
            .transport_type(TransportType::Websocket)
            .namespace("/v1/stream");
        Self {
            client_builder,
            client: None,
        }
    }
    pub fn subscribe_market_data(&self, product_id: &str) {
        // Get a reference to the connected client or bail out early
        let client = match &self.client {
            Some(c) => c,
            None => {
                println!("WebSocket client is not connected. Please call connect() first.");
                return;
            }
        };

        let market_price_msg = SubscriptionMessage {
            msg_type: MARKET_PRICE.to_string(),
            product_id: product_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&market_price_msg) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to serialize subscription message: {e}");
                return;
            }
        };

        // Now `client` is `&Client`, not `&Option<Client>`
        if let Err(e) = client.emit("subscribe", Payload::from(json_msg.to_string())) {
            eprintln!("Failed to emit subscribe: {e}");
        } else {
            println!("Subscribed MarketPrice: {product_id}");
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn connect(&mut self) -> Result<(), Error> {
        println!("Connecting WebSocket client...");

        // one Arc for the loop
        let connected_flag = Arc::new(AtomicBool::new(false));
        // cloned Arc for the callback
        let flag_for_cb = Arc::clone(&connected_flag);

        let builder =
            self.client_builder
                .clone()
                .on("open", move |_payload: Payload, _socket: RawClient| {
                    println!("Connected to {SERVER_URL}");
                    flag_for_cb.store(true, Ordering::SeqCst);
                });

        let c = builder.connect()?;

        // wait until callback fires
        while !connected_flag.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(100));
        }

        self.client = Some(c);
        println!("WebSocket client connected.");

        Ok(())
    }

    pub fn run_forever(&self) {
        println!("Running WebSocket client forever...");
        if let Some(_c) = &self.client {
            println!("WebSocket client is running.");
        } else {
            println!("WebSocket client is not connected. Please call connect() first.");
            return;
        }
        loop {
            std::thread::park();
        }
    }

    pub fn register_market_price_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        let builder = self.client_builder.clone().on(MARKET_PRICE, callback); // MARKER_PRICE is &str, no need for to_string()

        self.client_builder = builder;
    }
}
