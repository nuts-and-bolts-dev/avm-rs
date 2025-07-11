//! Simple Test Example
//!
//! This is a basic test to verify the avm-rs examples are working correctly.

use avm_rs::assembler::Assembler;
use avm_rs::state::MockLedger;
use avm_rs::{TealVersion, VirtualMachine};

fn main() {
    println!("=== Simple AVM Test ===\n");

    // Create VM using the builder pattern for demonstration
    let vm = VirtualMachine::builder().version(TealVersion::V8).build();

    // Test 1: Basic arithmetic (known working)
    println!("Test 1: Basic arithmetic (5 + 3 = 8)");
    let teal_code = r#"
#pragma version 8
int 5
int 3
+
int 8
==
return
"#;

    match execute_teal(&vm, teal_code) {
        Ok(success) => println!("Result: {success} ✓"),
        Err(e) => println!("Error: {e}"),
    }

    // Test 2: Simple string comparison
    println!("\nTest 2: String comparison");
    let teal_code = r#"
#pragma version 8
byte "hello"
byte "hello"
==
return
"#;

    match execute_teal(&vm, teal_code) {
        Ok(success) => println!("Result: {success} ✓"),
        Err(e) => println!("Error: {e}"),
    }

    // Test 3: Simple logical operations
    println!("\nTest 3: Logical AND operation");
    let teal_code = r#"
#pragma version 8
int 1
int 1
&&
return
"#;

    match execute_teal(&vm, teal_code) {
        Ok(success) => println!("Result: {success} ✓"),
        Err(e) => println!("Error: {e}"),
    }

    // Test 4: Simple comparison
    println!("\nTest 4: Simple comparison (75 > 50)");
    let teal_code = r#"
#pragma version 8
int 75
int 50
>
return
"#;

    match execute_teal(&vm, teal_code) {
        Ok(success) => println!("Result: {success} ✓"),
        Err(e) => println!("Error: {e}"),
    }

    // Test 5: Simple branch
    println!("\nTest 5: Simple branch");
    let teal_code = r#"
#pragma version 8
int 1
bnz success
int 0
return
success:
int 1
return
"#;

    match execute_teal(&vm, teal_code) {
        Ok(success) => println!("Result: {success} ✓"),
        Err(e) => println!("Error: {e}"),
    }

    println!("\n=== All tests completed! ===");
}

/// Helper function to execute TEAL source code
fn execute_teal(vm: &VirtualMachine, teal_code: &str) -> Result<bool, String> {
    let mut assembler = Assembler::new();
    let bytecode = assembler
        .assemble(teal_code)
        .map_err(|e| format!("Assembly error: {e}"))?;

    let mut ledger = MockLedger::default();

    // Use the simple execution method
    let result = vm
        .execute_simple(&bytecode, TealVersion::V8, &mut ledger)
        .map_err(|e| format!("Execution error: {e}"))?;
    Ok(result)
}
