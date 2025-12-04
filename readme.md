# Ethereal Rust Sdk
This is the Ethereal Rust SDK, which provides tools and libraries for interacting with the [Ethereal](https://ethereal.trade) platform using the Rust programming language.

## Features
- Socket.IO client for real-time communication for all supported WebSocket channels
- JSON serialization and deserialization
- HTTP requests with Reqwest

## Getting Started

At present, the Ethereal Rust SDK is under active development. To get started with the SDK, clone the repository and run the example code;`

We have a number of examples included in the `examples` directory. Here is how to run the `market_data` example:

```bash
git clone https://github.com/8ball030/ethereal_rust_sdk.git
cd ethereal_rust_sdk
cargo run --example market_data
```


The client can be used as somewhat illustrated in the example below:

```rust

use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::MarketPriceDto;
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::ws_client::WsClient;

fn market_data_callback(market_price: Payload, _socket: RawClient) {
    if let Payload::Text(values) = market_price {
        for value in values {
            if let Ok(market_price) = serde_json::from_value::<MarketPriceDto>(value) {
                info!(
                    "Market Price Update - Product ID: {:?}, Best Bid: {:?}, Best Ask: {:?}",
                    market_price.product_id,
                    market_price.best_bid_price,
                    market_price.best_ask_price
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let env = Environment::Production;

    let http_client = HttpClient::new(env.clone());
    let params = ProductControllerListParams::default();
    let products = http_client.product().list(params)?;

    let mut ws_client = WsClient::new(env.clone());
    ws_client.register_market_price_callback(market_data_callback);
    ws_client.connect()?;

    products.data.iter().for_each(|product| {
        ws_client.subscribe_market_data(&product.id.to_string());
    });
    ws_client.run_forever();
    Ok(())
}

```

As can be seen, the SDK provides both synchronous HTTP clients and WebSocket clients to interact with the Ethereal platform.

# Channels

In order to proces messages from the websocket client, the user must first register a callback, then subscribe to the desired channel.

The following example demonstrates how to do so for order updates.

### Order Status Updates

```rust
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::PageOfOrderDtos;
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::ws_client::WsClient;

use log::{error, info};
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn order_update_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            match serde_json::from_value::<PageOfOrderDtos>(value.clone()) {
                Ok(page) => {
                    for fill in page.data {
                        info!(
                            "Order update - ID: {}, Product ID: {}, Price: {}, Side: {:?} Quantity: {:?}",
                            fill.id, fill.product_id, fill.price, fill.side, fill.filled
                        );
                    }
                }
                Err(err) => {
                    error!("Failed to deserialize order data: {value}, error: {err}");
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let sender_address = std::env::var("SENDER_ADDRESS").unwrap_or_else(|_| {
        panic!("SENDER_ADDRESS environment variable is not set");
    });
    let env = Environment::Testnet;

    let http_client = HttpClient::new(env.clone());
    let params = SubaccountControllerListByAccountParams {
        sender: sender_address,
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;

    let mut ws_client = WsClient::new(env);
    ws_client.register_order_update_callback(order_update_callback);
    ws_client.connect()?;
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_order_update(&subaccount.id.to_string());
    });
    ws_client.run_forever();

    Ok(())
}
```

The example can be run with the following command:

```bash
╰─>$ cargo run --example order_updates 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/examples/order_updates`
2025-12-04T15:19:24.659Z INFO  [ethereal_rust_sdk::ws_client] Callback registered channel=OrderUpdate
2025-12-04T15:19:24.659Z INFO  [ethereal_rust_sdk::ws_client] Connecting websocket...
2025-12-04T15:19:26.294Z INFO  [ethereal_rust_sdk::ws_client] Websocket connected
2025-12-04T15:19:26.295Z INFO  [ethereal_rust_sdk::ws_client] Subscribed to channel=OrderUpdate subaccount_id=11111111-2222-3333-4444-444444444444
2025-12-04T15:19:41.089Z INFO  [order_updates] Order update - ID: 11111111-2222-3333-4444-555555555555, Product ID: dce327cc-4fbb-4d5d-9ede-1c1fca7ef4ba, Price: 92324, Side: SELL Quantity: "0.001"

```

## Order Fills

An order fill callback can be registered and subscribed to in a similar manner, this time using the `register_order_fill_callback` and `subscribe_order_fill` methods.

NOTE: The example below assumes you have already set the `SENDER_ADDRESS` environment variable.

Additionally, it should be pointed out that a different data model is used for order fills, namely `PageOfOrderFillDtos`.

```rust
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::PageOfOrderFillDtos;
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethereal_rust_sdk::ws_client::WsClient;

use log::{error, info};
use rust_socketio::client::RawClient;
use rust_socketio::Payload;

fn order_fill_callback(raw_data: Payload, _socket: RawClient) {
    if let Payload::Text(values) = raw_data {
        for value in values {
            match serde_json::from_value::<PageOfOrderFillDtos>(value.clone()) {
                Ok(page) => {
                    for fill in page.data {
                        info!(
                            "Order fill - ID: {}, Product ID: {}, Price: {}, Side: {:?} Quantity: {:?}",
                            fill.id, fill.product_id, fill.price, fill.side, fill.filled
                        );
                    }
                }
                Err(err) => {
                    error!("Failed to deserialize order data: {value}, error: {err}");
                }
            }
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let sender_address = std::env::var("SENDER_ADDRESS").unwrap_or_else(|_| {
        panic!("SENDER_ADDRESS environment variable is not set");
    });

    let env = Environment::Testnet;
    let http_client = HttpClient::new(env.clone());
    let params = SubaccountControllerListByAccountParams {
        sender: sender_address,
        ..Default::default()
    };
    let subaccounts = http_client.subaccount().list_by_account(params)?;

    let mut ws_client = WsClient::new(env);

    ws_client.register_order_fill_callback(order_fill_callback);
    ws_client.connect()?;
    subaccounts.data.iter().for_each(|subaccount| {
        ws_client.subscribe_order_fill(&subaccount.id.to_string());
    });
    ws_client.run_forever();

    Ok(())
}

```

```bash
╰─>$ cargo run --example order_fills
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/examples/order_fills`
2025-12-04T15:22:11.425Z INFO  [ethereal_rust_sdk::ws_client] Callback registered channel=OrderFill
2025-12-04T15:22:11.425Z INFO  [ethereal_rust_sdk::ws_client] Connecting websocket...
2025-12-04T15:22:13.104Z INFO  [ethereal_rust_sdk::ws_client] Websocket connected
2025-12-04T15:22:13.142Z INFO  [ethereal_rust_sdk::ws_client] Subscribed to channel=OrderFill subaccount_id=11111111-2222-3333-4444-444444444444
2025-12-04T15:22:26.745Z INFO  [order_fills] Order fill - ID: 11111111-2222-3333-4444-555555555555, Product ID: dce327cc-4fbb-4d5d-9ede-1c1fca7ef4ba, Price: 92724, Side: SELL Quantity: "0.001"
```




:NOTE: Instructions for getting started with the Ethereal Rust SDK will be provided here soon.

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes.
Before submitting a pull request, please ensure that your code adheres to the project's coding standards and passes all tests.
Please run the following commands to lint, format, build, and test the project before submitting your changes:

```sh
make fmt
make lint
make build
make test

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
- [ ] Implement Order posting for Ethereal REST API.
- [ ] Create async HTTP client for Ethereal REST API.
- [ ] Write tests for all modules and functionalities.
- [ ] Add more examples and documentation.
- [ ] Publish the crate to crates.io.
- [ ] Parse stringified numbers into appropriate numeric types.

## Acknowledgements
- [Ethereal](https://ethereal.trade) for providing the platform and API.
- [Reqwest](https://docs.rs/reqwest/) for HTTP requests in Rust.
- [Rust Socket.IO](https://docs.rs/rust_socketio/) for Socket.IO client functionality in Rust.
- [Serde](https://serde.rs/) for serialization and deserialization in Rust.
- [Serde JSON](https://docs.rs/serde_json/) for JSON support in Rust.
