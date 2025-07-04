.PHONY: all build test check fmt fmt-toml fmt-toml-fix clippy clean doc audit dev-deps ci

all: fmt fmt-toml clippy test

build:
	cargo build --all-features
	cargo build --no-default-features

test:
	cargo test --all-features
	cargo test --no-default-features

check:
	cargo check --all-features
	cargo check --no-default-features

fmt:
	cargo fmt --all

# Check TOML formatting
fmt-toml:
	taplo format

clippy:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --all-targets --no-default-features -- -D warnings

clean:
	cargo clean

doc:
	cargo doc --no-deps --all-features --open

audit:
	cargo audit

# Install development dependencies
dev-deps:
	rustup component add rustfmt clippy
	cargo install cargo-audit
	cargo install taplo-cli

# Run all CI checks locally
ci: fmt fmt-toml clippy test build
	@echo "All CI checks passed!"
