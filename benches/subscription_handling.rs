use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use ethereal_rust_sdk::types::ProductSubscriptionMessage;
use serde_json::Value;

// Create test subscriptions
fn create_subscription(channel: &str, product_id: &str) -> Value {
    let message = ProductSubscriptionMessage {
        msg_type: channel.to_string(),
        product_id: product_id.to_string(),
    };
    serde_json::to_value(&message).unwrap()
}

// Benchmark Vec<Value> cloning (as in connect())
fn bench_subscriptions_clone_10(c: &mut Criterion) {
    let mut subscriptions = Vec::new();
    for i in 0..10 {
        subscriptions.push(create_subscription("MarketPrice", &format!("PROD-{}", i)));
    }

    c.bench_function("subscriptions_clone_10", |b| {
        b.iter(|| {
            let _ = black_box(&subscriptions).clone();
        });
    });
}

fn bench_subscriptions_clone_50(c: &mut Criterion) {
    let mut subscriptions = Vec::new();
    for i in 0..50 {
        subscriptions.push(create_subscription("MarketPrice", &format!("PROD-{}", i)));
    }

    c.bench_function("subscriptions_clone_50", |b| {
        b.iter(|| {
            let _ = black_box(&subscriptions).clone();
        });
    });
}

fn bench_subscriptions_clone_100(c: &mut Criterion) {
    let mut subscriptions = Vec::new();
    for i in 0..100 {
        subscriptions.push(create_subscription("MarketPrice", &format!("PROD-{}", i)));
    }

    c.bench_function("subscriptions_clone_100", |b| {
        b.iter(|| {
            let _ = black_box(&subscriptions).clone();
        });
    });
}

// Benchmark Value::to_string() (as in resubscribe)
fn bench_value_to_string_single(c: &mut Criterion) {
    let subscription = create_subscription("MarketPrice", "ETH-USD");

    c.bench_function("value_to_string_single", |b| {
        b.iter(|| {
            let _ = black_box(&subscription).to_string();
        });
    });
}

fn bench_value_to_string_10(c: &mut Criterion) {
    let subscriptions: Vec<Value> = (0..10)
        .map(|i| create_subscription("MarketPrice", &format!("PROD-{}", i)))
        .collect();

    c.bench_function("value_to_string_10", |b| {
        b.iter(|| {
            for sub in subscriptions.iter() {
                let _ = black_box(sub).to_string();
            }
        });
    });
}

fn bench_value_to_string_50(c: &mut Criterion) {
    let subscriptions: Vec<Value> = (0..50)
        .map(|i| create_subscription("MarketPrice", &format!("PROD-{}", i)))
        .collect();

    c.bench_function("value_to_string_50", |b| {
        b.iter(|| {
            for sub in subscriptions.iter() {
                let _ = black_box(sub).to_string();
            }
        });
    });
}

fn bench_value_to_string_100(c: &mut Criterion) {
    let subscriptions: Vec<Value> = (0..100)
        .map(|i| create_subscription("MarketPrice", &format!("PROD-{}", i)))
        .collect();

    c.bench_function("value_to_string_100", |b| {
        b.iter(|| {
            for sub in subscriptions.iter() {
                let _ = black_box(sub).to_string();
            }
        });
    });
}

// Benchmark full resubscribe cycle (to_string + Payload creation)
fn bench_resubscribe_10(c: &mut Criterion) {
    let subscriptions: Vec<Value> = (0..10)
        .map(|i| create_subscription("MarketPrice", &format!("PROD-{}", i)))
        .collect();

    c.bench_function("resubscribe_10", |b| {
        b.iter(|| {
            for sub in subscriptions.iter() {
                let json_str = black_box(sub).to_string();
                // Simulate Payload creation (without actual emit)
                let _payload = rust_socketio::Payload::from(json_str);
            }
        });
    });
}

fn bench_resubscribe_50(c: &mut Criterion) {
    let subscriptions: Vec<Value> = (0..50)
        .map(|i| create_subscription("MarketPrice", &format!("PROD-{}", i)))
        .collect();

    c.bench_function("resubscribe_50", |b| {
        b.iter(|| {
            for sub in subscriptions.iter() {
                let json_str = black_box(sub).to_string();
                let _payload = rust_socketio::Payload::from(json_str);
            }
        });
    });
}

fn bench_resubscribe_100(c: &mut Criterion) {
    let subscriptions: Vec<Value> = (0..100)
        .map(|i| create_subscription("MarketPrice", &format!("PROD-{}", i)))
        .collect();

    c.bench_function("resubscribe_100", |b| {
        b.iter(|| {
            for sub in subscriptions.iter() {
                let json_str = black_box(sub).to_string();
                let _payload = rust_socketio::Payload::from(json_str);
            }
        });
    });
}

// Benchmark subscription addition (push to Vec)
#[allow(clippy::vec_init_then_push)]
fn bench_subscription_push(c: &mut Criterion) {
    let subscription = create_subscription("MarketPrice", "ETH-USD");

    c.bench_function("subscription_push", |b| {
        b.iter(|| {
            let mut subscriptions = vec![];
            subscriptions.push(black_box(&subscription).clone());
        });
    });
}

criterion_group!(
    benches,
    bench_subscriptions_clone_10,
    bench_subscriptions_clone_50,
    bench_subscriptions_clone_100,
    bench_value_to_string_single,
    bench_value_to_string_10,
    bench_value_to_string_50,
    bench_value_to_string_100,
    bench_resubscribe_10,
    bench_resubscribe_50,
    bench_resubscribe_100,
    bench_subscription_push
);
criterion_main!(benches);
