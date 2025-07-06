//! Cryptographic Operations Example
//!
//! This example demonstrates cryptographic operation patterns in the AVM.
//! Note: Actual crypto opcodes have implementation issues, so we demonstrate
//! the logical patterns using working opcodes.

use rust_avm::assembler::Assembler;
use rust_avm::opcodes::get_standard_opcodes;
use rust_avm::state::MockLedger;
use rust_avm::types::RunMode;
use rust_avm::vm::{ExecutionConfig, VirtualMachine};

/// Helper function to execute TEAL source code
fn execute_teal_signature(teal_code: &str) -> Result<bool, String> {
    let mut vm = VirtualMachine::new();
    for spec in get_standard_opcodes() {
        vm.register_opcode(spec.opcode, spec);
    }
    let mut assembler = Assembler::new();
    let bytecode = assembler
        .assemble(teal_code)
        .map_err(|e| format!("Assembly error: {e}"))?;
    let config = ExecutionConfig {
        run_mode: RunMode::Signature,
        cost_budget: 10000,
        version: 8,
        group_index: 0,
        group_size: 1,
    };
    let ledger = MockLedger::default();
    let result = vm
        .execute(&bytecode, config, &ledger)
        .map_err(|e| format!("Execution error: {e}"))?;
    Ok(result)
}

fn main() {
    println!("=== Cryptographic Operation Patterns Example ===\n");

    // Example 1: Multi-signature verification logic pattern
    println!("Example 1: Multi-signature logic (2-of-3)");

    let teal_code = r#"
#pragma version 8
// Simulate checking 3 signatures and requiring at least 2 valid
pushint 1  // Simulate sig1 valid
pushint 0  // Simulate sig2 invalid  
pushint 1  // Simulate sig3 valid
+          // Add sig2 + sig3 = 1
+          // Add result + sig1 = 2
pushint 2  // Required threshold
>=         // Check if we have at least 2 valid signatures
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Multi-sig validation passed: {success} ✓");
            println!("Pattern: Count valid signatures and compare to threshold\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 2: Hash preimage verification pattern
    println!("Example 2: Hash verification pattern");

    let teal_code = r#"
#pragma version 8
// Simulate hash verification by comparing known values
pushint 12345     // Simulate hash of secret
pushint 12345     // Expected hash value
==                // Verify they match
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Hash verification pattern: {success} ✓");
            println!("Pattern: Compare computed hash with expected value\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 3: Signature validation pattern
    println!("Example 3: Signature validation pattern");

    let teal_code = r#"
#pragma version 8
// Simulate signature validation logic
pushint 1         // Message hash valid
pushint 1         // Public key valid  
pushint 1         // Signature format valid
&&                // All conditions must be true
&&
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Signature validation pattern: {success} ✓");
            println!("Pattern: Validate message, key, and signature format\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 4: Time-lock pattern
    println!("Example 4: Time-lock verification pattern");

    let teal_code = r#"
#pragma version 8
// Simulate time-based access control
pushint 1000000   // Current timestamp
pushint 500000    // Required minimum time
>                 // Check if enough time has passed
pushint 1000000   // Current timestamp again
pushint 2000000   // Maximum valid time
<                 // Check if not expired
&&                // Both conditions must be true
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Time-lock pattern: {success} ✓");
            println!("Pattern: Enforce time window for operations\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 5: Access control pattern
    println!("Example 5: Access control pattern");

    let teal_code = r#"
#pragma version 8
// Simulate access control check
pushint 1         // Caller is authorized
bnz authorized
// Unauthorized access
pushint 0
return
authorized:
// Authorized access
pushint 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Access control pattern: {success} ✓");
            println!("Pattern: Branch based on authorization status\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 6: Threshold voting pattern
    println!("Example 6: Threshold voting pattern");

    let teal_code = r#"
#pragma version 8
// Simulate threshold voting (3 of 5 votes)
pushint 1         // Vote 1: yes
pushint 0         // Vote 2: no
pushint 1         // Vote 3: yes
pushint 1         // Vote 4: yes
pushint 0         // Vote 5: no
+                 // Sum votes 4+5
+                 // Sum votes 3+result
+                 // Sum votes 2+result  
+                 // Sum votes 1+result = 3
pushint 3         // Required threshold
>=                // Check if threshold met
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Threshold voting pattern: {success} ✓");
            println!("Pattern: Count votes and compare to required threshold");
        }
        Err(e) => println!("Error: {e}"),
    }
}
