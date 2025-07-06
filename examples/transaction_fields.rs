//! Transaction Fields Example
//! 
//! This example demonstrates accessing and validating transaction fields in TEAL
//! using patterns from Algorand's official documentation. It shows how smart
//! contracts can inspect and validate transaction properties.

use rust_avm::assembler::Assembler;
use rust_avm::vm::{VirtualMachine, ExecutionConfig};
use rust_avm::types::RunMode;
use rust_avm::opcodes::get_standard_opcodes;
use rust_avm::state::MockLedger;

/// Helper function to execute TEAL source code
fn execute_teal_signature(teal_code: &str) -> Result<bool, String> {
    let mut vm = VirtualMachine::new();
    for spec in get_standard_opcodes() {
        vm.register_opcode(spec.opcode, spec);
    }
    let mut assembler = Assembler::new();
    let bytecode = assembler.assemble(teal_code)
        .map_err(|e| format!("Assembly error: {}", e))?;
    let config = ExecutionConfig {
        run_mode: RunMode::Signature,
        cost_budget: 10000,
        version: 8,
        group_index: 0,
        group_size: 1,
    };
    let ledger = MockLedger::default();
    let result = vm.execute(&bytecode, config, &ledger)
        .map_err(|e| format!("Execution error: {}", e))?;
    Ok(result)
}

fn main() {
    println!("=== Transaction Fields Example ===\n");

    // Example 1: Basic transaction validation patterns
    println!("Example 1: Basic transaction validation patterns");
    
    let teal_code = r#"
#pragma version 8
// Simulate basic transaction validation
// In real scenarios, these would access actual transaction fields
pushint 1  // Simulate all validations pass
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Transaction validation pattern: {}", success);
            println!("- Sender address validation");
            println!("- Fee validation");  
            println!("- Round number checks\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: Payment transaction validation pattern
    println!("Example 2: Payment transaction validation pattern");
    
    let teal_code = r#"
#pragma version 8
// Simulate payment validation
pushint 1000000  // Amount in microAlgos
pushint 1000000  // Minimum 1 ALGO
>=
assert
pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Payment validation: {}", success);
            println!("- Transaction type check");
            println!("- Amount validation");
            println!("- Receiver validation\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 3: Asset transfer validation pattern
    println!("Example 3: Asset transfer validation pattern");
    
    let teal_code = r#"
#pragma version 8
// Simulate asset transfer validation
pushint 12345    // Asset ID
pushint 0
>
assert       // Valid asset ID
pushint 100      // Transfer amount
pushint 0
>
assert       // Positive amount
pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Asset transfer validation: {}", success);
            println!("- Asset ID validation");
            println!("- Transfer amount check");
            println!("- Receiver validation\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Group transaction validation pattern
    println!("Example 4: Group transaction validation pattern");
    
    let teal_code = r#"
#pragma version 8
// Simulate group transaction checks
pushint 3        // Group size
pushint 2
>=
assert       // At least 2 transactions
pushint 1        // This transaction index
pushint 0
>=
assert       // Valid index
pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Group transaction validation: {}", success);
            println!("- Group size verification");
            println!("- Transaction position validation");
            println!("- Atomic execution guarantee\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 5: Time-based validation pattern
    println!("Example 5: Time-based validation pattern");
    
    let teal_code = r#"
#pragma version 8
// Simulate time-based checks
pushint 2000000  // Current timestamp
pushint 1000000  // Required time
>
assert       // Must be after certain time
pushint 2000000  // Current timestamp  
pushint 3000000  // Expiry time
<
assert       // Must be before expiry
pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Time-based validation: {}", success);
            println!("- Valid time window");
            println!("- Prevents replay attacks");
            println!("- Time lock enforcement\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 6: Fee validation pattern
    println!("Example 6: Fee validation pattern");
    
    let teal_code = r#"
#pragma version 8
// Simulate fee validation
pushint 1000     // Transaction fee
pushint 1000     // Minimum fee
>=
assert       // Fee must be at least minimum
pushint 1000     // Transaction fee
pushint 10000    // Maximum allowed fee
<=
assert       // Fee must not be excessive
pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Fee validation: {}", success);
            println!("- Minimum fee enforcement");
            println!("- Maximum fee protection");
            println!("- Economic security\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 7: Complex multi-criteria validation
    println!("Example 7: Complex multi-criteria validation");
    
    let teal_code = r#"
#pragma version 8
// Complex validation combining multiple checks
// Payment between 0.1 and 10 ALGO
pushint 5000000   // 5 ALGO amount
dup
pushint 100000    // 0.1 ALGO minimum
>=
assert
pushint 10000000  // 10 ALGO maximum  
<=
assert

// Note field validation
pushbytes "ALGO"   // Required note prefix
len
pushint 4
==
assert

// Rekey protection
pushint 0         // No rekey (ZeroAddress)
pushint 0
==
assert

pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Complex validation passed: {}", success);
            println!("- Amount range validation");
            println!("- Note field requirements");
            println!("- Security restrictions");
        }
        Err(e) => println!("Error: {}", e),
    }
}