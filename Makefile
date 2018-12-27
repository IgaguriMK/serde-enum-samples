CRATE_NAME:=serde-enum-samples

.PHONY: all
all: soft-clean build check

.PHONY: build
build:
	cargo build

.PHONY: check
check:
	cargo clippy -- -D warnings
	cargo test

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean