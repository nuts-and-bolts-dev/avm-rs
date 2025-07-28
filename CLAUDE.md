# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of the Algorand Virtual Machine (AVM) that executes TEAL (Transaction Execution Approval Language) bytecode for smart contract logic and transaction validation. The project includes a CLI, assembler, and complete VM implementation.

## Development Commands

### Build & Test
```bash
# Standard build
cargo build

# Build with all features
cargo build --all-features

# Run all tests (with and without features)
cargo test --all-features
cargo test --no-default-features

# Run a specific test
cargo test test_name

# Build examples
cargo build --examples
```

### Code Quality
```bash
# Format code (includes TOML formatting with taplo)
make fmt
cargo fmt --all
taplo format

# Run linter (strict - no warnings allowed)
make clippy
cargo clippy --all-targets --all-features -- -D warnings

# All-in-one quality check
make all  # Runs fmt, clippy, test, examples

# CI simulation
make ci   # Complete CI pipeline locally
```

### CLI Usage
```bash
# Execute TEAL programs
cargo run -- execute program.teal
cargo run -- execute --step program.teal  # Step-by-step debugging

# Assemble TEAL to bytecode
cargo run -- assemble program.teal -o output.bytecode

# Validate TEAL programs
cargo run -- validate program.teal
```

## Architecture Overview

### Core Components

- **Virtual Machine (`src/vm/`)**: Main execution engine that processes TEAL bytecode
  - `VirtualMachine`: Central VM with opcode registry and execution logic
  - `EvalContext`: Runtime state including stack, program counter, and scratch space
  - `ExecutionConfig`: Configuration for run mode, cost budget, and features

- **Opcodes (`src/opcodes/`)**: Complete TEAL instruction set implementation
  - Modular design with separate files for each opcode category
  - `OpSpec` trait for opcode specification and execution
  - Standard opcodes registry via `get_standard_opcodes()`

- **Assembler (`src/assembler/`)**: TEAL source to bytecode compiler
  - Converts human-readable TEAL programs to executable bytecode
  - Supports labels, constants, and full TEAL syntax

- **State Management (`src/state/`)**: Ledger state abstraction
  - `LedgerAccess` trait for blockchain state queries
  - `MockLedger` for testing and development

- **CLI (`src/cli/`)**: Command-line interface
  - Execute, assemble, validate, and debug TEAL programs
  - Interactive REPL and step-by-step execution modes

### Error Handling

Uses `thiserror` for structured error types in `src/error.rs`:
- All operations return `AvmResult<T>` (alias for `Result<T, AvmError>`)
- Comprehensive error variants for all failure modes
- Contextual error information with specific details

### Features

- `tracing`: Optional execution tracing and debugging support
- Default features are minimal for lightweight usage

## Development Guidelines

### Code Standards
- Rust 1.88.0 toolchain (specified in `rust-toolchain.toml`)
- Zero warnings policy enforced by clippy
- Comprehensive error handling - never use `.unwrap()` or `.expect()` in production code
- Use `anyhow::Context` for adding context to errors in application code

### Testing
- Integration tests in `tests/` directory
- Property-based testing with `quickcheck` for opcode invariants
- Mock ledger for isolated VM testing
- Test both with and without optional features

### Security Considerations
- This is a blockchain VM implementation - security is critical
- All input validation must be comprehensive
- Cost accounting prevents DoS attacks via resource exhaustion
- Stack and memory bounds are strictly enforced

### Performance
- VM operations are cost-accounted to prevent abuse
- Maximum stack size: 1000 elements
- Maximum call stack depth: 8 levels
- Scratch space: 256 slots

## Important Notes

- The VM uses a dual-licensing scheme (MIT OR Apache-2.0)
- Based on the official Algorand go-algorand implementation
- Supports multiple TEAL versions with version-specific feature sets
- CLI provides comprehensive debugging capabilities including step-by-step execution
