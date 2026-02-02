# Ethereal Rust SDK — Follow-Up Audit (Current State Review)

**Project:** `ethereal_rust_sdk`  
**Inputs reviewed:**  
1) `audit/` folder (previous audit, benchmark guide/results, recommendations)  
2) current repository source code (post-audit state)

**Date:** 2026-02-02

---

## 0. Executive Summary

Overall, the repository has **materially improved** relative to the last audit baseline:

- The WebSocket client is now **fully async** and does not rely on blocking sleep loops.
- **Automatic reconnection** is implemented using the underlying async socket client’s reconnection settings.
- A **subscription cache** exists and is used for resubscription after reconnect.
- The repo contains a mature `benches/` suite aligned with the earlier benchmarking approach.

However, the current implementation still contains several **avoidable allocations and clones** in the subscription path, and a few **fragile design choices** (mainly around using `Arc<Vec<...>>` with `Arc::get_mut`). These issues can impact p99/p999 behavior during reconnect storms or when subscription counts increase.

This report focuses on: (1) what improved, (2) what remains, (3) concrete code-level recommendations.

---

## 0.1 Verification of This Report (Re-Audit)

The following was re-checked against the current `src/ws_client.rs` and related code (2026-02-02):

| Finding | Location | Verified |
|--------|----------|----------|
| Async `connect`, no blocking sleep | `ws_client.rs:65–138` | ✅ `connect()` is `async fn`, uses `run_till_event().await`, no `thread::sleep` |
| Reconnection: `reconnect_on_disconnect(true)`, delay, attempts | `ws_client.rs:100–104` | ✅ `.reconnect_on_disconnect(true)` `.reconnect_delay(10, 30)` `.max_reconnect_attempts(100)` |
| Subscriptions as `Arc<Vec<Value>>` | `ws_client.rs:39, 57` | ✅ `subscriptions: Arc<Vec<Value>>` |
| Resubscribe uses `sub.to_string()` | `ws_client.rs:86` | ✅ `Payload::from(sub.to_string())` in connect callback |
| `Arc::get_mut(...).expect(...)` on subscribe | `ws_client.rs:160–161, 178–179` | ✅ In `subscribe_with_product` and `subscribe_with_subaccount` |
| `json_msg.clone()` on push | `ws_client.rs:162, 180` | ✅ `subscriptions.push(json_msg.clone())` in both methods |
| `channel.to_string()`, `product_id.to_string()` | `ws_client.rs:148–149, 166–167` | ✅ New `String` per call |
| `expect()` in callbacks (tx.send, emit) | `ws_client.rs:79–80, 87–88, 107–108, 121–122` | ✅ Four places in connect/reconnect/close callbacks |
| Per-subscription `info!` in resubscribe loop | `ws_client.rs:84` | ✅ `info!("Subscribing to channel: {sub:?}")` inside loop |
| `run_till_event`: `state_rx.changed().await.unwrap()` | `ws_client.rs:143` | ✅ Can panic if all senders are dropped |

All conclusions of this report match the current codebase.

---

## 1. What the Previous Audit Recommended

From `audit/optimization_recommendations_en.md`, the highest-priority items were:

1. **Async WebSocket Connection** (replace blocking wait loops)  
2. **Automatic Reconnection** (with backoff, retry limits)  
3. **Caching Serialized Subscriptions** (critical)  
4. Optional: batching resubscriptions, avoiding clones on subscription addition, string conversion optimization, ClientBuilder optimization, and async `run_forever`.

---

## 2. Current State vs Audit Recommendations

### 2.1 Async WebSocket Connection — ✅ Implemented

The current client exposes an async `connect` method and uses the async socket client’s connect workflow.

Evidence (excerpt):  
```rust
pub async fn connect(&mut self) -> Result<(), Error> {
        info!("Connecting websocket...");
        let builder = self.client_builder.take().expect("connect called twice");

        // bool channel to indicate connection established.

        let subscriptions = Arc::clone(&self.subscriptions); // cheap clone
        let connection_tx = self.state_tx.clone();

        let connect_cb = move |_payload: Payload, socket: Client| {
            {
                let subscriptions = subscriptions.clone();
                let tx = connection_tx.clone();
                tx.send(ConnectionState::Connected)
                    .expect("Failed to send connected signal");
                async move {
                    info!("Websocket connected");
                    for sub in subscriptions.iter() {
                        info!("Subscribing to channel: {sub:?}");
                        socket
                            .emit("subscribe", Payload::from(sub.to_string()))
                            .await
                            .expect("Failed to emit subscribe message");
                    }
                }
            }
            .boxed()
        };

        let url = self.connection_url.clone();
        let disconnect_tx = self.state_tx.clone();
        let error_tx = self.state_tx.clone();
        self.client = Some(
            builder
                .on("open", connect_cb)
                .reconnect_on_disconnect(true)
                .reconnect_delay(10, 30)
                .max_reconnect_attempts(100)
                .on_reconnect(move || {
                    error!("Websocket reconnecting...");
                    let tx = disconnect_tx.clone();
                    tx.send(ConnectionState::Reconnecting)
                        .expect("Failed to send reconnecting signal");
                    let url = url.clone();
                    async move {
                        error!("Websocket reconnecting...");
                        let mut settings = ReconnectSettings::new();
                        settings.address(url);
                        settings
                    }
                    .boxed()
                })
                .on("close", move |err: Payload, _socket: Client| {
                    error!("Websocket closed......");
                    let tx = error_tx.clone();
                    tx.send(ConnectionState::Disconnected)
                        .expect("Failed to send disconnected signal");
                    async move {
                        error!("Websocket error: {:?}", err);
                    }
                    .boxed()
                })
                .connect()
                .await?,
        );
        match self.run_till_event().await {
            ConnectionState::Connected => {
                info!("All connected!")
            }
            _ => return Err(Error::StoppedEngineIoSocket),
        }

        Ok(())
    }
```

This removes the earlier “sleep until connected” pattern and is aligned with the audit’s highest-priority recommendation.

---

### 2.2 Automatic Reconnection — ✅ Implemented

The WebSocket builder config includes reconnection behavior:

```rust
            {
                let subscriptions = subscriptions.clone();
                let tx = connection_tx.clone();
                tx.send(ConnectionState::Connected)
                    .expect("Failed to send connected signal");
                async move {
                    info!("Websocket connected");
                    for sub in subscriptions.iter() {
                        info!("Subscribing to channel: {sub:?}");
                        socket
                            .emit("subscribe", Payload::from(sub.to_string()))
                            .await
                            .expect("Failed to emit subscribe message");
                    }
                }
            }
            .boxed()
        };

        let url = self.connection_url.clone();
        let disconnect_tx = self.state_tx.clone();
        let error_tx = self.state_tx.clone();
        self.client = Some(
            builder
                .on("open", connect_cb)
                .reconnect_on_disconnect(true)
                .reconnect_delay(10, 30)
                .max_reconnect_attempts(100)
                .on_reconnect(move || {
                    error!("Websocket reconnecting...");
                    let tx = disconnect_tx.clone();
                    tx.send(ConnectionState::Reconnecting)
                        .expect("Failed to send reconnecting signal");
                    let url = url.clone();
                    async move {
                        error!("Websocket reconnecting...");
                        let mut settings = ReconnectSettings::new();
                        settings.address(url);
                        settings
                    }
                    .boxed()
                })
                .on("close", move |err: Payload, _socket: Client| {
                    error!("Websocket closed......");
                    let tx = error_tx.clone();
                    tx.send(ConnectionState::Disconnected)
                        .expect("Failed to send disconnected signal");
                    async move {
                        error!("Websocket error: {:?}", err);
                    }
```

Notes:
- The code sets `reconnect_on_disconnect(true)` and configures reconnect delay/attempts.
- On reconnect, the code transitions internal state to `Reconnecting`, which is helpful for external observability.

---

### 2.3 Caching Serialized Subscriptions — ⚠️ Partially Implemented

There is a cache of subscriptions stored in the client:

```rust
pub struct WsClient {
    ...
    subscriptions: Arc<Vec<Value>>,
    ...
}
```

Subscriptions are serialized into `serde_json::Value` at subscription registration time:

```rust
                        .expect("Failed to send disconnected signal");
                    async move {
                        error!("Websocket error: {:?}", err);
                    }
                    .boxed()
                })
                .connect()
                .await?,
        );
        match self.run_till_event().await {
            ConnectionState::Connected => {
                info!("All connected!")
            }
            _ => return Err(Error::StoppedEngineIoSocket),
        }

        Ok(())
    }

    // runs till one of the state changes is detected.
    pub async fn run_till_event(&mut self) -> ConnectionState {
        self.state_rx.changed().await.unwrap();
        *self.state_rx.borrow()
    }

    fn subscribe_with_product(&mut self, channel: &str, product_id: &str) {
        let message = ProductSubscriptionMessage {
            msg_type: channel.to_string(),
            product_id: product_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&message) {
            Ok(v) => v,
            Err(e) => {
                error!("serialization_failed channel={channel} error={e}");
                return;
            }
        };
        let subscriptions = Arc::get_mut(&mut self.subscriptions)
            .expect("Failed to get mutable reference to subscriptions");
        subscriptions.push(json_msg.clone());
    }

    fn subscribe_with_subaccount(&mut self, channel: &str, subaccount_id: &str) {
        let message = SubaccountSubscriptionMessage {
            msg_type: channel.to_string(),
            subaccount_id: subaccount_id.to_string(),
        };

        let json_msg = match serde_json::to_value(&message) {
```

And on connect/reconnect, the client iterates `subscriptions` and re-emits them:

```rust
for sub in subscriptions.iter() {
    socket.emit("subscribe", Payload::from(sub.to_string())).await
        .expect("Failed to emit subscribe message");
}
```

**What is good:**
- This avoids reconstructing the subscription objects from scratch on reconnect.
- It makes reconnection behavior deterministic and allows benchmarking resubscription overhead.

**What is missing (performance):**
- In the connect callback, each `Value` is converted back to a string via `sub.to_string()` per resubscription.
- This reintroduces per-resubscribe serialization work and allocations.

**Recommendation:** cache `String` payloads (or `Bytes`) rather than `Value`:
- Build `String` once at subscription registration time
- On reconnect, `emit` the cached string directly

This matches the earlier audit’s “Critical: caching serialized subscriptions” intent more closely.

---

### 2.4 Batching Resubscriptions — ❌ Not Implemented (and may not be possible)

Currently, the code emits subscriptions individually in a loop.
Batching would require server support (e.g., a `subscribeMany` endpoint).

Recommendation:
- Confirm whether Ethereal supports batched subscriptions.
- If not, keep single emits but make them cheaper (avoid `Value.to_string()` each time).

---

### 2.5 Avoiding Cloning on Subscription Addition — ❌ Not Implemented

Current code pushes `json_msg.clone()` into the Vec:

```rust
subscriptions.push(json_msg.clone());
```

This clone is unnecessary, because `json_msg` is already owned by the function.

**Recommendation:**
```rust
subscriptions.push(json_msg);
```

This is a simple improvement and reduces allocations/copies during subscription setup.

---

### 2.6 String Conversion Optimization — ❌ Not Implemented

Both subscription builders allocate new `String`s on each call:

```rust
msg_type: channel.to_string(),
product_id: product_id.to_string(),
```

If channels and product IDs are repeated, consider:
- using `Arc<str>` for commonly repeated values
- or, at minimum, avoiding repeated conversions where possible

This is not urgent, but helps when subscription counts scale.

---

## 3. Code-Level Findings and Recommendations (Current State)

### 3.1 Design Fragility: `Arc<Vec<...>>` + `Arc::get_mut(...)`

Subscriptions are stored as `Arc<Vec<Value>>`, and updated via:

```rust
let subscriptions = Arc::get_mut(&mut self.subscriptions)
    .expect("Failed to get mutable reference to subscriptions");
```

This works only if the `Arc` is uniquely owned at the time of mutation.
In the current code, `connect()` clones the `Arc` into callbacks, which implies mutation is expected to happen **before** connect.

**Risk:**
- If a user calls subscription methods after connect (even accidentally), the SDK will panic.
- This is a correctness/UX hazard.

**Recommended options:**
1) Store `subscriptions: Vec<SerializedSub>` (not `Arc`) and clone/move it into the connect callback only after subscription registration is complete.
2) Store `subscriptions: Arc<Mutex<Vec<SerializedSub>>>` and allow runtime mutation safely (with explicit locking).
3) Use `Arc::make_mut` instead of `Arc::get_mut` if semantics allow cloning-on-write.

Given your current performance goals, option (1) is the best balance:
- no locks on hot path
- no panic risk
- easy reasoning about lifecycle

---

### 3.2 Logging in Resubscription Loop

In the connect callback, the code logs each subscription:

```rust
info!("Subscribing to channel: {sub:?}");
```

If subscription count is large, this can:
- dominate CPU during reconnect
- increase reconnect time and tail latency

**Recommendation:**
- log once per reconnect with number of subscriptions
- optionally sample per-subscription logs

---

### 3.3 Error Handling Strategy

Several code paths use `expect(...)` inside async callbacks (e.g., emitting subscribe messages).
In a production SDK, panics inside callbacks are hard to debug and can bring down the process.

**Recommendation:**
- replace `expect` in callbacks with error logging + state transition
- return gracefully to allow reconnect logic to continue

---

## 4. What Improved (Independent of Prior Audit)

### 4.1 Bench suite maturity — ✅

The presence of multiple Criterion benchmarks in `benches/` indicates the project now supports a stable performance workflow (connect latency, serialization, parsing, signing). This is consistent with the audit methodology.

### 4.2 Connection state tracking — ✅

`ConnectionState` is a good addition that makes it easier to build dashboards and to reason about reconnection behavior.

---

## 4.3 Additional Findings (Independent Re-Audit)

The following items were identified during re-verification and are not covered in the prior sections.

### 4.3.1 `run_till_event()` can panic

**Location:** `src/ws_client.rs:143`

```rust
pub async fn run_till_event(&mut self) -> ConnectionState {
    self.state_rx.changed().await.unwrap();  // panic if all senders dropped
    *self.state_rx.borrow()
}
```

`watch::Receiver::changed()` returns `Err` when all `Sender`s are dropped. If the caller holds the only copy of `WsClient` and something drops the internal `state_tx` (or the client is used after drop), `.unwrap()` will panic.

**Recommendation:** Prefer `match self.state_rx.changed().await { Ok(()) => ..., Err(_) => return ConnectionState::Disconnected }` (or map to a fallback state) instead of `.unwrap()`.

---

### 4.3.2 No duplicate subscription check

**Location:** `src/ws_client.rs:146–181`

`subscribe_with_product` and `subscribe_with_subaccount` always push a new entry. The same `(channel, product_id)` or `(channel, subaccount_id)` can be added multiple times. On reconnect, the server will receive duplicate subscribe messages, and the client will do extra serialization and network work.

**Recommendation:** Either document that duplicates are allowed, or before `push` check for an existing equal subscription (e.g. by keeping a `HashSet` of serialized payloads or of `(channel, product_id)` / `(channel, subaccount_id)`) and skip if already present.

---

### 4.3.3 Unbounded `tokio::spawn` in message callback

**Location:** `src/utils.rs:54–59`

```rust
for item in items {
    let callback = callback.clone();
    tokio::spawn(async move {
        callback(item).await;
    });
}
```

Each deserialized item from a single WebSocket payload triggers a new `tokio::spawn`. A burst of messages (e.g. one payload with many items) can create many tasks at once. There is no backpressure or limit. This is both a correctness (resource exhaustion) and p99/p999 latency concern.

**Recommendation:** Consider a bounded approach: e.g. spawn a single task that processes the batch, or send items into a bounded channel consumed by a fixed number of workers. At minimum, document that callbacks should be cheap and that high message rates may create many concurrent tasks.

---

### 4.3.4 Registering callbacks after `connect()` panics

**Location:** `src/ws_client.rs:192–195`

```rust
self.client_builder = self
    .client_builder
    .take()
    .expect("client_builder not set")  // panic if connect() was already called
    .on(channel, callback)
    .into();
```

`connect()` takes `client_builder` with `.take()`. After the first `connect()`, `client_builder` is `None`. Any later call to `register_*` (e.g. `register_market_data_callback`) hits `.expect("client_builder not set")` and panics.

**Recommendation:** Document that all `register_*` and `subscribe_*` calls must be made **before** `connect()`. Alternatively, change the design so that callbacks/subscriptions can be added after connect (e.g. keep builder or client in a form that accepts new handlers without panicking).

---

## 5. Action Plan (Minimal Patch Set)

### P0 (safety / correctness)
1) Remove `.expect(...)` panics inside connect callbacks (log and continue)
2) Remove `Arc::get_mut(...).expect(...)` lifecycle panic risk:
   - either enforce “subscribe-before-connect” via API design, or
   - switch to safer storage design
3) **run_till_event:** replace `state_rx.changed().await.unwrap()` with handling of `Err` (e.g. return `ConnectionState::Disconnected`)
4) **API contract:** document that `register_*` and `subscribe_*` must be called before `connect()`, or allow registration after connect without panic. (P0 only if post-connect registration is expected to be supported; if the SDK explicitly uses a builder-style API “all register before connect”, documenting and enforcing that contract is P1.)

### P1 (performance)
5) Store subscription payloads as pre-serialized `String` (or `Bytes`), not `Value`
6) Remove `json_msg.clone()` on push

### P2 (polish)
7) Reduce per-subscription info logs during reconnect
8) Consider string interning / `Arc<str>` for repeated IDs if scaling to many subscriptions
9) **Duplicates:** document or prevent duplicate subscriptions (same channel + id)
10) **Callbacks:** document or bound `tokio::spawn` usage in `get_typed_callback` (burst of items → many tasks)

---

## 6. Closing Assessment

The SDK is **meaningfully closer to production-grade** than at the time of the last audit.
The most important remaining gap is that “cached subscriptions” currently still incur re-serialization (`Value.to_string()`) during reconnect, and the `Arc::get_mut` mutation pattern introduces a lifecycle panic risk.

**Re-audit additions:** Section 0.1 confirms all findings against the current code; Section 4.3 adds four items: panic risk in `run_till_event()`, no duplicate-subscription check, unbounded `tokio::spawn` in message callbacks, and panic when registering callbacks after `connect()`.

Addressing the minimal patch set above would:
- further stabilize p99/p999 during reconnect storms,
- reduce CPU spikes at resubscription time,
- improve UX correctness (avoid panics on late subscription calls and in `run_till_event`),
- preserve the current excellent baseline performance.

