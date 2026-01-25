use serde_json::Error as SerdeError;
use std::{future::Future, marker::Send, pin::Pin};
use std::{io, sync::Arc};

use log::error;
use rust_socketio::{asynchronous::Client, Payload};

use crate::{async_client::client::HttpClient, enums::Environment, ws_client::WsClient};

pub async fn create_client(
    env: Environment,
    private_key: &str,
) -> anyhow::Result<(HttpClient, WsClient)> {
    let http_client = HttpClient::new(env, private_key).await;
    let ws_client = WsClient::new(env);
    Ok((http_client, ws_client))
}

pub fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Mainnet => "wss://ws.ethereal.trade",
        Environment::Testnet => "wss://ws.etherealtest.net",
    }
}

fn parse_payload_to_type<T>(payload: Payload) -> Result<Vec<T>, SerdeError>
where
    T: serde::de::DeserializeOwned,
{
    match payload {
        Payload::Text(t) => t.into_iter().map(serde_json::from_value::<T>).collect(),
        Payload::Binary(b) => serde_json::from_slice::<Vec<T>>(&b),
        _ => {
            let io_err = io::Error::other("Unsupported payload type");
            Err(SerdeError::io(io_err))
        }
    }
}

pub fn get_typed_callback<T, F, Fut>(
    callback: F,
) -> impl Fn(Payload, Client) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static
where
    T: serde::de::DeserializeOwned + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let callback = Arc::new(callback);

    move |payload: Payload, _socket: Client| {
        let callback = callback.clone();
        Box::pin(async move {
            match parse_payload_to_type::<T>(payload) {
                Ok(items) => {
                    for item in items {
                        let callback = callback.clone();
                        tokio::spawn(async move {
                            callback(item).await;
                        });
                    }
                }
                Err(e) => {
                    error!("Failed to parse payload: {e}");
                }
            }
        })
    }
}
