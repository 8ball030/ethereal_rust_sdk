mod common;

use ethereal_rust_sdk::{
    apis::order_api::OrderControllerListFillsBySubaccountIdParams,
    async_client::client::HttpClient,
    enums::Environment,
    models::{OrderFillDto, OrderSide},
    utils::create_client,
};
use log::info;
use uuid::Uuid;

use crate::common::TEST_PRIVATE_KEY;

async fn collect_exchange_trades(
    client: &HttpClient,
    subaccount_id: String,
    start_timestamp: Option<i64>,
) -> Result<Vec<OrderFillDto>, Box<dyn std::error::Error>> {
    let mut all_trades = Vec::new();
    let mut cursor: Option<String> = None;

    println!(
        "Collecting trades starting from timestamp: {:?}",
        start_timestamp
    );
    loop {
        let params = OrderControllerListFillsBySubaccountIdParams {
            subaccount_id: subaccount_id.clone(),
            created_after: start_timestamp.map(|ts| ts * 1000),
            cursor: cursor.clone(),
            order_by: Some("createdAt".to_string()),
            limit: Some(100),
            ..Default::default()
        };

        let result = client.order().list_fills_by_subaccount_id(params).await?;
        all_trades.extend(result.data);

        if !result.has_next.unwrap_or(false) {
            break;
        }
        cursor = result.next_cursor;
    }

    info!("Collected {} total trades", all_trades.len());
    Ok(all_trades)
}

fn write_trades_to_csv(trades: Vec<OrderFillDto>, http_client: &HttpClient) {
    let mut wtr =
        csv::Writer::from_path("collected_trades.csv").expect("Failed to create CSV writer");

    let from_product_id_to_symbol = |product_id: &Uuid| -> String {
        http_client
            .product_id_hashmap
            .get(product_id)
            .map(|product| product.ticker.clone())
            .unwrap_or_else(|| product_id.to_string())
    };

    wtr.write_record([
        "id",
        "subaccount_id",
        "ticker",
        "price",
        "quantity",
        "side",
        "timestamp",
    ])
    .expect("Failed to write CSV header");
    for trade in trades {
        wtr.write_record(&[
            trade.id.to_string(),
            trade.subaccount_id.clone().to_string(),
            from_product_id_to_symbol(&trade.product_id.clone()),
            trade.price.to_string(),
            trade.filled.to_string(),
            match trade.side {
                OrderSide::BUY => "BUY".to_string(),
                OrderSide::SELL => "SELL".to_string(),
            },
            trade.created_at.to_string(),
        ])
        .expect("Failed to write CSV record");
    }
    wtr.flush().expect("Failed to flush CSV writer");
}

#[tokio::main]
async fn main() {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Info).expect("log");
    let env = Environment::Mainnet;
    let (http_client, _ws_client) = create_client(env, TEST_PRIVATE_KEY, None).await.unwrap();
    info!("HTTP client and WS client created");

    let subaccount_id =
        std::env::var("SUBACCOUNT_ID").expect("SUBACCOUNT_ID environment variable not set");
    let start_timestamp = None; // You can set this to a specific timestamp if you want

    let trades = collect_exchange_trades(&http_client, subaccount_id, start_timestamp)
        .await
        .expect("Failed to collect trades");

    write_trades_to_csv(trades, &http_client);

    info!("Collected all trades and wrote to CSV successfully");
}
