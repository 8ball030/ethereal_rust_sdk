# Performance Benchmarking Guide

## Overview

A set of benchmarks has been created to measure the performance of critical components of Ethereal Rust SDK. Benchmarks use the `criterion` library, which provides statistically significant results and automatically generates reports.

## Dependency Installation

Dependencies are already added to `Cargo.toml`:
- `criterion` - benchmarking library
- `tokio-test` - utilities for testing async code

## Running Benchmarks

### Run All Benchmarks
```bash
cargo bench
```

### Run Specific Benchmark
```bash
cargo bench --bench json_parsing
cargo bench --bench serialization
cargo bench --bench subscription_handling
```

### Run with Filter
```bash
cargo bench --bench json_parsing -- "rpc_response"
```

## Benchmark Descriptions

### 1. `benches/json_parsing.rs`
**IMPORTANT:** SDK does not parse incoming JSON messages. These benchmarks measure JSON parsing performance that users may perform in their callbacks. This is NOT an SDK bottleneck, but user-level workload.

**Tested Scenarios:**
- Full parsing of RPC responses (small messages)
- Full parsing of market price messages (medium messages)
- Full parsing of orderbook messages (large messages)
- Full parsing of large messages with multiple data
- Fast key checking without full parsing
- Conditional parsing (only after key check)

**Metrics:**
- Parsing time per message (for users in callbacks)
- Full parsing vs fast checking comparison
- Impact of message size on performance

**Using Results:**
- For SDK users: determine if fast key checking should be used before full parsing
- For SDK users: measure impact of message size on performance
- **Does not relate to SDK optimization** - this is user workload

### 2. `benches/serialization.rs`
Measures subscription message serialization performance - **relevant to SDK**.

**Tested Scenarios:**
- Subscription message serialization with product_id (`serde_json::to_value`)
- Subscription message serialization with subaccount_id
- Serialization to string (`Value::to_string()`)
- JSON value cloning

**Metrics:**
- Serialization time per message (~100 ns)
- `Value` to string conversion time (~300 ns) - **critical for resubscription**
- JSON value cloning time (~80 ns)

**Using Results:**
- Assess impact of `Value::to_string()` on resubscription performance
- Determine need to cache serialized messages (recommendation #3)
- Measure impact of JSON value cloning on connection

### 3. `benches/subscription_handling.rs`
Measures subscription operation performance - **relevant to SDK**.

**Tested Scenarios:**
- `Vec<Value>` cloning on connection (10, 50, 100 subscriptions)
- `Value::to_string()` for single subscription
- `Value::to_string()` for N subscriptions (10, 50, 100) - **critical for resubscription**
- Full resubscribe cycle (to_string + Payload creation) for N subscriptions
- Subscription addition to `Vec` (push)

**Metrics:**
- `Vec<Value>` cloning time on connection
- `Value::to_string()` time per channel (~300 ns)
- Full resubscription cycle time for N channels
- Dependency on subscription count (10 / 50 / 100)

**Using Results:**
- Assess resubscription cost with large number of channels
- Determine need to cache serialized strings (recommendation #3)
- Measure impact of subscription count on connection time

## Results Interpretation

### Output Format

Criterion outputs results in the following format:
```
Benchmarking json_parse_rpc_response
Benchmarking json_parse_rpc_response: Warming up for 3.0000 s
Benchmarking json_parse_rpc_response: Collecting 100 samples in estimated 5.0000 s
Benchmarking json_parse_rpc_response: Analyzing
json_parse_rpc_response
                        time:   [123.45 ns 125.67 ns 128.90 ns]
                        change: [-5.2341% -2.1234% +1.2345%] (p = 0.05 > 0.05)
                        No change in performance detected.
```

### Key Metrics

- **time** - average operation execution time (in nanoseconds)
- **change** - performance change relative to previous run
- **p** - statistical significance of change

### Comparing Results

To compare results before and after optimizations:

1. **Run benchmarks before optimizations:**
   ```bash
   cargo bench > baseline.txt
   ```

2. **Implement optimizations in code**

3. **Run benchmarks after optimizations:**
   ```bash
   cargo bench > optimized.txt
   ```

4. **Compare results:**
   - Criterion automatically compares results with previous run
   - Results saved in `target/criterion/`

### Results Analysis

**Good Results (SDK):**
- Subscription serialization < 150 ns per message
- `Value::to_string()` < 350 ns per channel
- `Vec<Value>` cloning scales linearly with subscription count

**Problematic Results (SDK):**
- `Value::to_string()` > 300 ns per channel (becomes noticeable with large number of subscriptions)
- Resubscription on 50+ channels > 15 µs (can be optimized with caching)

**For Reference (user workload):**
- JSON parsing time < 500 ns for small messages (if user parses in callbacks)
- Fast key checking 35-40 times faster than full parsing

## Usage Recommendations

### Before Optimization

1. Run all benchmarks to get baseline metrics
2. Identify bottlenecks based on results
3. Determine optimization priorities

### After Optimization

1. Run benchmarks to measure improvements
2. Ensure optimizations did not degrade performance of other operations
3. Document performance changes

### Continuous Monitoring

1. Run benchmarks on each significant code change
2. Track performance degradation
3. Use CI/CD for automatic benchmark runs

## Additional Tools

### Profiling

For more detailed performance analysis, it is recommended to use:

- **perf** (Linux) - system profiler
- **flamegraph** - performance profile visualization
- **dhat-rs** - memory profiler

### Example perf usage:

```bash
perf record --call-graph=dwarf cargo bench --bench json_parsing
perf report
```

### Example flamegraph usage:

```bash
cargo install flamegraph
cargo flamegraph --bench json_parsing
```

## Expected Results

Based on code analysis, the following results are expected:

### JSON Parsing (for users in callbacks)
- Full parsing of small messages: ~200-500 ns
- Full parsing of large messages: ~1000-2000 ns
- Fast key checking: ~5-10 ns (35-40 times faster)

### Serialization (relevant to SDK)
- Subscription message serialization: ~100-150 ns
- `Value` to string conversion: ~250-350 ns (**critical for resubscription**)
- JSON value cloning: ~70-100 ns

### Subscription Management (relevant to SDK)
- `Vec<Value>` cloning (50 subscriptions): ~1000-2500 ns
- `Value::to_string()` per channel: ~250-350 ns
- Resubscription on 50 channels: ~12500-17500 ns (12.5-17.5 µs)
- Resubscription on 100 channels: ~25000-35000 ns (25-35 µs)

## Benchmark Status

### Relevant Benchmarks (measure SDK bottlenecks)
- ✅ `serialization.rs` - subscription serialization (`serde_json::to_value`, `Value::to_string()`)
- ✅ `subscription_handling.rs` - subscription operations (cloning `Vec<Value>`, `Value::to_string()`, resubscribe cost)
- ⚠️ `connect_latency.rs` - only CPU overhead from polling (does not measure actual network latency; analytical estimate: average overshoot = 50ms with interval=100ms)

### Reference Benchmarks (user workload)
- 📝 `json_parsing.rs` - JSON parsing in user callbacks (not an SDK bottleneck, for reference)

## Next Steps

After obtaining benchmark results:

1. Analyze results (see `benchmark_results_analysis_en.md`)
2. Determine optimization priorities (see `performance_analysis_en.md`)
3. Implement optimizations (see `optimization_recommendations_en.md`)
4. Re-run benchmarks to measure improvements
