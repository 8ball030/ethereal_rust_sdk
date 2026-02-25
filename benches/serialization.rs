use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use ethereal_rust_sdk::types::{ProductSubscriptionMessage, SubaccountSubscriptionMessage};
use serde_json::Value;

fn bench_serialize_product_subscription(c: &mut Criterion) {
    c.bench_function("serialize_product_subscription", |b| {
        let msg = ProductSubscriptionMessage {
            msg_type: "MarketPrice".to_string(),
            product_id: "ETH-USD".to_string(),
        };
        b.iter(|| {
            let _: Value = serde_json::to_value(black_box(&msg)).unwrap();
        });
    });
}

fn bench_serialize_subaccount_subscription(c: &mut Criterion) {
    c.bench_function("serialize_subaccount_subscription", |b| {
        let msg = SubaccountSubscriptionMessage {
            msg_type: "OrderFill".to_string(),
            subaccount_id: "0x1234567890abcdef".to_string(),
        };
        b.iter(|| {
            let _: Value = serde_json::to_value(black_box(&msg)).unwrap();
        });
    });
}

fn bench_serialize_to_string(c: &mut Criterion) {
    c.bench_function("serialize_to_string", |b| {
        let msg = ProductSubscriptionMessage {
            msg_type: "MarketPrice".to_string(),
            product_id: "ETH-USD".to_string(),
        };
        b.iter(|| {
            let json_value: Value = serde_json::to_value(black_box(&msg)).unwrap();
            let _ = json_value.to_string();
        });
    });
}

fn bench_clone_json_value(c: &mut Criterion) {
    c.bench_function("clone_json_value", |b| {
        let msg = ProductSubscriptionMessage {
            msg_type: "MarketPrice".to_string(),
            product_id: "ETH-USD".to_string(),
        };
        let json_value: Value = serde_json::to_value(&msg).unwrap();
        b.iter(|| {
            let _ = black_box(&json_value).clone();
        });
    });
}

criterion_group!(
    benches,
    bench_serialize_product_subscription,
    bench_serialize_subaccount_subscription,
    bench_serialize_to_string,
    bench_clone_json_value
);
criterion_main!(benches);
