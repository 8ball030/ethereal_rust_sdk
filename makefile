lint: 
	cargo clippy --all-features --all-targets -- -D warnings
fmt:
	cargo fmt --all
build:
	cargo build --all-features
test:
	cargo test --all-features
run:
	cargo run --all-features

all: fmt lint build test