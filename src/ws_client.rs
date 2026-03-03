use dashmap::DashMap;

use serde::de::DeserializeOwned;
use tokio::{
    sync::oneshot,
    task::JoinHandle,
    time::{Duration, Instant, MissedTickBehavior, interval, sleep},
};

use anyhow::anyhow;
use ethers::{
    abi::Address,
    signers::{LocalWallet, Signer},
};
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use std::env::var;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::{Mutex, mpsc, watch};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, Error, protocol::Message},
};

use crate::{
    // auth_utils::make_auth_token,
    // models::{Instrument, RpcErrorResponse, RpcResponse},
    // models::{InstrumentPublicResponseSchema, PublicGetAllInstrumentsResultSchema},
    // namespaces::{orders::OrdersNamespace, positions::PositionsNamespace},
    enums::Environment,
    routing::{extract_channel, extract_id, extract_id_tail},
    types::{ClientError, ExternalEvent, InternalCommand, ResponseSender, RpcMessage}, // signing::sign_ws_login,
                                                                                      // subscriptions::Subscriptions,
                                                                                      // types::{
                                                                                      //     ChannelResponse, ClientError, Environment, ExternalEvent, InternalCommand, LoginSuccess,
                                                                                      //     RequestScope, ResponseSender, RpcError, RpcResult, WsStream,
                                                                                      // },
};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(7);

pub struct AuthContext {
    // pub key_id: String,
    // pub account_id: String,
    // pub private_key_path: String,
    pub wallet: LocalWallet,
    pub public_address: String,
}

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    pub public_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    pub private_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
    // pub instruments_cache: Arc<DashMap<String, InstrumentPublicResponseSchema>>,
    connection_state_rx: watch::Receiver<ExternalEvent>,
    current_connection_state: Arc<Mutex<ExternalEvent>>,
    supervisor_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    subscription_tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
    // Stuff for loging in
    // pub wallet: Option<LocalWallet>,
    // pub public_address: Option<String>,
    // pub smart_contract_wallet_address: Option<Address>,
    // pub subaccount_id: Option<i64>,
    // stuff for signing.
    pub environment: Environment,
}

impl WsClient {
    // pub fn subscriptions(&self) -> Subscriptions<'_> {
    //     Subscriptions { client: self }
    // }

    // pub fn rpc(&self) -> Rpc<'_> {
    //     Rpc { client: self }
    // }

    pub async fn from_env(environment: Environment) -> Result<Self, ClientError> {
        // let private_key = match var("DERIVE_PRIVATE_KEY") {
        //     Ok(v) => v,
        //     Err(e) => return Err(ClientError::EnvVar(e)),
        // };
        // let smart_contract_wallet_address = match var("DERIVE_SMART_CONTRACT_WALLET_ADDRESS") {
        //     Ok(v) => v,
        //     Err(e) => return Err(ClientError::EnvVar(e)),
        // };
        // let subaccount_id = match var("DERIVE_SUBACCOUNT_ID") {
        //     Ok(s) => match s.parse::<i64>() {
        //         Ok(id) => id,
        //         Err(e) => return Err(ClientError::Anyhow(anyhow!(e))),
        //     },
        //     Err(e) => return Err(ClientError::EnvVar(e)),
        // };

        let client = WsClient::new(
            environment,
            // Some(private_key),
            // Some(smart_contract_wallet_address),
            // Some(subaccount_id),
        )
        .await?;
        Ok(client)
    }

    // pub fn orders(&self) -> OrdersNamespace<'_> {
    //     OrdersNamespace { ws_client: self }
    // }

    // pub fn positions(&self) -> PositionsNamespace<'_> {
    //     PositionsNamespace { ws_client: self }
    // }

    pub async fn new_public(environment: Environment) -> Result<Self, ClientError> {
        let client = WsClient::new(environment).await?;
        client.wait_for_connection().await;
        Ok(client)
    }

    pub async fn new(
        env: Environment,
        // private_key: Option<String>,
        // smart_contract_wallet_address: Option<String>,
        // subaccount_id: Option<i64>,
    ) -> Result<Self, ClientError> {
        let url = env.get_url().to_string();
        // let mut wallet = None;
        // let mut public_address = None;
        // match &private_key {
        //     Some(key) => {
        //         wallet = Some(key.parse::<LocalWallet>().expect("Invalid private key"));
        //         public_address = Some(format!("{:?}", wallet.as_ref().unwrap().address()));
        //         info!(
        //             "Creating WsClient in private mode with address: {}",
        //             public_address.as_ref().unwrap()
        //         );
        //     }
        //     None => {
        //         info!("Creating WsClient in public mode");
        //     }
        // }
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let pending_requests = Arc::new(DashMap::new());
        let public_subscriptions = Arc::new(DashMap::new());
        let private_subscriptions = Arc::new(DashMap::new());
        let next_id = Arc::new(AtomicU64::new(1));

        let (connection_state_tx, connection_state_rx) =
            watch::channel(ExternalEvent::Disconnected);

        let _ = connection_state_tx.send(ExternalEvent::Disconnected);

        let supervisor_handle = tokio::spawn(connection_supervisor(
            url,
            cmd_rx,
            shutdown_rx,
            pending_requests.clone(),
            public_subscriptions.clone(),
            private_subscriptions.clone(),
            connection_state_tx,
        ));

        // let instruments_cache = Arc::new(DashMap::new());

        let client = WsClient {
            write_tx: cmd_tx.clone(),
            pending_requests: pending_requests.clone(),
            public_subscriptions: public_subscriptions.clone(),
            private_subscriptions: private_subscriptions.clone(),
            next_id: next_id.clone(),
            shutdown_tx: shutdown_tx.clone(),
            // instruments_cache: Arc::new(DashMap::new()),
            connection_state_rx,
            current_connection_state: Arc::new(Mutex::new(ExternalEvent::Disconnected)),
            supervisor_handle: Arc::new(Mutex::new(Some(supervisor_handle))),
            subscription_tasks: Arc::new(Mutex::new(Vec::new())),
            // wallet,
            // public_address,
            // smart_contract_wallet_address: smart_contract_wallet_address
            //     .and_then(|addr| addr.parse::<Address>().ok()),
            // subaccount_id,
            // instruments_cache,
            environment: env,
        };
        // cache instruments on first creation
        // client.cache_instruments().await?;
        // if private_key.is_some() {
        //     client.wait_for_connection().await;
        // }
        Ok(client)
    }

    pub async fn send_rpc<T>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<String>();
        self.pending_requests.insert(id, tx);

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let text = request.to_string();

        if let Err(e) = self
            .write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))
        {
            self.pending_requests.remove(&id);
            return Err(ClientError::Transport(Box::new(e)));
        }

        let response = rx.await?;
        // Try to deserialize as a successful RPC result first
        match serde_json::from_str::<RpcMessage<T>>(&response) {
            Ok(result) => Ok(result.result),
            Err(parse_err) => {
                // Try to deserialize as an RPC error
                if let Ok(rpc_error) = serde_json::from_str::<RpcError>(&response) {
                    error!("RPC error response: {:?}; raw: {response}", rpc_error.error);
                    Err(ClientError::RpcError {
                        error: rpc_error.error,
                    })
                } else {
                    error!("Failed to parse RPC response; raw: {response}");
                    Err(ClientError::Parse(parse_err))
                }
            }
        }
    }

    pub async fn shutdown(&self, reason: &'static str) -> Result<(), ClientError> {
        info!("Shutdown requested: {reason}");
        self.public_subscriptions.clear();
        self.private_subscriptions.clear();
        let _ = self.shutdown_tx.send(true);
        let _ = self.write_tx.send(InternalCommand::Close);
        if let Some(handle) = self.supervisor_handle.lock().await.take() {
            match tokio::time::timeout(Duration::from_secs(5), handle).await {
                Ok(Ok(())) => {
                    info!("Supervisor task completed successfully");
                }
                Ok(Err(e)) => {
                    error!("Supervisor task panicked: {e:?}");
                    return Err(ClientError::Transport(Box::new(e)));
                }
                Err(e) => {
                    error!("Supervisor task timeout after 5s");
                    return Err(ClientError::Transport(Box::new(e)));
                }
            }
        }
        for task in self.subscription_tasks.lock().await.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn subscribe_channel<P, F>(
        &self,
        scope: RequestScope,
        channel: String,
        mut callback: F,
    ) -> Result<String, ClientError>
    where
        P: DeserializeOwned + Send + 'static,
        F: FnMut(P) + Send + 'static,
    {
        let _sub_result: ChannelResponse = self
            .send_rpc(
                "subscribe",
                serde_json::json!({
                    "channels": [channel.clone()]
                }),
            )
            .await?;

        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            match scope {
                RequestScope::Public => {
                    self.public_subscriptions.insert(channel.clone(), tx);
                    info!("Subscribed to public channel: {channel}");
                }
                RequestScope::Private => {
                    self.private_subscriptions.insert(channel.clone(), tx);
                    info!("Subscribed to private channel: {channel}");
                }
            }
        }

        let handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let parsed: P = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };

                callback(parsed);
            }
        });
        self.subscription_tasks.lock().await.push(handle);
        Ok(channel)
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), ClientError> {
        let channel = channel.to_string();
        if let Some(task) = self.subscription_tasks.lock().await.pop() {
            task.abort();
        }
        {
            if self.public_subscriptions.remove(&channel).is_some() {
                let _: ChannelResponse = self
                    .send_rpc(
                        "unsubscribe",
                        serde_json::json!({
                            "channels": [channel.clone()]
                        }),
                    )
                    .await?;
                info!("Unsubscribed from public channel: {channel}");
                return Ok(());
            }
        }
        {
            if self.private_subscriptions.remove(&channel).is_some() {
                let _: ChannelResponse = self
                    .send_rpc(
                        "unsubscribe",
                        serde_json::json!({
                            "channels": [channel.clone()]
                        }),
                    )
                    .await?;
                info!("Unsubscribed from private channel: {channel}");
                return Ok(());
            }
        }
        warn!("No active subscription found for channel: {channel}");
        Err(ClientError::Rpc(serde_json::json!({
            // "message": "No active subscription found for the specified channel"
        })))
    }

    pub async fn resubscribe_all(&self) -> Result<(), ClientError> {
        let public_channels: Vec<String> = {
            self.public_subscriptions
                .iter()
                .map(|e| e.key().clone())
                .collect()
        };
        let _: ChannelResponse = self
            .send_rpc(
                "subscribe",
                serde_json::json!({
                    "channels": public_channels
                }),
            )
            .await?;
        info!("Re-subscribed to public channels: {public_channels:?}");
        let private_channels: Vec<String> = {
            self.private_subscriptions
                .iter()
                .map(|e| e.key().clone())
                .collect()
        };
        let _: ChannelResponse = self
            .send_rpc(
                "subscribe",
                serde_json::json!({
                    "channels": private_channels
                }),
            )
            .await?;
        info!("Re-subscribed to private channels: {private_channels:?}");
        Ok(())
    }
    pub async fn run_till_event(&self) -> ExternalEvent {
        let mut rx = self.connection_state_rx.clone();
        // ONLY return when state changes
        loop {
            if rx.changed().await.is_ok() {
                let state = *rx.borrow_and_update();
                if state != *self.current_connection_state.lock().await {
                    let mut current_state = self.current_connection_state.lock().await;
                    *current_state = state;
                    return state;
                }
            }
        }
    }

    // pub async fn login(&self) -> Result<LoginSuccess, ClientError> {
    //     if self.wallet.is_none() {
    //         warn!("No wallet available for login");
    //         return Err(ClientError::Rpc(serde_json::json!({
    //             "message": "No wallet available for login"
    //         })));
    //     }
    //     if self.smart_contract_wallet_address.is_none() {
    //         warn!("No smart contract wallet available for login");
    //         return Err(ClientError::Rpc(serde_json::json!({
    //             "message": "No smart contract wallet available for login"
    //         })));
    //     }
    //     let wallet = self.wallet.as_ref().unwrap();
    //     let login_data = sign_ws_login(self.smart_contract_wallet_address.unwrap(), wallet).await;

    //     let result: Vec<u64> = self.send_rpc("public/login", login_data).await?;
    //     let success = LoginSuccess {
    //         id: self.next_id.load(Ordering::Relaxed).saturating_sub(1),
    //         result,
    //     };
    //     info!("Login successful: {success:?}");
    //     Ok(success)
    // }

    pub fn is_connected(&self) -> bool {
        // Remove async - this is just reading a value
        *self.connection_state_rx.borrow() == ExternalEvent::Connected
    }

    pub async fn wait_for_connection(&self) {
        let mut rx = self.connection_state_rx.clone();

        // If already connected, return immediately
        if *rx.borrow_and_update() == ExternalEvent::Connected {
            let mut current_state = self.current_connection_state.lock().await;
            *current_state = ExternalEvent::Connected;
            return;
        }

        // Otherwise wait for state changes until connected
        while rx.changed().await.is_ok() {
            if *rx.borrow_and_update() == ExternalEvent::Connected {
                let mut current_state = self.current_connection_state.lock().await;
                *current_state = ExternalEvent::Connected;
                return;
            }
        }
    }

    // for caching instruments for quick access and data. populate on first call
    // async fn get_instruments(&self) -> Result<PublicGetAllInstrumentsResultSchema, ClientError> {
    //     let result: PublicGetAllInstrumentsResultSchema = self
    //         .send_rpc(
    //             "public/get_all_instruments",
    //             serde_json::json!({
    //                 "expired": false,
    //                 "instrument_type": "perp"
    //             }),
    //         )
    //         .await?;
    //     Ok(result)
    // }

    // async fn cache_instruments(&self) -> Result<(), Error> {
    //     let instruments = self.get_instruments().await.unwrap();
    //     self.instruments_cache.clear();
    //     for instrument in &instruments.instruments {
    //         self.instruments_cache
    //             .insert(instrument.instrument_name.clone(), instrument.clone());
    //     }
    //     Ok(())
    // }
}

async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    connection_state_tx: watch::Sender<ExternalEvent>,
) {
    info!("Connection supervisor started for {url}");

    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }

        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                connection_state_tx.send(ExternalEvent::Connected).ok();
                info!("Connected to {url}");
                let result = run_single_connection(
                    &url,
                    ws_stream,
                    &mut cmd_rx,
                    &mut shutdown_rx,
                    &pending_requests,
                    &public_subscriptions,
                    &private_subscriptions,
                )
                .await;
                info!("Connection to {url} ended with result: {result:?}");

                if result.is_ok() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Connection exited normally for {url}");
                    break;
                }
                if let Err(e) = result {
                    connection_state_tx.send(ExternalEvent::Disconnected).ok();

                    error!("Connection error on {url}: {e}");
                }

                for key in pending_requests
                    .iter()
                    .map(|e| *e.key())
                    .collect::<Vec<u64>>()
                {
                    if let Some((_, tx)) = pending_requests.remove(&key) {
                        let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());
                    }
                }

                if *shutdown_rx.borrow() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Shutdown after connection end for {url}");
                    break;
                }

                if cmd_rx.is_closed() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Command channel closed for {url}, stopping supervisor");
                    break;
                }

                info!("Reconnecting to {url} after backoff");
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                connection_state_tx.send(ExternalEvent::Disconnected).ok();
            }
        }
    }

    info!("Connection supervisor exited for {url}");
}

async fn run_single_connection(
    url: &str,
    mut ws: WsStream,
    cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
) -> Result<(), Error> {
    // Set up ping interval
    let mut ping_interval = interval(PING_INTERVAL);
    ping_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let read_deadline = sleep(READ_TIMEOUT);
    tokio::pin!(read_deadline);

    loop {
        tokio::select! {
            _ = ping_interval.tick() => {
                if let Err(e) = ws.send(Message::Ping(Vec::new().into())).await {
                    warn!("Failed to send ping for {url}: {e}");
                    return Err(e);
                }
            }

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    info!("Shutdown requested for {url}");
                    let _ = ws.close(None).await;
                    return Ok(());
                }
            }

            maybe_cmd = cmd_rx.recv() => {
                match maybe_cmd {
                    Some(InternalCommand::Send(msg)) => {
                        ws.send(msg).await?;
                    }
                    Some(InternalCommand::Close) => {
                        info!("Close command received for {url}");
                        let _ = ws.close(None).await;
                        return Ok(());
                    }
                    None => {
                        info!("Command channel closed for {url}");
                        let _ = ws.close(None).await;
                        return Ok(());
                    }
                }
            }

            msg = ws.next() => {
                read_deadline.as_mut().reset(Instant::now() + READ_TIMEOUT);
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        handle_incoming(
                            &text,
                            pending_requests,
                            public_subscriptions,
                            private_subscriptions,
                        ).await;
                    }
                    Some(Ok(Message::Binary(bin))) => {
                        if let Ok(text) = String::from_utf8(bin.to_vec()) {
                            handle_incoming(
                                &text,
                                pending_requests,
                                public_subscriptions,
                                private_subscriptions,
                            ).await;
                        } else {
                            warn!("Non-UTF8 binary message on {url}");
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        ws.send(Message::Pong(data)).await?;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        // Pong received, connection is alive
                    }
                    Some(Ok(Message::Close(frame))) => {
                        warn!("WebSocket closed for {url}: {frame:?}");
                        return Ok(());
                    }
                    Some(Err(e)) => {
                        warn!("WebSocket error for {url}: {e}");
                        return Err(e);
                    }
                    Some(Ok(Message::Frame(_))) => {
                        warn!("Received unsupported Frame message on {url}");
                    }
                    None => {
                        warn!("WebSocket stream ended for {url}");
                        return Ok(());
                    }
                }
            }

        _ = &mut read_deadline => {
            warn!("WebSocket read timeout for {url} - connection appears dead");
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "WebSocket read timeout",
            )));
        }
        }
    }
}

#[inline(always)]
pub async fn handle_incoming(
    text: &Bytes,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
    private_subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<String>>>,
) {
    // let bytes = text.as_bytes();

    if let Some(id) = extract_id(text)
        && let Some((_, tx)) = pending_requests.remove(&id)
    {
        let _ = tx.send(text.to_owned());
        return;
    }

    // ---- fast path: channel_name ----
    if let Some(channel) = extract_channel(text) {
        for routes in [private_subscriptions, public_subscriptions] {
            if let Some(sender) = routes.get(channel) {
                if sender.send(text.to_owned()).is_err() {
                    routes.remove(channel);
                }
                return;
            }
        }

        warn!("No subscription handler for channel: {channel}");
        return;
    }
    // ---- fast path: id ----
    if let Some(id) = extract_id_tail(text) {
        if let Some((_, tx)) = pending_requests.remove(&id) {
            let _ = tx.send(text.to_owned());
        }
        return;
    }
    // ---- slow path / unhandled ----
    warn!("Received unhandled message: {text:?}");
}
