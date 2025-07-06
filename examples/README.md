# Rust AVM Examples

This directory contains examples demonstrating various features of the Algorand Virtual Machine (AVM) using TEAL code from Algorand's official documentation.

## Running the Examples

Each example can be run using cargo:

```bash
cargo run --example basic_arithmetic
cargo run --example crypto_operations
cargo run --example smart_contract
cargo run --example teal_assembly
cargo run --example control_flow
cargo run --example transaction_fields
```

## Examples Overview

### 1. Basic Arithmetic (`basic_arithmetic.rs`)
Demonstrates fundamental arithmetic operations in the AVM:
- Addition, subtraction, multiplication, division
- Modulo operations
- Comparison operators
- Bitwise operations (AND, OR, XOR)
- Mathematical functions (sqrt, exp)
- Min/Max patterns

**Key concepts**: Stack manipulation, integer operations, comparison logic

### 2. Cryptographic Operations (`crypto_operations.rs`)
Shows cryptographic primitives available in TEAL:
- SHA256 hashing
- Keccak256 hashing
- Ed25519 signature verification patterns
- Address validation
- Multi-signature logic
- Hash time locks (for atomic swaps)
- Recursive hashing

**Key concepts**: Hashing, signature verification, cryptographic patterns

### 3. Smart Contract State (`smart_contract.rs`)
Illustrates stateful smart contract patterns:
- Global state management
- Local state per account
- Counter application
- Escrow with timeout
- Voting system
- Token vault
- Access control patterns

**Key concepts**: Application mode, state storage, access control

### 4. TEAL Assembly (`teal_assembly.rs`)
Explores TEAL assembly and bytecode concepts:
- TEAL source to bytecode compilation
- Control flow compilation
- Subroutine patterns
- Bytecode optimization
- Template-based contracts
- Macro-like patterns

**Key concepts**: Assembly structure, compilation, optimization

### 5. Control Flow (`control_flow.rs`)
Demonstrates control flow constructs:
- If-then-else patterns
- Multi-way branching (switch-like)
- Loop implementation
- Nested subroutines
- Short-circuit evaluation
- Error handling patterns
- Complex algorithms (binary search)

**Key concepts**: Branching, loops, subroutines, complex logic

### 6. Transaction Fields (`transaction_fields.rs`)
Shows how to access and validate transaction data:
- Basic transaction field access
- Payment transaction validation
- Asset transfer validation
- Group transaction handling
- Application call validation
- Lease-based replay protection
- Complex multi-criteria validation

**Key concepts**: Transaction introspection, validation patterns

## TEAL Version

All examples use TEAL version 8 (`#pragma version 8`) which provides the latest features and opcodes.

## Learning Path

1. Start with **basic_arithmetic** to understand stack operations
2. Move to **control_flow** to learn program structure
3. Study **transaction_fields** for transaction validation
4. Explore **crypto_operations** for security features
5. Learn **smart_contract** for stateful applications
6. Understand **teal_assembly** for optimization

## Additional Resources

- [Algorand Developer Documentation](https://developer.algorand.org/)
- [TEAL Specification](https://developer.algorand.org/docs/get-details/dapps/avm/teal/)
- [TEAL Opcodes Reference](https://developer.algorand.org/docs/get-details/dapps/avm/teal/opcodes/)

## Contributing

When adding new examples:
1. Use real TEAL patterns from official documentation
2. Include comprehensive comments explaining the logic
3. Show both simple and complex use cases
4. Update this README with the new example description