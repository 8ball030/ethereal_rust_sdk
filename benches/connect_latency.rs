// Note: This benchmark does not measure actual network connection latency.
// It only measures CPU overhead from polling operations.
//
// Analytical estimate of latency penalty from polling:
// - With polling interval = 100ms, expected average overshoot = interval/2 = 50ms
// - Worst-case overshoot = 100ms (if connection completes right after previous check)
//
// This means busy-wait polling adds on average 50ms latency to "connect ready" moment,
// and worst-case up to 100ms latency.

use criterion::{criterion_group, criterion_main, Criterion};
use std::{
    hint::black_box,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

// Measure flag check cost (minimal polling overhead)
fn bench_flag_check(c: &mut Criterion) {
    let flag = Arc::new(AtomicBool::new(false));

    c.bench_function("flag_check", |b| {
        b.iter(|| {
            let _ = black_box(&flag).load(Ordering::SeqCst);
        });
    });
}

criterion_group!(benches, bench_flag_check);
criterion_main!(benches);
