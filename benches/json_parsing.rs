use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::Value;

// IMPORTANT: SDK does not parse incoming JSON messages.
// These benchmarks measure JSON parsing performance that users may perform
// in their callbacks. This is NOT an SDK bottleneck, but user-level workload.

// Example messages that may come from the exchange
const RPC_RESPONSE_SMALL: &str = r#"{"id":123,"result":{"status":"ok"}}"#;
const MARKET_PRICE_MESSAGE: &str = r#"{"type":"MarketPrice","productId":"ETH-USD","price":"2500.50","timestamp":1234567890}"#;
const ORDERBOOK_MESSAGE: &str = r#"{"type":"BookDepth","productId":"ETH-USD","bids":[["2500.00","1.5"],["2499.50","2.0"]],"asks":[["2501.00","1.2"],["2501.50","0.8"]]}"#;
const LARGE_MESSAGE: &str = r#"{"type":"TradeFill","productId":"ETH-USD","trades":[{"price":"2500.00","size":"1.5","side":"buy","timestamp":1234567890},{"price":"2500.10","size":"0.8","side":"sell","timestamp":1234567891},{"price":"2500.20","size":"2.0","side":"buy","timestamp":1234567892}]}"#;

fn bench_full_parse_rpc_response(c: &mut Criterion) {
    c.bench_function("json_parse_rpc_response", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(black_box(RPC_RESPONSE_SMALL)).unwrap();
        });
    });
}

fn bench_full_parse_market_price(c: &mut Criterion) {
    c.bench_function("json_parse_market_price", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(black_box(MARKET_PRICE_MESSAGE)).unwrap();
        });
    });
}

fn bench_full_parse_orderbook(c: &mut Criterion) {
    c.bench_function("json_parse_orderbook", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(black_box(ORDERBOOK_MESSAGE)).unwrap();
        });
    });
}

fn bench_full_parse_large_message(c: &mut Criterion) {
    c.bench_function("json_parse_large_message", |b| {
        b.iter(|| {
            let _: Value = serde_json::from_str(black_box(LARGE_MESSAGE)).unwrap();
        });
    });
}

fn bench_fast_key_check_id(c: &mut Criterion) {
    c.bench_function("json_fast_check_id", |b| {
        b.iter(|| {
            black_box(RPC_RESPONSE_SMALL).contains("\"id\":");
        });
    });
}

fn bench_fast_key_check_type(c: &mut Criterion) {
    c.bench_function("json_fast_check_type", |b| {
        b.iter(|| {
            black_box(MARKET_PRICE_MESSAGE).contains("\"type\":");
        });
    });
}

fn bench_conditional_parse_after_id_check(c: &mut Criterion) {
    c.bench_function("json_conditional_parse_after_id_check", |b| {
        b.iter(|| {
            let text = black_box(RPC_RESPONSE_SMALL);
            if text.contains("\"id\":") {
                let _: Value = serde_json::from_str(text).unwrap();
            }
        });
    });
}

fn bench_conditional_parse_after_type_check(c: &mut Criterion) {
    c.bench_function("json_conditional_parse_after_type_check", |b| {
        b.iter(|| {
            let text = black_box(MARKET_PRICE_MESSAGE);
            if text.contains("\"type\":") {
                let _: Value = serde_json::from_str(text).unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_full_parse_rpc_response,
    bench_full_parse_market_price,
    bench_full_parse_orderbook,
    bench_full_parse_large_message,
    bench_fast_key_check_id,
    bench_fast_key_check_type,
    bench_conditional_parse_after_id_check,
    bench_conditional_parse_after_type_check
);
criterion_main!(benches);
