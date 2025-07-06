//! Simple Test Example
//! 
//! This is a basic test to verify the rust-avm examples are working correctly.

use rust_avm::assembler::Assembler;
use rust_avm::vm::{VirtualMachine, ExecutionConfig};
use rust_avm::types::RunMode;
use rust_avm::opcodes::get_standard_opcodes;
use rust_avm::state::MockLedger;

fn main() {
    println!("=== Simple AVM Test ===\n");

    // Test 1: Basic arithmetic (known working)
    println!("Test 1: Basic arithmetic (5 + 3 = 8)");
    let teal_code = r#"
#pragma version 8
pushint 5
pushint 3
+
pushint 8
==
return
"#;

    match execute_teal(teal_code, RunMode::Signature) {
        Ok(success) => println!("Result: {} ✓", success),
        Err(e) => println!("Error: {}", e),
    }

    // Test 2: Simple string operation
    println!("\nTest 2: String length");
    let teal_code = r#"
#pragma version 8
pushbytes "hello"
len
pushint 5
==
return
"#;

    match execute_teal(teal_code, RunMode::Signature) {
        Ok(success) => println!("Result: {} ✓", success),
        Err(e) => println!("Error: {}", e),
    }

    // Test 3: Test crypto alone (SHA256)
    println!("\nTest 3: Simple SHA256");
    let teal_code = r#"
#pragma version 8
pushbytes "test"
sha256
return
"#;

    match execute_teal(teal_code, RunMode::Signature) {
        Ok(success) => println!("Result: {} ✓", success),
        Err(e) => println!("Error: {}", e),
    }

    // Test 4: Simple comparison
    println!("\nTest 4: Simple comparison (75 > 50)");
    let teal_code = r#"
#pragma version 8
pushint 75
pushint 50
>
return
"#;

    match execute_teal(teal_code, RunMode::Signature) {
        Ok(success) => println!("Result: {} ✓", success),
        Err(e) => println!("Error: {}", e),
    }

    // Test 5: Simple branch
    println!("\nTest 5: Simple branch");
    let teal_code = r#"
#pragma version 8
pushint 1
bnz success
pushint 0
return
success:
pushint 1
return
"#;

    match execute_teal(teal_code, RunMode::Signature) {
        Ok(success) => println!("Result: {} ✓", success),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== All tests completed! ===");
}

/// Helper function to execute TEAL source code
fn execute_teal(teal_code: &str, run_mode: RunMode) -> Result<bool, String> {
    let mut vm = VirtualMachine::new();
    for spec in get_standard_opcodes() {
        vm.register_opcode(spec.opcode, spec);
    }
    let mut assembler = Assembler::new();
    let bytecode = assembler.assemble(teal_code)
        .map_err(|e| format!("Assembly error: {}", e))?;
    let config = ExecutionConfig {
        run_mode,
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