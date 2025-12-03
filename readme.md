# Ethereal Rust Sdk
This is the Ethereal Rust SDK, which provides tools and libraries for interacting with the [Ethereal](https://ethereal.trade) platform using the Rust programming language.

## Features
- Socket.IO client for real-time communication
- JSON serialization and deserialization
- Asynchronous programming with Tokio
- HTTP requests with Reqwest

## Getting Started

At present, the Ethereal Rust SDK is under active development. To get started with the SDK, clone the repository and run the example code;`

We have a number of examples included in the `examples` directory. Here is how to run the `market_data` example:

```bash
git clone https://github.com/8ball030/ethereal_rust_sdk.git
cd ethereal_rust_sdk
cargo run --example market_data
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
- [ ] Integrate with Ethereal authentication system.
- [ ] Fully Integrate with Ethereal Websocket API.
- [ ] Add more examples and documentation.
- [ ] Write tests for all modules and functionalities.
- [ ] Publish the crate to crates.io.
- [ ] Parse stringified numbers into appropriate numeric types.

## Acknowledgements
- [Ethereal](https://ethereal.trade) for providing the platform and API.
- [Tokio](https://tokio.rs/) for asynchronous programming in Rust.
- [Reqwest](https://docs.rs/reqwest/) for HTTP requests in Rust.
- [Rust Socket.IO](https://docs.rs/rust_socketio/) for Socket.IO client functionality in Rust.
- [Serde](https://serde.rs/) for serialization and deserialization in Rust.
- [Anyhow](https://docs.rs/anyhow/) for error handling in Rust.
- [Serde JSON](https://docs.rs/serde_json/) for JSON support in Rust.
