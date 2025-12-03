use log::{error, info, warn};
use rust_socketio::{ClientBuilder, Error, Payload, RawClient, TransportType};
use std::{
    result::Result::Ok,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::types::{ProductSubscriptionMessage, SubaccountSubscriptionMessage};
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
        info!("Connecting websocket...");

        let connected_flag = Arc::new(AtomicBool::new(false));
        let flag_for_cb = Arc::clone(&connected_flag);

        let builder =
            self.client_builder
                .clone()
                .on("open", move |_payload: Payload, _socket: RawClient| {
                    info!("Websocket connected");
                    flag_for_cb.store(true, Ordering::SeqCst);
                });

        let c = builder.connect()?;

        while !connected_flag.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(100));
        }

        self.client = Some(c);
        Ok(())
    }

    pub fn run_forever(&self) {
        loop {
            std::thread::sleep(Duration::from_secs(60));
        }
    }

    fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    fn subscribe_with_product(&self, channel: &str, product_id: &str) {
        if !self.is_connected() {
            warn!("websocket_not_connected action=subscribe");
            return;
        }

        let message = ProductSubscriptionMessage {
            msg_type: channel.to_string(),
            product_id: product_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&message) {
            Ok(v) => v,
            Err(e) => {
                error!("serialization_failed channel={channel} error={e}");
                return;
            }
        };

        let client = self.client.as_ref().unwrap();
        if let Err(e) = client.emit("subscribe", Payload::from(json_msg.to_string())) {
            error!("emit_failed channel={channel} error={e}");
        } else {
            info!("Subscribed to channel={channel} product_id={product_id}");
        }
    }

    fn subscribe_with_subaccount(&self, channel: &str, subaccount_id: &str) {
        if !self.is_connected() {
            warn!("websocket_not_connected action=subscribe");
            return;
        }

        let message = SubaccountSubscriptionMessage {
            msg_type: channel.to_string(),
            subaccount_id: subaccount_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&message) {
            Ok(v) => v,
            Err(e) => {
                error!("serialization_failed channel={channel} error={e}");
                return;
            }
        };

        let client = self.client.as_ref().unwrap();
        if let Err(e) = client.emit("subscribe", Payload::from(json_msg.to_string())) {
            error!("emit_failed channel={channel} error={e}");
        } else {
            info!("Subscribed to channel={channel} subaccount_id={subaccount_id}");
        }
    }

    fn register_callback_internal<F>(&mut self, channel: &str, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        let builder = self.client_builder.clone().on(channel, callback);
        self.client_builder = builder;
        info!("Callback registered channel={channel}");
    }

    pub fn subscribe_market_data(&self, product_id: &str) {
        self.subscribe_with_product(public_channels::MARKET_PRICE, product_id);
    }

    pub fn register_market_price_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::MARKET_PRICE, callback);
    }

    pub fn subscribe_orderbook_data(&self, product_id: &str) {
        self.subscribe_with_product(public_channels::BOOK_DEPTH, product_id);
    }

    pub fn register_orderbook_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::BOOK_DEPTH, callback);
    }

    pub fn subscribe_trade_fill_data(&self, product_id: &str) {
        self.subscribe_with_product(public_channels::TRADE_FILL, product_id);
    }

    pub fn register_trade_fill_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::TRADE_FILL, callback);
    }

    pub fn subscribe_transfer_events(&self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::TOKEN_TRANSFER, subaccount_id);
    }

    pub fn register_transfer_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::TOKEN_TRANSFER, callback);
    }

    pub fn subscribe_order_fill(&self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::ORDER_FILL, subaccount_id);
    }

    pub fn register_order_fill_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::ORDER_FILL, callback);
    }

    pub fn subscribe_order_update(&self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::ORDER_UPDATE, subaccount_id);
    }

    pub fn register_order_update_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::ORDER_UPDATE, callback);
    }

    pub fn subscribe_subaccount_liquidation(&self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::SUBACCOUNT_LIQUIDATION, subaccount_id);
    }

    pub fn register_subaccount_liquidation_callback<F>(&mut self, callback: F)
    where
        F: Fn(Payload, RawClient) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::SUBACCOUNT_LIQUIDATION, callback);
    }
}
