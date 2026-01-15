# Performance Analysis and Bottlenecks

**Date:** January 2025  
**Project:** Ethereal Rust SDK

**Important Note:** SDK does not parse incoming JSON messages. JSON parsing is user-level callback workload (if user does it). SDK passes `Payload` from `rust_socketio` library to user callbacks without parsing.

## Identified Bottlenecks

### 1. Blocking WebSocket Connection

**Problem:**
- `connect()` method uses blocking wait loop:
  ```rust
  while !connected_flag.load(Ordering::SeqCst) {
      std::thread::sleep(Duration::from_millis(100));
  }
  ```
- This blocks execution thread during connection
- With network issues, can block thread for extended time

**Locations:**
- `src/ws_client.rs:71-73` - blocking connection wait

**Impact:**
- Blocks main application thread
- Cannot perform other operations during connection
- Poor integration with async applications

### 2. Excessive Subscription Cloning

**Problem:**
- On connection, entire subscription vector is cloned:
  ```rust
  let subscriptions = self.subscriptions.clone();
  ```
- On resubscription, string conversion is performed for each channel:
  ```rust
  Payload::from(sub.to_string())
  ```
- On subscription addition, JSON value is cloned:
  ```rust
  self.subscriptions.push(json_msg.clone());
  ```

**Locations:**
- `src/ws_client.rs:53` - subscription vector cloning
- `src/ws_client.rs:63` - string conversion on resubscription
- `src/ws_client.rs:98, 114` - JSON value cloning

**Impact:**
- Unnecessary memory allocations on each connection
- Data copying instead of moving
- Additional memory load with large number of subscriptions

### 3. Cost of `Value::to_string()` on Resubscription

**Problem:**
- On each reconnection, `sub.to_string()` is executed for each `Value` in subscription vector
- This happens in "open" callback during resubscription: `Payload::from(sub.to_string())`
- `Value::to_string()` performs JSON serialization, which is CPU-intensive
- With large number of subscriptions, this creates significant load

**Locations:**
- `src/ws_client.rs:63` - `Payload::from(sub.to_string())` in resubscription loop

**Impact:**
- CPU-intensive operation on each reconnection
- Unnecessary string allocations for each channel
- Slower subscription recovery with large number of channels

### 4. Cloning `Vec<Value>` on Connection

**Problem:**
- On connection, entire subscription vector is cloned:
  ```rust
  let subscriptions = self.subscriptions.clone();
  ```
- This happens to pass into `on("open", move |...| { ... })` callback
- With large number of subscriptions, cloning can be expensive

**Locations:**
- `src/ws_client.rs:53` - subscription vector cloning

**Impact:**
- Unnecessary memory allocations on each connection
- Copying all `Value` elements in vector
- Performance degradation with growing number of subscriptions

### 5. No Batching of Resubscriptions

**Problem:**
- On reconnection, separate `emit("subscribe", ...)` is sent for each channel in loop:
  ```rust
  for sub in subscriptions.iter() {
      _socket.emit("subscribe", Payload::from(sub.to_string()));
  }
  ```
- Could send one command with array of all channels (if server API supports)

**Locations:**
- `src/ws_client.rs:61-66` - resubscription loop one channel at a time

**Impact:**
- More network round-trips (N `emit()` calls for N subscriptions)
- More JSON serialization (N times `sub.to_string()`)
- Slower subscription recovery

### 6. Busy-wait on Connection

**Problem:**
- `connect()` method uses busy-wait loop with `thread::sleep(100ms)`:
  ```rust
  while !connected_flag.load(Ordering::SeqCst) {
      std::thread::sleep(Duration::from_millis(100));
  }
  ```
- This adds latency to client readiness (worst-case up to 100ms)
- Blocks execution thread

**Locations:**
- `src/ws_client.rs:71-73` - busy-wait loop waiting for connection

**Impact:**
- Adds latency to "connect ready" moment (up to 100ms worst-case)
- Blocks execution thread
- Inefficient CPU usage (polling instead of event-driven approach)

### 7. Cost of `ClientBuilder.clone()` on Callback Registration

**Problem:**
- On each callback registration, `ClientBuilder` is cloned:
  ```rust
  let builder = self.client_builder.clone().on(channel, callback);
  self.client_builder = builder;
  ```
- This happens on each `register_*_callback()` call
- Can be expensive operation with large number of callbacks

**Locations:**
- `src/ws_client.rs:121` - ClientBuilder cloning

**Impact:**
- Unnecessary allocations on callback registration
- Affects application "setup time"
- Potential degradation with large number of subscriptions

### 8. No Buffer Pool for Reuse

**Problem:**
- Temporary buffers may be created during message processing
- No buffer reuse to reduce allocations

**Impact:**
- Unnecessary allocations during message processing
- Additional load (allocations are still expensive in Rust)

## Metrics for Measurement

### Subscription Serialization
- Subscription message serialization time (`serde_json::to_value`)
- `Value` to string conversion time (`Value::to_string()`)
- Impact of subscription count on resubscription time (N subscriptions → N times `to_string()`)

### Subscription Cloning
- `Vec<Value>` cloning time on connection
- Impact of subscription count on cloning cost
- Individual `Value` element cloning time

### Resubscription (resubscribe cost)
- `Value::to_string()` execution time for N subscriptions
- `Payload` creation time for N subscriptions
- Dependency on subscription count (10 / 100 / 1000)

### Connection
- Connection establishment time
- Latency from busy-wait polling (impact of `sleep(100ms)` on client readiness)
- Impact of blocking operations on performance

### ClientBuilder Operations
- `ClientBuilder` cloning time on callback registration
- Impact of callback count on setup time

### JSON Parsing (for reference - user workload)
- Various message type parsing time (if user parses in callbacks)
- Full parsing vs fast key checking comparison
- Impact of message size on performance

## Problem Prioritization

### Critical (Priority 1)
1. **Blocking WebSocket Connection** - blocks execution thread
2. **No Reconnection Management** - poor fault tolerance

### High (Priority 2)
3. **Cost of `Value::to_string()` on Resubscription** - CPU-intensive operation on each reconnection
4. **Cloning `Vec<Value>` on Connection** - unnecessary allocations on each connection
5. **No Batching of Resubscriptions** - slower recovery (N network round-trips)

### Medium (Priority 3)
6. **Busy-wait on Connection** - adds latency to client readiness (up to 100ms worst-case)
7. **Blocking Loop in run_forever** - blocks execution thread
8. **Cost of `ClientBuilder.clone()`** - unnecessary allocations on callback registration (affects setup time)
9. **JSON Value Cloning on Subscription Addition** - unnecessary allocations

### Low (Priority 4)
10. **No Buffer Pool for Reuse** - optimization for high-load scenarios

## Measurement Recommendations

To assess impact of problems on performance, it is recommended:

1. **Run benchmarks** from `benches/` folder:
   - `serialization` - measure subscription serialization performance (relevant to SDK)
   - `subscription_handling` - measure resubscription cost for N channels (relevant to SDK)
   - `json_parsing` - measure JSON parsing performance (for reference - user workload in callbacks)

2. **Profile** real application:
   - Use `perf` or `flamegraph` for profiling
   - Measure execution time of critical operations (`Value::to_string()`, `Vec::clone()`, `ClientBuilder::clone()`)
   - Identify bottlenecks in real usage scenarios

3. **Load Testing**:
   - Testing with large number of subscriptions (10, 50, 100+)
   - Testing frequent reconnections (measure resubscribe cost)
   - Measure latency from busy-wait polling on connection
