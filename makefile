TOML_FILE := Cargo.toml

# Extract version from TOML
VERSION := $(shell sed -n 's/^version *= *"\(.*\)"/\1/p' $(TOML_FILE))

# Default: bump patch
PATCH_VERSION := $(shell \
	echo $(VERSION) | awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}' \
)

# Allow override
NEW_VERSION ?= $(PATCH_VERSION)

.PHONY: version tag release


version:
	@echo "Current version: $(VERSION)"
	# Update version in Cargo.toml
	@sed -i.bak 's/^version *= *".*"/version = "$(NEW_VERSION)"/' $(TOML_FILE)
	@rm -f $(TOML_FILE).bak
	@echo "Release version: $(NEW_VERSION)"
	cargo check
tag:
	@git tag -a v$(NEW_VERSION) -m "Release v$(NEW_VERSION)"
	@git push origin v$(NEW_VERSION)

package:
	@echo packaging crate
	git add $(TOML_FILE) Cargo.lock
	@git commit -m "Bump version to v$(NEW_VERSION)"
	@git push
	echo added git
	@cargo package

release: version package tag
	@echo "Creating GitHub release v$(NEW_VERSION)"
	@gh release create v$(NEW_VERSION) \
		--title "v$(NEW_VERSION)" \
		--notes "Release v$(NEW_VERSION)"
	@echo "Creating crate release v$(NEW_VERSION)"
	@cargo publish

lint: 
	cargo clippy --all-features --all-targets --examples --tests -- -D warnings 
fmt:
	cargo fmt --all
	cargo clippy --all-features --all-targets --examples --tests --fix --allow-dirty -- -D warnings
build:
	cargo build --all-features
test:
	cargo test --all-features
run:
	cargo run --all-features

codegen:
	bash build_scripts/pre_processing.sh

	openapi-generator-cli generate \
	  -i openapi.json \
	  -g rust \
	  -o ./generated \
	--additional-properties=supportAsync=true,useSingleRequestParameter=true


	cp ./generated/src/models/* ./src/models/
	cp ./generated/docs/* ./docs/generated/
	cp -r ./generated/src/apis ./src/

	# rebuild mod.rs
	@echo "#![allow(clippy::all)]" > ./src/models/mod.rs
	@echo "#![allow(unused_imports)]" >> ./src/models/mod.rs
	@echo "#![allow(dead_code)]" >> ./src/models/mod.rs
	@echo "#![allow(non_camel_case_types)]" >> ./src/models/mod.rs
	@echo "#![allow(clippy::upper_case_acronyms)]" >> ./src/models/mod.rs
	# rebuild api mod.rs

	@echo "#![allow(clippy::all)]" > ./src/apis/mod.rs
	cat ./generated/src/apis/mod.rs >> ./src/apis/mod.rs

	# cleanup
	rm -rf ./generated

	@for f in ./src/models/*.rs; do \
		base=$$(basename $$f); \
		if [ "$$base" = "mod.rs" ]; then continue; fi; \
		name=$${base%.rs}; \
		camel=$$(echo $$name | sed -E 's/(^|_)([a-z])/\U\2/g'); \
		echo "pub mod $$name;" >> ./src/models/mod.rs; \
		echo "pub use $$name::$$camel;" >> ./src/models/mod.rs; \
	done
	python build_scripts/post_processing.py

all: codegen fmt lint build test
