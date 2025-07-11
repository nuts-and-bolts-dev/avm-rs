# AVM-RS - Algorand Virtual Machine Implementation

A complete implementation of the Algorand Virtual Machine (AVM) written in Rust, designed to execute TEAL (Transaction Execution Approval Language) bytecode for smart contract logic and transaction validation.
## Quick Start

### Prerequisites

- Rust 1.88+ (2024 edition)
- Cargo

### Installation

```bash
git clone <repository-url>
cd avm-rs
cargo build --release
```

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

## Examples

The project includes comprehensive examples demonstrating various TEAL patterns and AVM features:

### Running Examples

```bash
# Build all examples
make examples

# Run all examples sequentially
make run-examples

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