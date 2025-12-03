use rust_socketio::{ClientBuilder, Error, Payload, RawClient, TransportType};
use std::{
    result::Result::Ok,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::types::SubscriptionMessage;
use crate::{channels::public_channels, enums::Environment};
use rust_socketio::client::Client;

fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Production => "wss://ws.ethereal.trade",
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
                    println!("WebSocket connection established.");
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
    fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    pub fn subscribe_market_data(&self, product_id: &str) {
        // Get a reference to the connected client or bail out early
        if !self.is_connected() {
            println!("WebSocket client is not connected. Please call connect() first.");
            return;
        }

        let market_price_msg = SubscriptionMessage {
            msg_type: public_channels::MARKET_PRICE.to_string(),
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
        let client = self.client.as_ref().unwrap();
        if let Err(e) = client.emit("subscribe", Payload::from(json_msg.to_string())) {
            eprintln!("Failed to emit subscribe: {e}");
        } else {
            println!("Subscribed MarketPrice: {product_id}");
        }
    }
    pub fn register_market_price_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        let builder = self
            .client_builder
            .clone()
            .on(public_channels::MARKET_PRICE, callback); // MARKER_PRICE is &str, no need for to_string()

        self.client_builder = builder;
    }
    pub fn subscribe_orderbook_data(&self, product_id: &str) {
        // Get a reference to the connected client or bail out early
        if !self.is_connected() {
            println!("WebSocket client is not connected. Please call connect() first.");
            return;
        }
        let orderbook_msg = SubscriptionMessage {
            msg_type: public_channels::BOOK_DEPTH.to_string(),
            product_id: product_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&orderbook_msg) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to serialize subscription message: {e}");
                return;
            }
        };
        let client = self.client.as_ref().unwrap();
        if let Err(e) = client.emit("subscribe", Payload::from(json_msg.to_string())) {
            eprintln!("Failed to emit subscribe: {e}");
        } else {
            println!("Subscribed BookDepth: {product_id}");
        }
    }
    pub fn register_orderbook_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        let builder = self
            .client_builder
            .clone()
            .on(public_channels::BOOK_DEPTH, callback); // BOOK_DEPTH is &str, no need for to_string()

        self.client_builder = builder;
    }
    pub fn subscribe_trade_fill_data(&self, product_id: &str) {
        // Get a reference to the connected client or bail out early
        if !self.is_connected() {
            println!("WebSocket client is not connected. Please call connect() first.");
            return;
        }
        let trade_fill_msg = SubscriptionMessage {
            msg_type: public_channels::TRADE_FILL.to_string(),
            product_id: product_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&trade_fill_msg) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to serialize subscription message: {e}");
                return;
            }
        };
        let client = self.client.as_ref().unwrap();
        if let Err(e) = client.emit("subscribe", Payload::from(json_msg.to_string())) {
            eprintln!("Failed to emit subscribe: {e}");
        } else {
            println!("Subscribed TradeFill: {product_id}");
        }
    }
    pub fn register_trade_fill_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        let builder = self
            .client_builder
            .clone()
            .on(public_channels::TRADE_FILL, callback); // TRADE_FILL is &str, no need for to_string()
        self.client_builder = builder;
    }
}
