.PHONY: all build test check fmt fmt-toml fmt-toml-fix clippy clean doc audit dev-deps ci examples run-examples help

# Default target
all: fmt fmt-toml clippy test examples

# Show available targets
help:
	@echo "Available targets:"
	@echo "  all           - Run formatting, linting, tests, and build examples"
	@echo "  build         - Build the project with all features"
	@echo "  test          - Run all tests"
	@echo "  check         - Check code compilation"
	@echo "  fmt           - Format Rust code"
	@echo "  fmt-toml      - Format TOML files"
	@echo "  clippy        - Run clippy linter"
	@echo "  examples      - Build all examples"
	@echo "  run-examples  - Run all examples (demonstrative)"
	@echo "  clean         - Clean build artifacts"
	@echo "  doc           - Generate and open documentation"
	@echo "  audit         - Run security audit"
	@echo "  dev-deps      - Install development dependencies"
	@echo "  ci            - Run all CI checks locally"
	@echo "  help          - Show this help message"

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

# Build all examples
examples:
	@echo "Building all examples..."
	cargo build --examples

# Run all examples
run-examples:
	@echo "Running all examples..."
	@echo "=== Basic Arithmetic ==="
	cargo run --example basic_arithmetic
	@echo ""
	@echo "=== Simple Test ==="
	cargo run --example simple_test
	@echo ""
	@echo "=== Control Flow ==="
	cargo run --example control_flow
	@echo ""
	@echo "=== Crypto Operations ==="
	cargo run --example crypto_operations
	@echo ""
	@echo "=== Smart Contract ==="
	cargo run --example smart_contract
	@echo ""
	@echo "=== TEAL Assembly ==="
	cargo run --example teal_assembly
	@echo ""
	@echo "=== Transaction Fields ==="
	cargo run --example transaction_fields
	@echo "All examples completed!"

# Run all CI checks locally
ci: fmt fmt-toml clippy test build examples
	@echo "All CI checks passed!"
