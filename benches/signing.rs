// benches/submit_order.rs
//
// Run:
//   cargo bench
//
// Notes:
// - This bench measures the *client-side work* inside submit_order (hashmap lookups,
//   signing context + message build + signature encoding + dto build).
// - It avoids network I/O by factoring the “build/sign DTO” path into a helper used by the bench.
// - If you cannot modify library code, see the alt section at bottom (bench a wrapper/newtype).

use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use ethereal_rust_sdk::{
    async_client::client::HttpClient,
    enums::Environment,
    models::{
        OrderSide, OrderType, SubmitOrderDto, SubmitOrderDtoData, SubmitOrderLimitDtoData,
        TimeInForce,
    },
    signable_messages::TradeOrder,
    signing::{to_scaled_e9, Eip712, SigningContext},
    with_signing_fields,
};
use ethers::utils::hex;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tokio::runtime::Runtime;

pub async fn create_test_client() -> anyhow::Result<HttpClient> {
    let env = Environment::Testnet;
    let private_key = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a";
    let http_client = HttpClient::new(env, private_key, None).await;
    Ok(http_client)
}

#[allow(clippy::too_many_arguments)]
pub fn build_submit_order_dto_for_bench(
    client: &HttpClient,
    ticker: &str,
    quantity: Decimal,
    price: Decimal,
    side: OrderSide,
    _type: OrderType,
    time_in_force: TimeInForce,
    post_only: bool,
    reduce_only: bool,
    expires_at: Option<f64>,
) -> Result<SubmitOrderDto, Box<dyn std::error::Error>> {
    if !client.product_hashmap.contains_key(ticker) {
        return Err(format!("Ticker {ticker} not found").into());
    }
    let product_info = client.product_hashmap.get(ticker).unwrap();
    let ctx = SigningContext::new(&client.wallet, &client.subaccounts[0]);
    let message = with_signing_fields!(
        eip_signing_fields,
        ctx,
        TradeOrder {
            quantity: to_scaled_e9(quantity)?,
            price: to_scaled_e9(price)?,
            reduce_only,
            side: side as u8,
            engine_type: product_info.engine_type.to_string().parse()?,
            product_id: product_info.onchain_id.to_string().parse()?,
        }
    );
    let signature = message.sign(client.env, &client.wallet)?;

    let order_dto = with_signing_fields!(
        dto_signing_fields,
        ctx,
        SubmitOrderLimitDtoData {
            quantity: quantity,
            price: price,
            side,
            onchain_id: product_info.onchain_id,
            engine_type: product_info.engine_type,
            reduce_only: Some(reduce_only),
            post_only,
            expires_at,
            time_in_force,
            ..Default::default()
        }
    );
    let dto = SubmitOrderDto {
        data: SubmitOrderDtoData::SubmitOrderLimitDtoData(order_dto),
        signature: "0x".to_string() + &hex::encode(signature.to_vec()),
    };
    Ok(dto)
}

// Helper used by the bench to synchronously produce a ready client. This wraps the
// async `create_test_client` so the bench setup can run outside of the timed path.
fn make_client_ready_for_bench() -> HttpClient {
    let rt = Runtime::new().expect("tokio runtime");
    rt.block_on(async {
        create_test_client()
            .await
            .expect("failed to create test client")
    })
}
fn bench_submit_order_build_and_sign(c: &mut Criterion) {
    // Tokio runtime once, reused.
    let rt = Runtime::new().expect("tokio runtime");

    // Build client ONCE (outside timing) to avoid including initialization costs.
    // Ensure product_hashmap contains the ticker and wallet/subaccounts are configured.
    let client = Arc::new(make_client_ready_for_bench());

    // Fixed inputs, keep them realistic.
    let ticker = "BTC-USD";
    let quantity = dec!(0.01);
    let price = dec!(50_000.0);
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;
    let time_in_force = TimeInForce::Ioc;
    let post_only = true;
    let reduce_only = false;
    let expires_at = None;

    c.bench_function("submit_order/build_sign_encode_dto", |b| {
        let client = client.clone();

        b.iter_batched(
            || {
                // Clone cheap inputs per-iter. Avoid allocating in the hot path unless you want
                // to measure allocations.
                (
                    client.clone(),
                    ticker.to_string(),
                    quantity,
                    price,
                    side,
                    r#type,
                    time_in_force,
                    post_only,
                    reduce_only,
                    expires_at,
                )
            },
            |(
                client,
                ticker,
                quantity,
                price,
                side,
                r#type,
                tif,
                post_only,
                reduce_only,
                expires_at,
            )| {
                rt.block_on(async move {
                    // Bench the extracted helper (no network).
                    let _dto = build_submit_order_dto_for_bench(
                        &client,
                        &ticker,
                        quantity,
                        price,
                        side,
                        r#type,
                        tif,
                        post_only,
                        reduce_only,
                        expires_at,
                    )
                    .expect("dto build/sign failed");
                });
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_submit_order_build_and_sign);
criterion_main!(benches);
