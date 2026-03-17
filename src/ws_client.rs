use std::{future::Future, sync::{Arc, Mutex}, time::Duration};
use anyhow::Result;

use dashmap::DashMap;
use futures_util::FutureExt;
use log::{error, info, warn};
use serde_json::Value;
use tokio::{net::TcpStream, sync::{mpsc, watch}, task::JoinHandle};
use yawc::{MaybeTlsStream, WebSocket};

use crate::{
    // channels::public_channels,
    enums::Environment,
    models::{
        BookDepthMessage, MarketPriceDto, PageOfOrderDtos, PageOfOrderFillDtos,
        SubaccountLiquidation, TradeStreamMessage, TransferDto,
    },
    types::{ProductSubscriptionMessage, ResponseSender, SubaccountSubscriptionMessage},
    // utils::{get_server_url, get_typed_callback},
};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(7);


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConnectionState {
    Disconnected,
    Connected,
    Reconnecting,
}
// pub enum InternalCommand {
//     Send(Message),
//     Close,
// }



pub struct WsClient {
    env: Environment,
    state_rx: watch::Receiver<ConnectionState>,
    state_tx: Arc<watch::Sender<ConnectionState>>,
    pub environment: Environment,
    supervisor_handle: Option<Arc<Mutex<JoinHandle<()>>>>,
    subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
}

impl WsClient {
    pub fn new(environment: Environment) -> Self {

        let (state_tx, state_rx) = watch::channel::<ConnectionState>(ConnectionState::Disconnected);
        // let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let pending_requests = Arc::new(DashMap::new());
        let subscriptions = Arc::new(DashMap::new());
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        Self {
            env: environment,
            state_rx,
            state_tx: Arc::new(state_tx),
            environment,
            supervisor_handle: None,
            subscriptions,
            pending_requests,
        }
    }

    // #[allow(clippy::result_large_err)]
    // // given a closure as a callback
    // pub async fn connect(&mut self) -> Result<(), Error> {
    //     // info!("Connecting websocket...");
    //     let url = get_server_url(&self.env).to_string();
    //     info!("Connecting to url={url}");
    //     let supervisor_handle = tokio::spawn(connection_supervisor(
    //         url,
    //         self.cmd_rx,
    //         self.shutdown_rx,
    //         self.pending_requests.clone(),
    //         self.subscriptions.clone(),
    //         self.state_tx,
    //     ));


        // let builder = self.client_builder.take().expect("connect called twice");

        // // bool channel to indicate connection established.

        // let subscriptions = Arc::clone(&self.subscriptions); // cheap clone
        // let connection_tx = self.state_tx.clone();

        // let connect_cb = move |_payload: Payload, socket: Client| {
        //     {
        //         let subscriptions = subscriptions.clone();
        //         let tx = connection_tx.clone();
        //         tx.send(ConnectionState::Connected)
        //             .expect("Failed to send connected signal");
        //         async move {
        //             info!("Websocket connected");
        //             for sub in subscriptions.iter() {
        //                 info!("Subscribing to channel: {sub:?}");
        //                 socket
        //                     .emit("subscribe", Payload::from(sub.to_string()))
        //                     .await
        //                     .expect("Failed to emit subscribe message");
        //             }
        //         }
        //     }
        //     .boxed()
        // };

        // let url = self.connection_url.clone();
        // let disconnect_tx = self.state_tx.clone();
        // let error_tx = self.state_tx.clone();
        // self.client = Some(
        //     builder
        //         .on("open", connect_cb)
        //         .reconnect_on_disconnect(true)
        //         .reconnect_delay(10, 30)
        //         .max_reconnect_attempts(100)
        //         .on_reconnect(move || {
        //             error!("Websocket reconnecting...");
        //             let tx = disconnect_tx.clone();
        //             tx.send(ConnectionState::Reconnecting)
        //                 .expect("Failed to send reconnecting signal");
        //             let url = url.clone();
        //             async move {
        //                 error!("Websocket reconnecting...");
        //                 let mut settings = ReconnectSettings::new();
        //                 settings.address(url);
        //                 settings
        //             }
        //             .boxed()
        //         })
        //         .on("close", move |err: Payload, _socket: Client| {
        //             error!("Websocket closed......");
        //             let tx = error_tx.clone();
        //             tx.send(ConnectionState::Disconnected)
        //                 .expect("Failed to send disconnected signal");
        //             async move {
        //                 error!("Websocket error: {:?}", err);
        //             }
        //             .boxed()
        //         })
        //         .connect()
        //         .await?,
        // );
        // match self.run_till_event().await {
        //     ConnectionState::Connected => {
        //         info!("All connected!")
        //     }
        //     _ => return Err(Error::StoppedEngineIoSocket),
        // }

        // Ok(())
    }

    // runs till one of the state changes is detected.
    // pub async fn run_till_event(&mut self) -> ConnectionState {
    //     self.state_rx.changed().await.unwrap();
    //     *self.state_rx.borrow()
    // }

    // fn subscribe_with_product(&mut self, channel: &str, product_id: &str) {
    //     let message = ProductSubscriptionMessage {
    //         msg_type: channel.to_string(),
    //         product_id: product_id.to_string(),
    //     };

    //     let json_msg = match serde_json::to_value(&message) {
    //         Ok(v) => v,
    //         Err(e) => {
    //             error!("serialization_failed channel={channel} error={e}");
    //             return;
    //         }
    //     };
    //     let subscriptions = Arc::get_mut(&mut self.subscriptions)
    //         .expect("Failed to get mutable reference to subscriptions");
    //     subscriptions.push(json_msg.clone());
    // }

    // fn subscribe_with_subaccount(&mut self, channel: &str, subaccount_id: &str) {
    //     let message = SubaccountSubscriptionMessage {
    //         msg_type: channel.to_string(),
    //         subaccount_id: subaccount_id.to_string(),
    //     };

    //     let json_msg = match serde_json::to_value(&message) {
    //         Ok(v) => v,
    //         Err(e) => {
    //             error!("serialization_failed channel={channel} error={e}");
    //             return;
    //         }
    //     };
    //     let subscriptions = Arc::get_mut(&mut self.subscriptions)
    //         .expect("Failed to get mutable reference to subscriptions");
    //     subscriptions.push(json_msg.clone());
    // }

    // fn register_callback_internal<F, T, Fut>(&mut self, channel: &str, callback: F)
    // where
    //     T: serde::de::DeserializeOwned + Send + 'static,
    //     F: Fn(T) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     // we wrap the user callback to parse the payload into the expected type
    //     let callback = get_typed_callback::<T, F, Fut>(callback);
    //     self.client_builder = self
    //         .client_builder
    //         .take()
    //         .expect("client_builder not set")
    //         .on(channel, callback)
    //         .into();
    //     info!("Callback registered channel={channel}");
    // }

    // pub fn register_market_data_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(MarketPriceDto) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::MARKET_PRICE, callback);
    // }

    // pub fn subscribe_market_data(&mut self, product_id: &str) {
    //     self.subscribe_with_product(public_channels::MARKET_PRICE, product_id);
    // }

    // pub fn register_orderbook_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(BookDepthMessage) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::BOOK_DEPTH, callback);
    // }

    // pub fn subscribe_orderbook_data(&mut self, product_id: &str) {
    //     self.subscribe_with_product(public_channels::BOOK_DEPTH, product_id);
    // }

    // pub fn subscribe_trade_fill_data(&mut self, product_id: &str) {
    //     self.subscribe_with_product(public_channels::TRADE_FILL, product_id);
    // }

    // pub fn register_trade_fill_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(TradeStreamMessage) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::TRADE_FILL, callback);
    // }

    // pub fn subscribe_transfer_events(&mut self, subaccount_id: &str) {
    //     self.subscribe_with_subaccount(public_channels::TOKEN_TRANSFER, subaccount_id);
    // }

    // pub fn register_transfer_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(TransferDto) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::TOKEN_TRANSFER, callback);
    // }

    // pub fn subscribe_order_fill(&mut self, subaccount_id: &str) {
    //     self.subscribe_with_subaccount(public_channels::ORDER_FILL, subaccount_id);
    // }

    // pub fn register_order_fill_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(PageOfOrderFillDtos) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::ORDER_FILL, callback);
    // }

    // pub fn subscribe_order_update(&mut self, subaccount_id: &str) {
    //     self.subscribe_with_subaccount(public_channels::ORDER_UPDATE, subaccount_id);
    // }

    // pub fn register_order_update_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(PageOfOrderDtos) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::ORDER_UPDATE, callback);
    // }

    // pub fn subscribe_subaccount_liquidation(&mut self, subaccount_id: &str) {
    //     self.subscribe_with_subaccount(public_channels::SUBACCOUNT_LIQUIDATION, subaccount_id);
    // }

    // pub fn register_subaccount_liquidation_callback<F, Fut>(&mut self, callback: F)
    // where
    //     F: Fn(SubaccountLiquidation) -> Fut + Send + Sync + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     self.register_callback_internal(public_channels::SUBACCOUNT_LIQUIDATION, callback);
    // }
// }

async fn connection_supervisor(
    url: String,
    // mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    state_tx: watch::Sender<ConnectionState>,
) {

    loop {

    }
    // info!("Connection supervisor started for {url}");

    // loop {
    //     if *shutdown_rx.borrow() {
    //         info!("Supervisor sees shutdown for {url}");
    //         break;
    //     }

    //     match connect_async(&url).await {
    //         Ok((ws_stream, _)) => {
    //             state_tx.send(ConnectionState::Connected).ok();
    //             info!("Connected to {url}");
    //             let result = run_single_connection(
    //                 &url,
    //                 ws_stream,
    //                 &mut cmd_rx,
    //                 &mut shutdown_rx,
    //                 &pending_requests,
    //                 &public_subscriptions,
    //                 &private_subscriptions,
    //             )
    //             .await;
    //             info!("Connection to {url} ended with result: {result:?}");

    //             if result.is_ok() {
    //                 connection_state_tx.send(ExternalEvent::Exited).ok();
    //                 info!("Connection exited normally for {url}");
    //                 break;
    //             }
    //             if let Err(e) = result {
    //                 connection_state_tx.send(ExternalEvent::Disconnected).ok();

    //                 error!("Connection error on {url}: {e}");
    //             }

    //             for key in pending_requests
    //                 .iter()
    //                 .map(|e| *e.key())
    //                 .collect::<Vec<u64>>()
    //             {
    //                 if let Some((_, tx)) = pending_requests.remove(&key) {
    //                     let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());
    //                 }
    //             }

    //             if *shutdown_rx.borrow() {
    //                 connection_state_tx.send(ExternalEvent::Exited).ok();
    //                 info!("Shutdown after connection end for {url}");
    //                 break;
    //             }

    //             if cmd_rx.is_closed() {
    //                 connection_state_tx.send(ExternalEvent::Exited).ok();
    //                 info!("Command channel closed for {url}, stopping supervisor");
    //                 break;
    //             }

    //             info!("Reconnecting to {url} after backoff");
    //             tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    //         }
    //         Err(e) => {
    //             error!("Failed to connect to {url}: {e}");
    //             if *shutdown_rx.borrow() || cmd_rx.is_closed() {
    //                 break;
    //             }
    //             tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    //             connection_state_tx.send(ExternalEvent::Disconnected).ok();
    //         }
    //     }
    // }

    // info!("Connection supervisor exited for {url}");
}



async fn run_single_connection(
    url: &str,
    mut ws: WebSocket<MaybeTlsStream<TcpStream>>,
    // cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
) -> Result<()> {
    // Set up ping interval
    loop {}
    // let mut ping_interval = interval(PING_INTERVAL);
    // ping_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    // let read_deadline = sleep(READ_TIMEOUT);
    // tokio::pin!(read_deadline);

    // loop {
    //     tokio::select! {
    //         _ = ping_interval.tick() => {
    //             if let Err(e) = ws.send(Message::Ping(Vec::new().into())).await {
    //                 warn!("Failed to send ping for {url}: {e}");
    //                 return Err(e);
    //             }
    //         }

    //         _ = shutdown_rx.changed() => {
    //             if *shutdown_rx.borrow() {
    //                 info!("Shutdown requested for {url}");
    //                 let _ = ws.close(None).await;
    //                 return Ok(());
    //             }
    //         }

    //         maybe_cmd = cmd_rx.recv() => {
    //             match maybe_cmd {
    //                 Some(InternalCommand::Send(msg)) => {
    //                     ws.send(msg).await?;
    //                 }
    //                 Some(InternalCommand::Close) => {
    //                     info!("Close command received for {url}");
    //                     let _ = ws.close(None).await;
    //                     return Ok(());
    //                 }
    //                 None => {
    //                     info!("Command channel closed for {url}");
    //                     let _ = ws.close(None).await;
    //                     return Ok(());
    //                 }
    //             }
    //         }

    //         msg = ws.next() => {
    //             read_deadline.as_mut().reset(Instant::now() + READ_TIMEOUT);
    //             match msg {
    //                 Some(Ok(Message::Text(text))) => {
    //                     handle_incoming(
    //                         &text,
    //                         pending_requests,
    //                         public_subscriptions,
    //                         private_subscriptions,
    //                     ).await;
    //                 }
    //                 Some(Ok(Message::Binary(bin))) => {
    //                     if let Ok(text) = String::from_utf8(bin.to_vec()) {
    //                         handle_incoming(
    //                             &text,
    //                             pending_requests,
    //                             public_subscriptions,
    //                             private_subscriptions,
    //                         ).await;
    //                     } else {
    //                         warn!("Non-UTF8 binary message on {url}");
    //                     }
    //                 }
    //                 Some(Ok(Message::Ping(data))) => {
    //                     ws.send(Message::Pong(data)).await?;
    //                 }
    //                 Some(Ok(Message::Pong(_))) => {
    //                     // Pong received, connection is alive
    //                 }
    //                 Some(Ok(Message::Close(frame))) => {
    //                     warn!("WebSocket closed for {url}: {frame:?}");
    //                     return Ok(());
    //                 }
    //                 Some(Err(e)) => {
    //                     warn!("WebSocket error for {url}: {e}");
    //                     return Err(e);
    //                 }
    //                 Some(Ok(Message::Frame(_))) => {
    //                     warn!("Received unsupported Frame message on {url}");
    //                 }
    //                 None => {
    //                     warn!("WebSocket stream ended for {url}");
    //                     return Ok(());
    //                 }
    //             }
    //         }

    //     _ = &mut read_deadline => {
    //         warn!("WebSocket read timeout for {url} - connection appears dead");
    //         return Err(Error::Io(std::io::Error::new(
    //             std::io::ErrorKind::TimedOut,
    //             "WebSocket read timeout",
    //         )));
    //     }
    //     }
    // }
}

#[inline(always)]
pub async fn handle_incoming(
    text: &str,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
) {
    // let bytes = text.as_bytes();

    // if let Some(id) = extract_id(bytes)
    //     && let Some((_, tx)) = pending_requests.remove(&id)
    // {
    //     let _ = tx.send(text.to_owned());
    //     return;
    // }

    // // ---- fast path: channel_name ----
    // if let Some(channel) = extract_channel(bytes) {
    //     for routes in [private_subscriptions, public_subscriptions] {
    //         if let Some(sender) = routes.get(channel) {
    //             if sender.send(text.to_owned()).is_err() {
    //                 routes.remove(channel);
    //             }
    //             return;
    //         }
    //     }

    //     warn!("No subscription handler for channel: {channel}");
    //     return;
    // }
    // // ---- fast path: id ----
    // if let Some(id) = extract_id_tail(bytes) {
    //     if let Some((_, tx)) = pending_requests.remove(&id) {
    //         let _ = tx.send(text.to_owned());
    //     }
    //     return;
    // }
    // ---- slow path / unhandled ----
    // warn!("Received unhandled message: {text}");
}
