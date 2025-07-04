# Rust AVM - Algorand Virtual Machine Implementation

A complete implementation of the Algorand Virtual Machine (AVM) written in Rust, designed to execute TEAL (Transaction Execution Approval Language) bytecode for smart contract logic and transaction validation.

## Features

- **Complete AVM Implementation**: Full support for TEAL bytecode execution
- **50+ Opcodes**: Comprehensive opcode support including arithmetic, cryptographic, stack manipulation, and state access operations
- **Type Safety**: Leverages Rust's type system for memory safety and error handling
- **Stack-based Execution**: Efficient stack-based virtual machine with cost model enforcement
- **TEAL Assembler**: Built-in assembler for converting TEAL source code to bytecode
- **State Management**: Trait-based interface for blockchain state access
- **Cryptographic Operations**: Support for Ed25519, SHA256, Keccak256, and other crypto functions
- **Dual Execution Modes**: Support for both signature verification and application execution modes

## Architecture

The project is organized into several key modules:

- **`vm`**: Core virtual machine execution engine
- **`opcodes`**: Opcode implementations organized by category
- **`assembler`**: TEAL source code to bytecode compiler
- **`state`**: Blockchain state access interfaces and mock implementations
- **`crypto`**: Cryptographic function implementations
- **`types`**: Core data types and value representations
- **`error`**: Comprehensive error handling with descriptive error types

## Quick Start

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo

### Installation

```bash
git clone <repository-url>
cd rust-avm
cargo build --release
```

### Basic Usage

```rust
use rust_avm::{
    opcodes::get_standard_opcodes,
    state::MockLedger,
    types::RunMode,
    vm::{VirtualMachine, ExecutionConfig},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple TEAL program: pushint 1, return
    let program = vec![
        0x82, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // pushint 1
        0x43, // return
    ];

    // Set up VM with standard opcodes
    let mut vm = VirtualMachine::new();
    for spec in get_standard_opcodes() {
        vm.register_opcode(spec.opcode, spec);
    }

    // Configure execution
    let config = ExecutionConfig {
        run_mode: RunMode::Signature,
        cost_budget: 1000,
        version: 2,
        group_index: 0,
        group_size: 1,
    };

    // Create ledger state
    let ledger = MockLedger::new();

    // Execute program
    let result = vm.execute(&program, config, &ledger)?;
    println!("Program result: {}", result);

    Ok(())
}
```

### TEAL Assembly

```rust
use rust_avm::assembler::Assembler;

let source = r#"
    pushint 42
    pushint 24
    +
    return
"#;

let mut assembler = Assembler::new();
let bytecode = assembler.assemble(source)?;
```

## Supported Opcodes

### Arithmetic Operations
- Basic arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparisons: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Logical operations: `&&`, `||`, `!`
- Bitwise operations: `|`, `&`, `^`, `~`

### Stack Management
- `dup`, `dup2` - Duplicate stack values
- `pop` - Remove top value
- `swap` - Swap top two values
- `select` - Conditional value selection

### Flow Control
- `bnz`, `bz`, `b` - Conditional and unconditional branches
- `return` - Program termination
- `assert` - Runtime assertions
- `callsub`, `retsub` - Subroutine calls

### Cryptographic Functions
- `sha256`, `keccak256`, `sha512_256` - Hash functions
- `ed25519verify` - Signature verification
- ECDSA operations (placeholder implementations)

### State Access (Application Mode)
- `app_global_get`, `app_global_put`, `app_global_del` - Global state
- `app_local_get`, `app_local_put`, `app_local_del` - Local state
- `balance`, `min_balance` - Account information
- Asset and application parameter access

### Transaction Fields
- `txn`, `gtxn` - Transaction field access
- `global` - Global blockchain parameters

## Development

### Running Tests

```bash
cargo test
```

### Code Quality

The project uses comprehensive linting and formatting:

```bash
# Run all CI checks
make ci

# Individual commands
make fmt      # Format code
make clippy   # Run linter
make test     # Run tests
make build    # Build project
```

### Project Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # Example program
├── error.rs            # Error types and handling
├── types.rs            # Core data types
├── vm/
│   └── mod.rs          # Virtual machine implementation
├── opcodes/
│   ├── mod.rs          # Opcode registry and specifications
│   ├── arithmetic.rs   # Arithmetic operations
│   ├── crypto.rs       # Cryptographic operations
│   ├── flow.rs         # Control flow operations
│   ├── stack.rs        # Stack manipulation
│   ├── state.rs        # State access operations
│   └── transaction.rs  # Transaction field access
├── assembler/
│   └── mod.rs          # TEAL assembler implementation
├── state/
│   └── mod.rs          # State management interfaces
└── crypto/
    └── mod.rs          # Cryptographic utilities
```

## Design Principles

### Shape Up Methodology

This project follows the Shape Up methodology for development:

- **Fixed Time, Variable Scope**: Features are designed to fit within time boundaries
- **Right Level of Abstraction**: Solutions are specified at the appropriate detail level
- **Risk Management**: Potential issues are identified and mitigated early

### Security First

- All operations use safe Rust patterns
- Comprehensive error handling prevents panics
- Input validation on all external data
- Memory safety guaranteed by Rust's ownership system

### Production Ready

- Robust error handling with descriptive error types
- Comprehensive logging and debugging support
- Configurable execution parameters
- Clean separation of concerns

## Compatibility

This implementation aims for compatibility with the official go-algorand AVM while leveraging Rust's safety features:

- **Opcode Behavior**: Matches go-algorand execution semantics
- **TEAL Version Support**: Supports multiple TEAL versions
- **Cost Model**: Implements the same cost accounting as go-algorand
- **Error Handling**: Provides similar error conditions with improved type safety

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure all tests pass: `make ci`
5. Submit a pull request

### Code Style

- Follow Rust naming conventions
- Use descriptive error messages
- Add documentation for public APIs
- Include tests for new functionality

## License

[Add your license here]

## Acknowledgments

Based on the official Algorand Virtual Machine implementation from the [go-algorand](https://github.com/algorand/go-algorand) repository.