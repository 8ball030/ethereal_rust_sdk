use std::{future::Future, sync::Arc};

use futures_util::FutureExt;
use log::{error, info};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder, ReconnectSettings},
    Error, Payload, TransportType,
};
use serde_json::Value;
use tokio::sync::watch;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConnectionState {
    Disconnected,
    Connected,
    Reconnecting,
}

pub struct WsClient {
    client_builder: Option<ClientBuilder>,
    client: Option<Client>,
    subscriptions: Arc<Vec<Value>>,
    state_rx: watch::Receiver<ConnectionState>,
    state_tx: Arc<watch::Sender<ConnectionState>>,
    connection_url: String,
}

impl WsClient {
    pub fn new(environment: Environment) -> Self {
        let url = get_server_url(&environment).to_string();
        let client_builder = ClientBuilder::new(&url)
            .transport_type(TransportType::Websocket)
            .namespace("/v1/stream");

        let (state_tx, state_rx) = watch::channel::<ConnectionState>(ConnectionState::Disconnected);

        Self {
            client_builder: Some(client_builder),
            client: None,
            subscriptions: Arc::new(Vec::new()),
            state_rx,
            state_tx: Arc::new(state_tx),
            connection_url: url,
        }
    }

    #[allow(clippy::result_large_err)]
    // given a closure as a callback
    pub async fn connect(&mut self) -> Result<(), Error> {
        info!("Connecting websocket...");
        let builder = self.client_builder.take().expect("connect called twice");

        // bool channel to indicate connection established.

        let subscriptions = Arc::clone(&self.subscriptions); // cheap clone
        let connection_tx = self.state_tx.clone();

        let connect_cb = move |_payload: Payload, socket: Client| {
            {
                let subscriptions = subscriptions.clone();
                let tx = connection_tx.clone();
                tx.send(ConnectionState::Connected)
                    .expect("Failed to send connected signal");
                async move {
                    info!("Websocket connected");
                    for sub in subscriptions.iter() {
                        info!("Subscribing to channel: {sub:?}");
                        socket
                            .emit("subscribe", Payload::from(sub.to_string()))
                            .await
                            .expect("Failed to emit subscribe message");
                    }
                }
            }
            .boxed()
        };

        let url = self.connection_url.clone();
        let disconnect_tx = self.state_tx.clone();
        let error_tx = self.state_tx.clone();
        self.client = Some(
            builder
                .on("open", connect_cb)
                .reconnect_on_disconnect(true)
                .reconnect_delay(10, 30)
                .max_reconnect_attempts(100)
                .on_reconnect(move || {
                    error!("Websocket reconnecting...");
                    let tx = disconnect_tx.clone();
                    tx.send(ConnectionState::Reconnecting)
                        .expect("Failed to send reconnecting signal");
                    let url = url.clone();
                    async move {
                        error!("Websocket reconnecting...");
                        let mut settings = ReconnectSettings::new();
                        settings.address(url);
                        settings
                    }
                    .boxed()
                })
                .on("close", move |err: Payload, _socket: Client| {
                    error!("Websocket closed......");
                    let tx = error_tx.clone();
                    tx.send(ConnectionState::Disconnected)
                        .expect("Failed to send disconnected signal");
                    async move {
                        error!("Websocket error: {:?}", err);
                    }
                    .boxed()
                })
                .connect()
                .await?,
        );
        match self.run_till_event().await {
            ConnectionState::Connected => {
                info!("All connected!")
            }
            _ => return Err(Error::StoppedEngineIoSocket),
        }

        Ok(())
    }

    // runs till one of the state changes is detected.
    pub async fn run_till_event(&mut self) -> ConnectionState {
        self.state_rx.changed().await.unwrap();
        *self.state_rx.borrow()
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

    fn register_callback_internal<F, T, Fut>(&mut self, channel: &str, callback: F)
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        // we wrap the user callback to parse the payload into the expected type
        let callback = get_typed_callback::<T, F, Fut>(callback);
        self.client_builder = self
            .client_builder
            .take()
            .expect("client_builder not set")
            .on(channel, callback)
            .into();
        info!("Callback registered channel={channel}");
    }

    pub fn register_market_data_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(MarketPriceDto) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::MARKET_PRICE, callback);
    }

    pub fn subscribe_market_data(&mut self, product_id: &str) {
        self.subscribe_with_product(public_channels::MARKET_PRICE, product_id);
    }

    pub fn register_orderbook_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(BookDepthMessage) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::BOOK_DEPTH, callback);
    }

    pub fn subscribe_orderbook_data(&mut self, product_id: &str) {
        self.subscribe_with_product(public_channels::BOOK_DEPTH, product_id);
    }

    pub fn subscribe_trade_fill_data(&mut self, product_id: &str) {
        self.subscribe_with_product(public_channels::TRADE_FILL, product_id);
    }

    pub fn register_trade_fill_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(TradeStreamMessage) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::TRADE_FILL, callback);
    }

    pub fn subscribe_transfer_events(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::TOKEN_TRANSFER, subaccount_id);
    }

    pub fn register_transfer_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(TransferDto) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::TOKEN_TRANSFER, callback);
    }

    pub fn subscribe_order_fill(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::ORDER_FILL, subaccount_id);
    }

    pub fn register_order_fill_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(PageOfOrderFillDtos) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::ORDER_FILL, callback);
    }

    pub fn subscribe_order_update(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::ORDER_UPDATE, subaccount_id);
    }

    pub fn register_order_update_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(PageOfOrderDtos) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::ORDER_UPDATE, callback);
    }

    pub fn subscribe_subaccount_liquidation(&mut self, subaccount_id: &str) {
        self.subscribe_with_subaccount(public_channels::SUBACCOUNT_LIQUIDATION, subaccount_id);
    }

    pub fn register_subaccount_liquidation_callback<F, Fut>(&mut self, callback: F)
    where
        F: Fn(SubaccountLiquidation) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.register_callback_internal(public_channels::SUBACCOUNT_LIQUIDATION, callback);
    }
}
