use std::sync::Arc;

use futures_util::FutureExt;
use log::{error, info};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Error, Payload, TransportType,
};
use serde_json::Value;
use tokio::sync::mpsc;

use crate::{
    channels::public_channels,
    enums::Environment,
    models::{
        BookDepthMessage, MarketPriceDto, PageOfOrderDtos, PageOfOrderFillDtos,
        SubaccountLiquidation, TradeStreamMessage, TransferDto,
    },
    types::{ProductSubscriptionMessage, SubaccountSubscriptionMessage},
    utils::{get_server_url, get_typed_callback},
};

pub async fn run_forever() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
pub struct WsClient {
    client_builder: Option<ClientBuilder>,
    client: Option<Client>,
    subscriptions: Arc<Vec<Value>>,
}

impl WsClient {
    pub fn new(environment: Environment) -> Self {
        let url = get_server_url(&environment);
        let client_builder = ClientBuilder::new(url)
            .transport_type(TransportType::Websocket)
            .namespace("/v1/stream");
        Self {
            client_builder: Some(client_builder),
            client: None,
            subscriptions: Arc::new(Vec::new()),
        }
    }

    #[allow(clippy::result_large_err)]
    // given a closure as a callback
    pub async fn connect(mut self) -> Result<(), Error> {
        info!("Connecting websocket...");
        let builder = self.client_builder.take().expect("connect called twice");

        // bool channel to indicate connection established.
        let (_tx, mut rx) = mpsc::channel::<bool>(16);

        let subscriptions = Arc::clone(&self.subscriptions); // cheap clone

        let connect_cb = move |_payload: Payload, socket: Client| {
            {
                let subscriptions = subscriptions.clone();
                async move {
                    info!("Websocket connected");
                    for sub in subscriptions.iter() {
                        info!("Subscribing to channel: {sub:?}");
                        socket
                            .emit("subscribe", Payload::from(sub.to_string()))
                            .await
                            .expect("Failed to emit subscribe message");
                    }
                    // tx.send(true).await.expect("Failed to send connected signal");
                }
            }
            .boxed()
        };
        self.client = Some(builder.on("open", connect_cb).connect().await?);
        // wait for connection to be established
        match rx.recv().await {
            Some(_) => info!("Websocket connection established"),
            None => error!("Websocket connection failed to establish"),
        }
        Ok(())
    }

    fn subscribe_with_product(&mut self, channel: &str, product_id: &str) {
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
        let subscriptions = Arc::get_mut(&mut self.subscriptions)
            .expect("Failed to get mutable reference to subscriptions");
        subscriptions.push(json_msg.clone());
    }

    fn subscribe_with_subaccount(&mut self, channel: &str, subaccount_id: &str) {
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
        let subscriptions = Arc::get_mut(&mut self.subscriptions)
            .expect("Failed to get mutable reference to subscriptions");
        subscriptions.push(json_msg.clone());
    }

    fn register_callback_internal<F, T>(&mut self, channel: &str, callback: F)
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        // we wrap the user callback to parse the payload into the expected type
        let callback = get_typed_callback::<T, F>(callback);
        self.client_builder = self
            .client_builder
            .take()
            .expect("client_builder not set")
            .on(channel, callback)
            .into();
        info!("Callback registered channel={channel}");
    }

    pub fn register_market_data_callback<F>(&mut self, callback: F)
    where
        F: Fn(MarketPriceDto) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::MARKET_PRICE, callback);
    }

    pub fn subscribe_market_data(&mut self, product_id: &str) {
        self.subscribe_with_product(public_channels::MARKET_PRICE, product_id);
    }

    pub fn register_orderbook_callback<F>(&mut self, callback: F)
    where
        F: Fn(BookDepthMessage) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::BOOK_DEPTH, callback);
    }

    pub fn subscribe_orderbook_data(&mut self, product_id: &str) {
        self.subscribe_with_product(public_channels::BOOK_DEPTH, product_id);
    }

    pub fn subscribe_trade_fill_data(&mut self, product_id: &str) {
        self.subscribe_with_product(public_channels::TRADE_FILL, product_id);
    }

    pub fn register_trade_fill_callback<F>(&mut self, callback: F)
    where
        F: Fn(TradeStreamMessage) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::TRADE_FILL, callback);
    }

    pub fn subscribe_transfer_events(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::TOKEN_TRANSFER, subaccount_id);
    }

    pub fn register_transfer_callback<F>(&mut self, callback: F)
    where
        F: Fn(TransferDto) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::TOKEN_TRANSFER, callback);
    }

    pub fn subscribe_order_fill(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::ORDER_FILL, subaccount_id);
    }

    pub fn register_order_fill_callback<F>(&mut self, callback: F)
    where
        F: Fn(PageOfOrderFillDtos) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::ORDER_FILL, callback);
    }

    pub fn subscribe_order_update(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::ORDER_UPDATE, subaccount_id);
    }

    pub fn register_order_update_callback<F>(&mut self, callback: F)
    where
        F: Fn(PageOfOrderDtos) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::ORDER_UPDATE, callback);
    }

    pub fn subscribe_subaccount_liquidation(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::SUBACCOUNT_LIQUIDATION, subaccount_id);
    }

    pub fn register_subaccount_liquidation_callback<F>(&mut self, callback: F)
    where
        F: Fn(SubaccountLiquidation) + Send + Sync + 'static,
    {
        self.register_callback_internal(public_channels::SUBACCOUNT_LIQUIDATION, callback);
    }
}
