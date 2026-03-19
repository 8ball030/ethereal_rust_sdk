use bytes::Bytes;
use log::{debug, error, info, warn};
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use dashmap::DashMap;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use thiserror::Error;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot, watch},
    task::JoinHandle,
    time::{interval, sleep, Instant, MissedTickBehavior},
};
use yawc::{Frame, MaybeTlsStream, OpCode, Options, WebSocket};

use crate::{
    // channels::public_channels,
    channels::Channels,
    enums::Environment,
    routing::extract_event,
    subscriptions::Subscriptions,
    types::ResponseSender, // utils::{get_server_url, get_typed_callback},
};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(7);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConnectionState {
    Disconnected,
    Connected,
    Reconnecting,
}
pub enum InternalCommand {
    Send(Frame),
    Close,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("WebSocket error: {0}")]
    WebsocketError(#[from] yawc::WebSocketError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
    #[error("Transport error: {0}")]
    Transport(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
}

pub struct WsClient {
    env: Environment,
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    state_rx: watch::Receiver<ConnectionState>,
    state_tx: watch::Sender<ConnectionState>,
    pub environment: Environment,
    supervisor_handle: Option<Arc<Mutex<JoinHandle<()>>>>,
    subs: Arc<DashMap<String, mpsc::UnboundedSender<Bytes>>>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    shutdown_tx: watch::Sender<bool>,
    next_id: Arc<AtomicU64>,
    subscription_tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

#[inline(always)]
pub fn deserialise_to_type<T>(s: &Bytes) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    match serde_json::from_slice::<T>(s) {
        Ok(val) => Ok(val),
        Err(e) => {
            error!("Deserialization error: {e:?}");
            error!("Raw response: {}", String::from_utf8_lossy(s));
            Err(e)
        }
    }
}

impl WsClient {
    pub fn new(environment: Environment) -> Self {
        let (state_tx, state_rx) = watch::channel::<ConnectionState>(ConnectionState::Disconnected);
        let pending_requests = Arc::new(DashMap::new());
        let subs = Arc::new(DashMap::new());

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let supervisor_handle = tokio::spawn(connection_supervisor(
            environment.get_server_url().to_string(),
            cmd_rx,
            shutdown_rx,
            pending_requests.clone(),
            subs.clone(),
            state_tx.clone(),
        ));
        let next_id = Arc::new(AtomicU64::new(1));

        let subscription_tasks = Arc::new(Mutex::new(Vec::new()));

        Self {
            env: environment,
            write_tx: cmd_tx.clone(),
            state_rx,
            state_tx,
            environment,
            supervisor_handle: Some(Arc::new(Mutex::new(supervisor_handle))),
            subs,
            pending_requests,
            shutdown_tx,
            next_id,
            subscription_tasks,
        }
    }

    pub fn subscriptions(&self) -> Subscriptions<'_> {
        Subscriptions { client: self }
    }
    pub async fn subscribe_channels<P, F, Fut>(
        &self,
        event: Channels,
        payloads: Vec<Bytes>,
        mut callback: F,
    ) -> Result<(), ClientError>
    where
        P: DeserializeOwned + Send + 'static,
        F: FnMut(P) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let (tx, mut rx) = mpsc::unbounded_channel::<Bytes>();

        self.subs.insert(event.as_string(), tx);
        debug!("Subscribing to public channel: {event:?}");

        let handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let parsed: P = match deserialise_to_type(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg:?}");
                        continue;
                    }
                };
                callback(parsed).await;
            }
        });

        for payload in payloads {
            let res = self.send_rpc_nowait(payload).await;
            match res {
                Ok(_) => debug!("Subscription message sent for channel: {event:?}"),
                Err(e) => {
                    error!("Failed to send subscription message for channel {event:?}: {e}");
                    return Err(e);
                }
            }
        }
        println!("Subscription result: ok! Channel: {event:?}");
        self.subscription_tasks
            .lock()
            .expect("Unable to subscribe")
            .push(handle);
        Ok(())
    }

    pub async fn send_rpc_nowait(&self, msg: Bytes) -> Result<(), ClientError> {
        self.write_tx
            .send(InternalCommand::Send(Frame::text(msg)))
            .map_err(|e| ClientError::Transport(Box::new(e)))
    }
    pub async fn send_rpc<T>(&self, msg: Bytes) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<Bytes>();
        self.pending_requests.insert(id, tx);

        if let Err(e) = self.write_tx.send(InternalCommand::Send(Frame::text(msg))) {
            self.pending_requests.remove(&id);
            return Err(ClientError::Transport(Box::new(e)));
        }

        let response = rx.await;
        let resp = match response {
            Ok(r) => r,
            Err(e) => {
                return Err(ClientError::Transport(Box::new(e)));
            }
        };

        let envelope: T = deserialise_to_type(&resp)?;
        Ok(envelope)
    }
}

async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    subscriptions: Arc<DashMap<String, mpsc::UnboundedSender<Bytes>>>,
    connection_state_tx: watch::Sender<ConnectionState>,
) {
    loop {
        info!("Connection supervisor started for {url}");

        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }
        let client = WebSocket::connect(url.parse().unwrap())
            .with_options(Options::default().with_high_compression())
            .await;

        match client {
            Ok(ws_stream) => {
                info!("Connected to {url}");
                let result = run_single_connection(
                    ws_stream,
                    &mut cmd_rx,
                    &mut shutdown_rx,
                    &pending_requests,
                    &subscriptions,
                )
                .await;
                info!("Connection to {url} ended with result: {result:?}");

                if result.is_ok() {
                    info!("Connection exited normally for {url}");
                    break;
                }
                if let Err(e) = result {
                    error!("Connection error on {url}: {e}");
                }
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                connection_state_tx.send(ConnectionState::Disconnected).ok();
            }
        }
    }
    info!("Connection supervisor exited for {url}");
}

async fn run_single_connection(
    mut ws: WebSocket<MaybeTlsStream<TcpStream>>,
    cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<Bytes>>>,
) -> Result<(), ClientError> {
    // Set up ping interval
    let mut ping_interval = interval(PING_INTERVAL);
    ping_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let read_deadline = sleep(READ_TIMEOUT);
    tokio::pin!(read_deadline);

    loop {
        tokio::select! {
            _ = ping_interval.tick() => {
                if let Err(e) = ws.send(Frame::ping(Vec::new())).await {
                    warn!("Failed to send ping for connection {e}");
                    return Err(ClientError::WebsocketError(e));
                }
                info!("Ping sent successfully");
            }

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    info!("Shutdown requested.");
                    let _ = ws.close().await;
                    return Ok(());
                }
            }

            maybe_cmd = cmd_rx.recv() => {
                match maybe_cmd {
                    Some(InternalCommand::Send(msg)) => {
                        ws.send(msg).await?;
                    }
                    Some(InternalCommand::Close) => {
                        info!("Close command received");
                        let _ = ws.close().await;
                        return Ok(());
                    }
                    None => {
                        info!("Command channel closed.");
                        let _ = ws.close().await;
                        return Ok(());
                    }
                }
            }

            msg = ws.next() => {
                read_deadline.as_mut().reset(Instant::now() + READ_TIMEOUT);
                match msg {
                    None => {
                        warn!("WebSocket stream ended.");
                        return Ok(());
                    },
                    Some(frame) => {
                        let (opcode, _is_fin, body) = frame.into_parts();
                        match opcode {
                            OpCode::Text => {
                                // info!("Received text frame");
                                handle_incoming(
                                    &body,
                                    pending_requests,
                                    subscriptions
                                ).await;
                            },
                            OpCode::Pong => {
                                info!("Received pong frame");
                            },
                            _ => {
                                warn!("Received unsupported non-text frame, opcode: {opcode:?}");
                                continue;
                            }

                        }
                    }
                }
            }

        _ = &mut read_deadline => {
            warn!("WebSocket read timeout. No messages received within {READ_TIMEOUT:?}");
            return Err(ClientError::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, "WebSocket read timeout")));
        }
        }
    }
}

#[inline(always)]
pub async fn handle_incoming(
    bytes: &Bytes,
    _pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    subscriptions: &Arc<DashMap<String, mpsc::UnboundedSender<Bytes>>>,
) {
    // // ---- fast path: channel_name ----
    if let Some(channel) = extract_event(bytes) {
        for routes in [subscriptions] {
            if let Some(sender) = routes.get(channel) {
                if sender.send(bytes.to_owned()).is_err() {
                    routes.remove(channel);
                }
                return;
            }
        }
        warn!("No subscription handler for channel: {channel}");
        return;
    }
    warn!("Received unhandled message: {bytes:?}");
}
