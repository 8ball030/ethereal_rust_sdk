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

codegen:
	# curl https://api.ethereal.trade/openapi.json | jq > openapi.json
	openapi-generator-cli generate \
	  -i openapi.json \
	  -g rust \
	  -o ./generated
	  -c ./openapi_config.yaml
	cp ./generated/src/models/* ./src/models/
	cp ./generated/docs/* ./docs/generated/
	rm -rf ./generated
	# we now need to make the models/mod.rs file manually
	echo "#![allow(clippy::all)]" > ./src/models/mod.rs
	echo "#![allow(unused_imports)]" >> ./src/models/mod.rs
	echo "#![allow(dead_code)]" >> ./src/models/mod.rs
	echo "pub mod product_dto;" >> ./src/models/mod.rs
	echo "pub mod page_of_product_dtos;" >> ./src/models/mod.rs
	echo "pub use product_dto::ProductDto;" >> ./src/models/mod.rs
	echo "pub use page_of_product_dtos::PageOfProductDtos;" >> ./src/models/mod.rs
	# engine type
	echo "pub mod engine_type;" >> ./src/models/mod.rs
	echo "pub use engine_type::EngineType;" >> ./src/models/mod.rs



all: codegen fmt lint build test
