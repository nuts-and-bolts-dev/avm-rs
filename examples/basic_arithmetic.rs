//! Basic Arithmetic Operations Example
//!
//! This example demonstrates basic arithmetic operations in the AVM using TEAL code
//! from Algorand's official documentation. It shows how to perform calculations,
//! work with the stack, and use comparison operations.

use rust_avm::assembler::Assembler;
use rust_avm::state::MockLedger;
use rust_avm::{ExecutionConfig, TealVersion, VirtualMachine};

/// Helper function to execute TEAL source code
fn execute_teal_signature(teal_code: &str) -> Result<bool, String> {
    // Use the ergonomic API - VM with standard opcodes for version 8
    let vm = VirtualMachine::with_version(TealVersion::V8);

    let mut assembler = Assembler::new();
    let bytecode = assembler
        .assemble(teal_code)
        .map_err(|e| format!("Assembly error: {e}"))?;

    // Use the fluent configuration API
    let config = ExecutionConfig::new(TealVersion::V8).with_cost_budget(10000);

    let ledger = MockLedger::default();
    let result = vm
        .execute(&bytecode, config, &ledger)
        .map_err(|e| format!("Execution error: {e}"))?;
    Ok(result)
}

fn main() {
    println!("=== Basic Arithmetic Operations Example ===\n");

    // Example 1: Simple addition and multiplication
    // TEAL: (10 + 20) * 3 = 90
    println!("Example 1: (10 + 20) * 3");

    let teal_code = r#"
#pragma version 8
pushint 10
pushint 20
+
pushint 3
*
pushint 90
==
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!(
                "Result: {} (Expected: 90)",
                if success { "✓ Correct" } else { "✗ Failed" }
            );
            println!();
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 2: Division and modulo
    // TEAL: Check 100 / 7 = 14 and 100 % 7 = 2
    println!("Example 2: Division and modulo");

    let teal_code = r#"
#pragma version 8
// Check division: 100 / 7 = 14
pushint 100
pushint 7
/
pushint 14
==
// Check modulo: 100 % 7 = 2  
pushint 100
pushint 7
%
pushint 2
==
// Both must be true
&&
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!(
                "Division and modulo check: {}",
                if success {
                    "✓ Both correct (100/7=14, 100%7=2)"
                } else {
                    "✗ Failed"
                }
            );
            println!();
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 3: Comparison operations - range check
    // TEAL: Check if 75 is in range (50, 100)
    println!("Example 3: Range check (50 < 75 < 100)");

    let teal_code = r#"
#pragma version 8
// Check if 75 > 50
pushint 75      
pushint 50      
>               // Result: 1 (true)
// Check if 75 < 100  
pushint 75      
pushint 100
<               // Result: 1 (true)
// Both conditions must be true
&&              
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!(
                "Is 75 in range (50, 100)? {}",
                if success { "✓ Yes" } else { "✗ No" }
            );
            println!();
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 4: Bitwise operations
    // TEAL: Demonstrating bitwise AND, OR, XOR
    println!("Example 4: Bitwise operations");

    let teal_code = r#"
#pragma version 8
// Test bitwise AND: 10 & 12 = 8
pushint 10
pushint 12
&
pushint 8
==
// Test bitwise OR: 10 | 12 = 14
pushint 10
pushint 12
|
pushint 14
==
&&
// Test bitwise XOR: 10 ^ 12 = 6
pushint 10
pushint 12
^
pushint 6
==
&&
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!(
                "Bitwise operations verified: {}",
                if success {
                    "✓ All correct"
                } else {
                    "✗ Failed"
                }
            );
            println!("10 & 12 = 8 (bitwise AND)");
            println!("10 | 12 = 14 (bitwise OR)");
            println!("10 ^ 12 = 6 (bitwise XOR)\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 5: Integer operations (no sqrt available)
    // TEAL: Power of 2 operations using multiplication
    println!("Example 5: Power operations using multiplication");

    let teal_code = r#"
#pragma version 8
// Calculate 2^3 = 8 using multiplication
pushint 2
pushint 2
*
pushint 2
*
pushint 8
==
// Calculate 3^2 = 9 using multiplication
pushint 3
pushint 3
*
pushint 9
==
&&
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!(
                "Power operations verified: {}",
                if success { "✓ Correct" } else { "✗ Failed" }
            );
            println!("2^3 = 8 (using repeated multiplication)");
            println!("3^2 = 9 (using multiplication)\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 6: Min/Max operations using conditional logic
    // TEAL: Finding minimum of two values using simple comparison
    println!("Example 6: Min operation using comparison");

    let teal_code = r#"
#pragma version 8
// Check if min(42, 17) == 17
// Logic: 42 > 17, so 17 is minimum
pushint 42
pushint 17
>
// This returns true (1), which means 17 is indeed smaller
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!(
                "Min operation result: {}",
                if success {
                    "✓ Correct: min(42, 17) = 17"
                } else {
                    "✗ Failed"
                }
            );
            println!("Logic: if 42 < 17 is false, then 17 is the minimum");
        }
        Err(e) => println!("Error: {e}"),
    }
}
