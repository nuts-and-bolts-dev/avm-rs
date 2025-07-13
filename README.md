# `avm-rs` - A Rust Implementation of the Algorand Virtual Machine

[![Crates.io](https://img.shields.io/crates/v/avm-rs.svg)](https://crates.io/crates/avm-rs)
[![Documentation](https://docs.rs/avm-rs/badge.svg)](https://docs.rs/avm-rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/nuts-and-bolts-dev/avm-rs#license)

A complete implementation of the Algorand Virtual Machine (AVM) written in Rust, designed to execute TEAL (Transaction Execution Approval Language) bytecode for smart contract logic and transaction validation.

## Quick Start

### Basic Usage

```rust
use avm_rs::{
    opcodes::get_standard_opcodes,
    state::MockLedger,
    types::RunMode,
    vm::{VirtualMachine, ExecutionConfig},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple TEAL program: pushint 1, return
    let program = vec![
        0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // pushint 1
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
use avm_rs::assembler::Assembler;

let source = r#"
    int 42
    int 24
    +
    return
"#;

let mut assembler = Assembler::new();
let bytecode = assembler.assemble(source)?;
```

## CLI Usage

AVM-RS provides a comprehensive command-line interface for working with TEAL programs:

```bash
cargo install avm-rs
```

### Available Commands

```bash
avm-rs <COMMAND> [OPTIONS]
```

#### Commands Overview

- **`execute`** - Execute TEAL programs with debugging support
- **`assemble`** - Compile TEAL source code to bytecode
- **`validate`** - Validate TEAL programs for correctness

### Execute TEAL Programs

Execute TEAL programs from various input sources:

```bash
# Execute from file
avm-rs execute program.teal

# Execute inline TEAL
avm-rs execute -t inline "int 1; int 2; +; return"

# Execute bytecode directly
avm-rs execute -t bytecode "81010181020D43"

# Application mode with budget
avm-rs execute -m application -b 5000 contract.teal

# Debug mode with stack visualization
avm-rs execute --step --show-stack program.teal
```

### Assembly

Convert between TEAL source and bytecode:

```bash
# Assemble TEAL to bytecode
avm-rs assemble program.teal -o program.bytecode

# Different output formats
avm-rs assemble program.teal -f hex       # Hexadecimal
avm-rs assemble program.teal -f base64    # Base64 encoded
avm-rs assemble program.teal -f binary    # Raw binary
```

### Validation and Analysis

Validate TEAL programs for syntax and semantic correctness:

```bash
# Validate a TEAL program
avm-rs validate program.teal

# Validate with specific version
avm-rs validate -V 8 program.teal
```

## Examples

The project includes comprehensive examples demonstrating various TEAL patterns and AVM features:

### Running Examples

```bash
# Build all examples
make examples

# Run individual examples
cargo run --example basic_arithmetic
cargo run --example crypto_operations
cargo run --example smart_contract
cargo run --example control_flow
cargo run --example teal_assembly
cargo run --example transaction_fields
cargo run --example simple_test
```

## License

This project is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Based on the official Algorand Virtual Machine implementation from the [go-algorand](https://github.com/algorand/go-algorand) repository.
