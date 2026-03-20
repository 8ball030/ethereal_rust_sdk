# Ethereal Rust Sdk
This is the Ethereal Rust SDK, which provides tools and libraries for interacting with the [Ethereal](https://ethereal.trade) platform using the Rust programming language.

## Features
- Yawc for real-time, zero-copy communication for all supported WebSocket channels
- JSON serialization and deserialization
- Async HTTP requests with Reqwest
- Fully typed data models generated from the Ethereal OpenAPI specification.
- Support for all Ethereal REST API endpoints
- Support for all Ethereal WebSocket API channels using async callbacks
- Comprehensive error handling
- Example code for common use cases

## Getting Started

At present, the Ethereal Rust SDK is under active development. To get started with the SDK, clone the repository and run the example code;`

We have a number of examples included in the `examples` directory. Here is how to run the `simple_order_submission` example:

```bash
git clone https://github.com/8ball030/ethereal_rust_sdk.git
cd ethereal_rust_sdk
cargo run --example simple_order_submission
```

## Installation
To use the Ethereal Rust SDK in your project run the following command to add it as a dependency in your `Cargo.toml` file:

```bash
cargo add ethereal_rust_sdk
```

## Usage of the SDK

There are two main clients provided by the SDK: an asynchronous HTTP client for interacting with the Ethereal REST API, and a WebSocket client for real-time data via the Ethereal WebSocket API.

### Creating Clients
A convenient utility function is provided to create both clients. Here is an example of how to use it:

```rust
    let env = Environment::Testnet;
    let private_key = "your_private_key_here";
    let (http_client, ws_client) = create_client(env, private_key, None).await?;
```

## HTTP Client
All of the HTTP client functionality is encapsulated in the `HttpClient` struct. This client can be used to make requests to various endpoints of the Ethereal REST API.

The client has been generated using the OpenAPI specification provided by Ethereal, ensuring that all endpoints and data models are up-to-date with the latest API version.

### Submitting Orders

```rust
// examples/simple_order_submission.rs
mod common;
use ethereal_rust_sdk::models::{OrderSide, OrderType, TimeInForce};
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, _) = common::create_test_clients().await?;

    println!("Creating order...");

    let ticker = "BTC-USD";
    let quantity = dec!(0.001);
    let price = dec!(80000.0);
    let side = OrderSide::BUY;
    let r#type = OrderType::Limit;

    let expires_at = None;
    let time_in_force = TimeInForce::Gtd;

    // We have a few more options when creating an order now.
    let mut post_only = false;
    let mut reduce_only = false;

    let order = client
        .submit_order(
            ticker,
            quantity,
            price,
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();
    println!("Order submitted: {order:?}");

    // We can also create orders with post only flag
    println!("Creating post only reduce only order...");
    post_only = true;
    reduce_only = false;
    let order = client
        .submit_order(
            ticker,
            quantity,
            price,
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();
    println!("Post and reduce only order submitted: {order:?}");

    println!("Creating order with expires_at...");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    let expires_at = Some(now + 60); // Expires in 60 seconds
    let order = client
        .submit_order(
            ticker,
            quantity,
            price,
            side,
            r#type,
            time_in_force,
            post_only,
            reduce_only,
            expires_at,
        )
        .await
        .unwrap();
    println!("Order with expires_at submitted: {order:?}");

    println!("Fetching all current orders to cancel...");
    let orders = client.get_open_orders().await?;
    let cancel_result = client
        .cancel_orders(orders.iter().map(|order| order.id.to_string()).collect())
        .await?;
    println!("Cancel result: {cancel_result:?}");
    Ok(())
}

```

### Fetching Positions

Positions can be fetched similarly:

```rust
// examples/fetch_positions.rs
use ethereal_rust_sdk::apis::{
    position_api::PositionControllerListBySubaccountIdParams,
    subaccount_api::SubaccountControllerListByAccountParams,
};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, _) = common::create_test_clients().await?;
    let params = SubaccountControllerListByAccountParams {
        sender: http_client.address.clone(),
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params).await?;

    let positions = http_client
        .position()
        .list_by_subaccount_id(PositionControllerListBySubaccountIdParams {
            subaccount_id: subaccounts.data.first().unwrap().id.to_string(),
            ..Default::default()
        })
        .await?;
    println!("Positions: {positions:#?}");

    Ok(())
}

```

### WebSocket Client

The Websocket client can be used as somewhat illustrated in the example below:

# Channels

In order to proces messages from the websocket client, the user must first register a callback, then subscribe to the desired channel.


## Market Data Subscription
```rust
// examples/market_data.rs
mod common;

use ethereal_rust_sdk::models::TickerMessage;
use log::info;

async fn ticker_callback(msg: TickerMessage) {
    info!("Received ticker message: {:?}", msg);
}

#[tokio::main]
async fn main() {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Info).expect("log");
    let (http_client, ws_client) = common::create_test_clients().await.unwrap();
    info!("HTTP client and WS client created");
    let tickers = common::get_product_tickers(&http_client).await.unwrap();

    ws_client
        .subscriptions()
        .ticker(tickers, ticker_callback)
        .await
        .unwrap();

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
}

```
As can be seen, the SDK provides both asynchronous HTTP clients and WebSocket clients to interact with the Ethereal platform.

### Order Status Updates

The following example demonstrates how to register for order updates.

```rust
// examples/order_update.rs
mod common;
use ethereal_rust_sdk::models::OrderUpdateMessage;

use log::info;

async fn order_update_callback(raw_data: OrderUpdateMessage) {
    let data = raw_data.data;
    let orders = data.d;

    for order in orders {
        info!(
            "Order update @ ID: {}, Symbol: {}, Price: {:?}, Side: {:?} Quantity: {:?} Status: {:?}",
            order.id, order.s, order.px, order.sd, order.qty, order.st
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, ws_client) = common::create_test_clients().await?;
    let subaccount_ids = common::get_subaccount_ids(&http_client).await?;

    ws_client
        .subscriptions()
        .order_update(subaccount_ids, order_update_callback)
        .await?;

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
    Ok(())
}

```

## Order Fills

An order fill callback can be registered and subscribed to in a similar manner, this time using the `register_order_fill_callback` and `subscribe_order_fill` methods.

Additionally, it should be pointed out that a different data model is used for order fills, namely `PageOfOrderFillDtos`.

```rust
// examples/order_fills.rs
mod common;
use ethereal_rust_sdk::models::OrderFillMessage;

use log::info;

async fn order_fill_callback(raw_data: OrderFillMessage) {
    let fill = raw_data.data;
    let orders = fill.d;

    for fill in orders {
        info!(
            "Order fill @ ID: {}, Symbol: {}, Price: {}, Side: {:?} Quantity: {:?}",
            fill.id, fill.s, fill.px, fill.sd, fill.sz
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let (http_client, ws_client) = common::create_test_clients().await?;
    let subaccount_ids = common::get_subaccount_ids(&http_client).await?;

    ws_client
        .subscriptions()
        .order_fill(subaccount_ids, order_fill_callback)
        .await?;

    info!("Starting event loop...");
    common::run_forever(&ws_client).await;
    Ok(())
}

```

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes.
Before submitting a pull request, please ensure that your code adheres to the project's coding standards and passes all tests.
Please run the following commands to lint, format, build, and test the project before submitting your changes:

```sh
make codegen
make fmt
make lint
make build
make test
make docs
# Or simpler
make all
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## TODO
- [x] Generate datatypes from Ethereal API spec.
- [x] Set up continuous integration and deployment (CI/CD) pipeline.
- [x] Fully Integrate with Ethereal Websocket API.
- [x] Implement Read HTTP client for Ethereal REST API.
- [x] Create example code for using the SDK.
- [x] Implement Order posting for Ethereal REST API.
- [x] Create async HTTP client for Ethereal REST API.
- [x] Write tests for all modules and functionalities.
- [x] Add more examples and documentation.
- [x] Publish the crate to crates.io.
- [x] Template all other signable apis.
- [x] Create abstraction for signable requests.
- [x] Parse Websocket stringified numbers into appropriate numeric types.
- [ ] Parse Api stringified numbers into appropriate numeric types.
- [ ] Add Market Order support for order submission.
- [ ] Add Stop Order support for order submission.

## Acknowledgements
- [Ethereal](https://ethereal.trade) for providing the platform and API.
- [Reqwest](https://docs.rs/reqwest/) for HTTP requests in Rust.
- [YAWC](https://docs.rs/yawc/) for WebSocket communication in Rust.
- [Serde](https://serde.rs/) for serialization and deserialization in Rust.
- [Serde JSON](https://docs.rs/serde_json/) for JSON support in Rust.
