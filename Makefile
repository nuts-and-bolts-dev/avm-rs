.PHONY: all build test check fmt clippy clean doc audit dev-deps ci examples help

# Run formatting, linting, tests, and build examples
all: fmt clippy test examples

# Build the project with all possible features
build:
	cargo build --all-features
	cargo build --no-default-features

# Run all tests
test:
	cargo test --all-features
	cargo test --no-default-features

# Check code compilation
check:
	cargo check --all-features
	cargo check --no-default-features

# Format code
fmt:
	cargo fmt --all
	taplo format

# Run clippy linter
clippy:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --all-targets --no-default-features -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Generate and open documentation
doc:
	cargo doc --no-deps --all-features --open

# Run security audit
audit:
	cargo audit

# Install development dependencies
dev-deps:
	rustup component add rustfmt clippy
	cargo install cargo-audit
	argo install taplo-cli --features=lsp

# Build all examples
examples:
	cargo build --examples

# Run all CI checks locally
ci: fmt clippy test build examples

# Show this help message
help:
	@echo ''
	@echo 'Usage:'
	@echo '  make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "\033[36m%-15s\033[0m %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
