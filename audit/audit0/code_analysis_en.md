# Ethereal Rust SDK Code Audit

## Project Overview

Ethereal Rust SDK is a client library for working with Ethereal exchange REST API and WebSocket API. The project provides a synchronous HTTP client for REST requests and a WebSocket client for subscribing to real-time data channels.

## Architecture and Components

### 1. Main Modules

#### `src/lib.rs`
Main library module, exports:
- `apis` - OpenAPI-generated clients for REST API
- `models` - data models generated from OpenAPI specification
- `ws_client` - WebSocket client for subscribing to data streams
- `sync_client` - synchronous HTTP client for REST API
- `channels` - module with channel constants for subscriptions
- `signing` - module for EIP-712 message signing
- `types` - subscription data types
- `utils` - utilities

#### `src/ws_client.rs`
Main module containing WebSocket client implementation.

**Key Components:**

- **`WsClient`** - public client API
  - `new(environment)` - create client for specified environment (Mainnet/Testnet)
  - `connect()` - connect to WebSocket server
  - `run_forever()` - run infinite loop to maintain connection
  - Subscription methods: `subscribe_market_data()`, `subscribe_orderbook_data()`, `subscribe_trade_fill_data()`, etc.
  - Callback registration methods: `register_market_price_callback()`, `register_orderbook_callback()`, etc.

- **Internal methods:**
  - `subscribe_with_product()` - subscribe to channel with product_id
  - `subscribe_with_subaccount()` - subscribe to channel with subaccount_id
  - `register_callback_internal()` - register callback for channel message handling

- **Connection handling:**
  - Uses `rust_socketio` library for WebSocket connection
  - Automatically resubscribes to all active subscriptions on connection
  - Uses `Arc<AtomicBool>` for connection state synchronization
  - Blocking connection wait via `std::thread::sleep`

#### `src/sync_client/`
Synchronous HTTP client module for REST API.

**Key Components:**

- **`HttpClient`** - main HTTP client
  - `new(environment, private_key)` - create client with authentication
  - Methods to access various APIs: `product()`, `order()`, `position()`, `funding()`, etc.
  - Uses `HashMap` for product caching (`product_hashmap`)

- **Specialized clients:**
  - `ProductClient` - work with products
  - `OrderClient` - order management
  - `PositionClient` - work with positions
  - `SubaccountClient` - subaccount management
  - Other clients for various APIs

#### `src/models/`
Data models generated from OpenAPI specification:
- Models for all API data types
- Request and response structures
- Enums for various data types

#### `src/channels.rs`
Module with channel constants for WebSocket subscriptions:
- `BOOK_DEPTH` - order book depth
- `MARKET_PRICE` - market price
- `ORDER_FILL` - order fill
- `TRADE_FILL` - trade fill
- `ORDER_UPDATE` - order update
- `SUBACCOUNT_LIQUIDATION` - subaccount liquidation
- `TOKEN_TRANSFER` - token transfer

#### `src/types.rs`
Subscription data types:
- `ProductSubscriptionMessage` - subscription message with product_id
- `SubaccountSubscriptionMessage` - subscription message with subaccount_id

### 2. Data Flows

#### REST API Requests (HTTP)
1. Client created via `HttpClient::new(environment, private_key)`
2. Requests executed through specialized clients: `http_client.product().list(params)`
3. Requests signed using EIP-712 signature
4. Responses deserialized into corresponding models

#### WebSocket Subscriptions (pub-sub)
1. Client created via `WsClient::new(environment)`
2. Callbacks registered via `register_*_callback()` methods
3. Subscriptions executed via `subscribe_*()` methods
4. Subscriptions stored in `Vec<Value>` (`subscriptions`) as JSON values
5. On connection, all subscriptions automatically restored via `emit("subscribe", Payload::from(sub.to_string()))`
6. **Important:** SDK does not parse incoming JSON messages. User callbacks receive `Payload` from `rust_socketio` library, and JSON parsing (if needed) is performed by users in their callbacks
7. Message routing by channels is handled by `rust_socketio` library based on registered callbacks

### 3. Connection Management

**WebSocket Client:**
- Uses `rust_socketio` library for WebSocket connection
- On connection (`connect()`), connection is established and callback registered for "open" event
- In "open" callback, resubscription to all active channels is performed
- Uses blocking connection wait via loop with `std::thread::sleep(Duration::from_millis(100))`
- `run_forever()` method maintains connection active

**Message Handling:**
- Messages handled through callbacks registered via `register_callback_internal()`
- Callbacks receive `Payload` (from `rust_socketio`) and `RawClient` as parameters
- **SDK does not perform JSON parsing** - this is user's responsibility in their callbacks
- In examples, users themselves deserialize `Payload::Text` into corresponding models if needed

## Technologies Used

- **rust_socketio** - WebSocket client
- **reqwest** - HTTP client for REST API
- **serde/serde_json** - JSON serialization/deserialization
- **ethers** - library for Ethereum/EIP-712 signatures
- **log/simple_logger** - logging
- **uuid** - UUID generation
- **url** - URL handling

## Design Patterns

1. **Builder Pattern** - used in `ClientBuilder` for WebSocket client configuration
2. **Callback Pattern** - callback registration for message handling
3. **Pub-Sub Pattern** - subscriptions to data channels via WebSocket
4. **Client Pattern** - specialized clients for various APIs

## Usage Examples

See examples in `examples/` folder:
- `market_data.rs` - market data subscription
- `orderbook_data.rs` - order book data subscription
- `order_fills.rs` - order fill subscription
- `order_updates.rs` - order update subscription
- `simple_order_submission.rs` - simple order submission

Typical workflow for WebSocket:
1. Create client: `WsClient::new(Environment::Testnet)`
2. Register callbacks: `ws_client.register_market_price_callback(callback)`
3. Subscribe to channels: `ws_client.subscribe_market_data(&product_id)`
4. Connect: `ws_client.connect()?`
5. Maintain connection: `ws_client.run_forever()`

Typical workflow for REST API:
1. Create client: `HttpClient::new(Environment::Testnet, private_key)`
2. Execute requests: `http_client.product().list(params)`
3. Work with results: process data from responses

## Implementation Details

1. **Synchronous HTTP Client** - uses blocking calls for REST API
2. **Blocking WebSocket Connection** - `connect()` uses busy-wait with `thread::sleep(100ms)` to wait for connection
3. **Subscription Cloning** - on connection, entire `Vec<Value>` is cloned for passing into `on("open")` callback
4. **JSON Value Cloning** - on subscription addition, `Value` is cloned (`json_msg.clone()`)
5. **String Conversion on Resubscription** - on each reconnection, `sub.to_string()` is executed for each `Value` in resubscription loop
6. **No Reconnection Management** - no automatic reconnection on connection loss
7. **SDK Does Not Parse Incoming JSON** - parsing is performed by users in callbacks, SDK passes `Payload` as-is
8. **Socket.IO Usage** - SDK uses `rust_socketio` library, which provides Socket.IO protocol over WebSocket
