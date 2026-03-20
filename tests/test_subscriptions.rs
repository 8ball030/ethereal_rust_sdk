use log::{info, Level::Info};
use simple_logger::init_with_level;

mod common;
use crate::common::create_test_ws_client;

#[tokio::test]
async fn ticker_sub() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = create_test_ws_client().await?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    let tx_arc = std::sync::Arc::new(tokio::sync::Mutex::new(Some(tx)));
    client.wait_for_connection().await;

    let _ = client
        .subscriptions()
        .ticker(["BTCUSD".to_string()].to_vec(), move |_msg| {
            // Parses into a json value initally
            let tx = tx_arc.clone();
            async move {
                info!("Got a ticker update");
                let _ = tx.lock().await.take().map(|tx| tx.send(()));
            }
        })
        .await;

    // wait for 20 seconds or until we receive the signal to exit
    tokio::select! {
        _ = rx => {
            info!("Received signal to exit after receiving the first update");
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            info!("Timeout reached without receiving an update");
            panic!("Test failed: Did not receive an instruments update within the timeout period");
        }
    }
    Ok(())
}
