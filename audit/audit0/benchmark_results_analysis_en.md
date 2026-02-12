# Performance Benchmark Results Analysis

**Date:** January 2025  
**Project:** Ethereal Rust SDK

**Important Note:** SDK does not parse incoming JSON messages. JSON parsing is user-level callback workload (if user does it). Relevant benchmarks for SDK: subscription serialization, subscription cloning, and resubscription cost.

**Measurement Methodology:**
- All numbers in tables are marked as:
  - **(measured)** - real results from Criterion benchmarks
  - **(estimate)** - theoretical estimates, require measurement after implementing optimizations
- Tool: Criterion.rs
- Number of measurements: 100 per operation
- Build mode: Release (optimized)

## Results Overview

Benchmarks were run to measure performance of critical SDK components. Results show good performance of basic operations and identify optimization opportunities in resubscription operations.

## Detailed Results

### 1. Subscription Serialization (relevant to SDK)

**Source:** Results from `cargo bench --bench serialization` (measured)

| Operation | Time (ns) | Assessment |
|----------|-----------|------------|
| Product Subscription | 105.67 (measured) | ✅ Excellent |
| Subaccount Subscription | 104.23 (measured) | ✅ Excellent |
| Value → String | 298.50 (measured) | ✅ Good |
| Value Cloning | 78.56 (measured) | ✅ Excellent |

**Conclusions:**
- Subscription message serialization is very fast (~105 ns, measured)
- `Value` to string conversion takes ~298.50 ns (measured) - ~2.8x more than serialization
- JSON value cloning is relatively cheap (~78.56 ns, measured)

**Critical Finding:** ⚠️ On resubscription, `Value::to_string()` is executed for each channel, taking ~298.50 ns per channel (measured). With 50 subscriptions this is ~14.9 µs, with 100 subscriptions ~29.9 µs.

### 2. Subscription Cloning on Connection (relevant to SDK)

**Source:** Results from `cargo bench --bench subscription_handling` (measured)

| Operation | Subscription Count | Time (ns) | Assessment |
|----------|-------------------|----------|------------|
| Vec<Value> Cloning | 10 | 961.64 ns (measured) | ✅ Good |
| Vec<Value> Cloning | 50 | 10.284 µs (measured) | ✅ Acceptable |
| Vec<Value> Cloning | 100 | 20.697 µs (measured) | ✅ Acceptable |

**Conclusions:**
- `Vec<Value>` cloning scales linearly with subscription count (measured)
- With large number of subscriptions (>50), cloning cost becomes noticeable (10.284 µs for 50 subscriptions, 20.697 µs for 100)
- This happens on each connection (`connect()`)

**Recommendation:**
- ✅ **Caching Serialized Subscriptions** (recommendation #3) will eliminate need to clone `Value`:
  - Use `Vec<String>` or `Vec<Arc<str>>` instead of `Vec<Value>`
  - **Important:** `String::clone()` still allocates and copies content, so gain will be moderate (~1.2-3x CPU, estimate)
  - For significant speedup (~10-20x), need to use `Arc<str>` or `Bytes` to avoid copying content
  - **Expected Gain:**
    - With `Vec<String>`: ~1.2-3x faster cloning (estimate, requires measurement)
    - With `Vec<Arc<str>>`: ~10-20x faster cloning (estimate, requires measurement)
  - With 50 subscriptions using `Vec<String>`: 10.284 µs → ~3-8 µs (estimate)
  - With 50 subscriptions using `Vec<Arc<str>>`: 10.284 µs → ~0.5-1 µs (estimate)

### 3. Resubscription Cost - relevant to SDK

**Source:** Results from `cargo bench --bench subscription_handling` (measured)

| Operation | Subscription Count | Time (ns) | Assessment |
|----------|-------------------|----------|------------|
| `Value::to_string()` | 1 | 195.89 ns (measured) | ✅ Good |
| `Value::to_string()` | 10 | 1.9532 µs (measured) | ✅ Good |
| `Value::to_string()` | 50 | 9.7719 µs (measured) | ⚠️ Can optimize |
| `Value::to_string()` | 100 | 19.456 µs (measured) | ⚠️ Can optimize |
| Full resubscribe cycle | 10 | 3.7620 µs (measured) | ✅ Good |
| Full resubscribe cycle | 50 | 18.514 µs (measured) | ⚠️ Can optimize |
| Full resubscribe cycle | 100 | 36.984 µs (measured) | ⚠️ Can optimize |

**Conclusions:**
- Resubscription cost scales linearly with subscription count (measured)
- With large number of subscriptions (>50), cost becomes noticeable (9.7719 µs for 50 subscriptions, 19.456 µs for 100)
- Full resubscribe cycle includes `Value::to_string()` + `Payload` creation, adding ~1.8 µs overhead for 10 subscriptions, ~8.7 µs for 50, ~17.5 µs for 100

**Critical Finding:** ⚠️ On resubscription to 50 channels, ~18.5 µs CPU work is performed (measured: resubscribe_50), with 100 channels ~37.0 µs (measured: resubscribe_100). This happens on each reconnection.

**Recommendation:**
- ✅ **Caching Serialized Subscriptions** (recommendation #3) will give significant gain:
  - Instead of `Value::to_string()` (~195.89 ns per channel, measured) use cached string
  - **Important:** Gain depends on implementation:
    - With `String::clone()`: ~1.2-3x faster (estimate, `String::clone()` still copies content)
    - With `Arc<str>` or `Bytes`: ~10-20x faster (estimate, avoids copying content)
  - **Expected Gain (with `Arc<str>`): ~10-20x faster resubscription (estimate, requires measurement after implementation)**
  - With 50 subscriptions using `Arc<str>`: 18.5 µs → ~0.9-1.8 µs (estimate)
  - With 100 subscriptions using `Arc<str>`: 37.0 µs → ~1.8-3.7 µs (estimate)

### 4. JSON Parsing (for reference - user workload)

**Note:** These benchmarks measure JSON parsing performance that users may perform in their callbacks. This is NOT an SDK bottleneck.

| Operation | Time (ns) | Message Size | Assessment |
|----------|-----------|--------------|------------|
| RPC response | 203.50 | Small (~30 bytes) | ✅ Excellent |
| Market Price | 274.73 | Medium (~100 bytes) | ✅ Good |
| Orderbook | 764.61 | Large (~300 bytes) | ✅ Acceptable |
| Large Message | 1,392.3 | Very large (~500+ bytes) | ✅ Acceptable |
| Fast "id" check | 5.29 | - | ✅ Excellent (38x faster) |
| Fast "type" check | 7.85 | - | ✅ Excellent (35x faster) |

**Important:** This relates to user JSON parsing in callbacks. SDK itself does not parse JSON - passes `Payload` to user without parsing.

**Conclusions:**
- JSON parsing for users is fast (~200-300 ns for typical messages)
- Fast key checking is 35-40 times faster than full parsing
- This can be useful for users who want to filter messages without full parsing

### 5. Subscription Management (old results - not relevant)

**Note:** Old benchmarks with `HashMap` and `Mutex` are not relevant to current SDK architecture, as SDK uses `Vec<Value>` without locks. These results removed from analysis.

## Comparative Analysis

### Serialization: Current vs Cached

**Important:** Gain depends on caching implementation:
- `String::clone()` still copies content → moderate gain (~1.2-3x)
- `Arc<str>` or `Bytes` avoids copying → significant gain (~10-20x)

```
Current (Value::to_string()):     195.89 ns per channel (measured from subscription_handling)
Cached (String::clone()):          ~65-160 ns per channel (estimate, depends on JSON size)
Cached (Arc<str>::clone()):        ~10 ns per channel (estimate, only pointer cloning)
─────────────────────────────────────────────
Expected Gain (String):            ~1.2-3x (estimate, requires measurement)
Expected Gain (Arc<str>):          ~10-20x (estimate, requires measurement)
```

**Application (based on real measurements, with `Arc<str>`):**
- On resubscription to 10 channels:
  - Current: 3.7620 µs (measured: resubscribe_10)
  - Cached: ~0.2 µs (estimate: 10 × 10 ns + Payload overhead)
  - **Expected Gain: ~19x faster (estimate)**

- On resubscription to 50 channels:
  - Current: 18.514 µs (measured: resubscribe_50)
  - Cached: ~0.9 µs (estimate: 50 × 10 ns + Payload overhead)
  - **Expected Gain: ~20x faster (estimate)**

- On resubscription to 100 channels:
  - Current: 36.984 µs (measured: resubscribe_100)
  - Cached: ~1.8 µs (estimate: 100 × 10 ns + Payload overhead)
  - **Expected Gain: ~20x faster (estimate)**

### Subscription Cloning: Current vs Cached

**Important:** Gain depends on implementation:
- `Vec<String>::clone()` still copies content of each string → moderate gain (~1.2-3x)
- `Vec<Arc<str>>::clone()` clones only pointers → significant gain (~10-20x)

```
Current (Vec<Value> clone):        96-207 ns per element (measured: 961.64 ns/10, 10.284 µs/50, 20.697 µs/100)
Cached (Vec<String> clone):       ~32-69 ns per element (estimate, depends on JSON size)
Cached (Vec<Arc<str>> clone):     ~5-10 ns per element (estimate, only pointer cloning)
─────────────────────────────────────────────
Expected Gain (String):            ~1.2-3x (estimate, requires measurement)
Expected Gain (Arc<str>):          ~10-20x (estimate, requires measurement)
```

**Application (based on real measurements, with `Vec<Arc<str>>`):**
- On connection with 50 subscriptions:
  - Current: 10.284 µs (measured: subscriptions_clone_50)
  - Cached: ~0.5 µs (estimate: 50 × 10 ns)
  - **Expected Gain: ~20x faster (estimate)**

- On connection with 100 subscriptions:
  - Current: 20.697 µs (measured: subscriptions_clone_100)
  - Cached: ~1.0 µs (estimate: 100 × 10 ns)
  - **Expected Gain: ~20x faster (estimate)**

## Identified Bottlenecks

### 1. Cost of `Value::to_string()` on Resubscription (Critical)

**Problem:**
- On each reconnection, `Value::to_string()` is executed for each channel
- Time: 195.89 ns per channel (measured: value_to_string_single)

**Impact (based on real measurements):**
- With 10 subscriptions: 1.9532 µs (measured: value_to_string_10)
- With 50 subscriptions: 9.7719 µs (measured: value_to_string_50)
- With 100 subscriptions: 19.456 µs (measured: value_to_string_100)

**Solution:**
- Cache serialized strings instead of `Value`
- Expected gain: ~10-20x faster resubscription (with `Arc<str>`, estimate) or ~1.2-3x (with `String::clone()`, estimate)

### 2. Cloning `Vec<Value>` on Connection (High Priority)

**Problem:**
- On each connection, entire subscription vector is cloned
- Time: 96-207 ns per element (measured: 961.64 ns/10, 10.284 µs/50, 20.697 µs/100)

**Impact (based on real measurements):**
- With 50 subscriptions: 10.284 µs (measured: subscriptions_clone_50)
- With 100 subscriptions: 20.697 µs (measured: subscriptions_clone_100)

**Solution:**
- Use `Vec<Arc<str>>` instead of `Vec<Value>` for maximum gain
- Alternative: `Vec<String>` for simplicity (but smaller gain)
- With `Vec<Arc<str>>`: expected gain ~10-20x faster cloning (estimate)
- With `Vec<String>`: expected gain ~1.2-3x faster cloning (estimate)

### 3. No Batching of Resubscriptions (High Priority)

**Problem:**
- Separate `emit()` sent for each channel
- Each message requires network round-trip

**Impact:**
- With 10 subscriptions: 10 network round-trips
- With 50 subscriptions: 50 network round-trips
- With 100 subscriptions: 100 network round-trips

**Solution:**
- Send one command with all channels (if API supports)
- Gain: 1 network round-trip instead of N

## Target Metrics After Optimizations

### After Implementing Cached Serialized Subscriptions

| Operation | Current Time | Target Time (Arc<str>) | Improvement |
|-----------|--------------|------------------------|-------------|
| Resubscription (10 channels) | 3.7620 µs (measured) | ~0.2 µs (estimate) | **~19x (estimate)** |
| Resubscription (50 channels) | 18.514 µs (measured) | ~0.9 µs (estimate) | **~20x (estimate)** |
| Resubscription (100 channels) | 36.984 µs (measured) | ~1.8 µs (estimate) | **~20x (estimate)** |
| Cloning (50 subscriptions) | 10.284 µs (measured) | ~0.5 µs (estimate) | **~20x (estimate)** |
| Cloning (100 subscriptions) | 20.697 µs (measured) | ~1.0 µs (estimate) | **~20x (estimate)** |

**Note:** Improvements are for implementation with `Arc<str>`. With `String::clone()`, expected gain will be ~1.2-3x (estimate).

### After Implementing Batching (if API supports)

| Operation | Current Time | Target Time | Improvement |
|-----------|--------------|-------------|-------------|
| Resubscription (10 channels) | ~10 network round-trips | ~1 round-trip | **10x** |
| Resubscription (50 channels) | ~50 network round-trips | ~1 round-trip | **50x** |
| Resubscription (100 channels) | ~100 network round-trips | ~1 round-trip | **100x** |

## Prioritized Recommendations

### Priority 1 (Critical - Immediately)

1. **Caching Serialized Subscriptions**
   - Expected gain: ~10-20x faster resubscription (estimate, with `Arc<str>`; with `String::clone()` ~1.2-3x)
   - Implementation ease: Medium (requires choice between `String` and `Arc<str>`)
   - Risk: Low
   - **Expected Improvement (with `Arc<str>`):** Resubscription on 50 channels: 18.5 µs (measured) → ~0.9-1.8 µs (estimate)

### Priority 2 (High - Soon)

2. **Batching Resubscriptions** (if API supports)
   - Gain: 10-100x fewer network round-trips
   - Implementation ease: Medium (requires API check)
   - Risk: Medium
   - **Expected Improvement:** Resubscription on 50 channels: 50 round-trips → 1 round-trip

### Priority 3 (Medium - When Possible)

3. **Async Connection** - UX improvement (not measured by benchmarks)
4. **Automatic Reconnection** - fault tolerance improvement (not measured by benchmarks)

## General Conclusions

### ✅ What Works Well

1. **Subscription Serialization** - very fast (~100 ns per message)
2. **Subscription Cloning** - acceptable for small number of subscriptions (<50)
3. **JSON Parsing** (for users) - fast (~200-300 ns for typical messages)

### ⚠️ What Can Be Improved

1. **Caching Serialized Subscriptions** - critical optimization with large effect (~10-20x with `Arc<str>`, ~1.2-3x with `String::clone()`, estimate)
2. **Batching Resubscriptions** - significant improvement with large number of subscriptions
3. **Async Connection** - UX improvement (not measured by benchmarks)

### 📊 Final Performance Assessment

**Current State:** ✅ Good
- Basic operations are fast (serialization ~105 ns, Value cloning ~78 ns)
- No critical bottlenecks in hot paths for small number of subscriptions (<10)
- With large number of subscriptions (>50), resubscription cost becomes noticeable:
  - Resubscription on 50 channels: 18.5 µs (measured)
  - Resubscription on 100 channels: 37.0 µs (measured)
  - Cloning 50 subscriptions: 10.3 µs (measured)
  - Cloning 100 subscriptions: 20.7 µs (measured)

**After Optimizations:** ✅ Excellent
- Resubscription will be ~10-20x faster (estimate, with `Arc<str>`; requires measurement)
- Subscription cloning will be ~10-20x faster (estimate, with `Arc<str>`; requires measurement)
- Fewer network round-trips with batching
- Improved UX with async connection

**Note:** Improvements are for implementation with `Arc<str>` or `Bytes`. With `String::clone()`, expected gain will be moderate (~1.2-3x, estimate).

## Notes on Metrics

- All values are marked as **(measured)** or **(estimate)**
- **(measured)** = real results from Criterion benchmarks:
  - `serialization`: Product Subscription (105.67 ns), Subaccount Subscription (104.23 ns), Value → String (298.50 ns), Clone Value (78.56 ns)
  - `subscription_handling`: Clone Vec<Value> (961.64 ns/10, 10.284 µs/50, 20.697 µs/100), Value::to_string() (195.89 ns/1, 1.9532 µs/10, 9.7719 µs/50, 19.456 µs/100), Resubscribe (3.7620 µs/10, 18.514 µs/50, 36.984 µs/100)
- **(estimate)** = theoretical estimates, require measurement after implementing optimizations

## Next Steps

1. ✅ Benchmarks executed and analyzed
2. ⏳ Implement cached serialized subscriptions (Priority 1)
3. ⏳ Check API support for batching (Priority 2)
4. ⏳ Implement batching resubscriptions (if supported)
5. ⏳ Re-run benchmarks after optimizations
6. ⏳ Measure improvements in real usage scenarios
