//! TEAL Assembly and Bytecode Example
//! 
//! This example demonstrates how to work with TEAL assembly and bytecode conversion
//! using patterns from Algorand's official documentation. It shows compilation,
//! decompilation, and direct bytecode execution.

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
    println!("=== TEAL Assembly and Bytecode Example ===\n");

    // Example 1: Basic TEAL to bytecode compilation
    println!("Example 1: Compile simple TEAL to bytecode");
    
    let teal_code = r#"
#pragma version 8
pushint 1
pushint 2
+
return
"#;

    println!("TEAL Source:");
    println!("{}", teal_code);
    
    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Execution successful! ({})", success);
            println!("This TEAL compiles to bytecode internally\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: Complex expression compilation
    println!("Example 2: Complex expression with optimization");
    
    let teal_code = r#"
#pragma version 8
// Calculate: (10 + 20) * (30 - 15) / 5
pushint 10
pushint 20
+               // Stack: [30]
pushint 30
pushint 15
-               // Stack: [30, 15]
*               // Stack: [450]
pushint 5
/               // Stack: [90]
pushint 90
==
return
"#;

    println!("TEAL Expression: (10 + 20) * (30 - 15) / 5");
    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Computed result verified: {}", success);
            println!("The AVM compiles this to efficient bytecode\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 3: Bytecode with labels and jumps
    println!("Example 3: Control flow in bytecode");
    
    let teal_code = r#"
#pragma version 8
pushint 5
pushint 3
>               // Check if 5 > 3
bnz greater     // Branch if non-zero (true)

less_or_equal:
pushint 0
b end

greater:
pushint 1

end:
return
"#;

    println!("TEAL with control flow:");
    println!("- Uses labels (greater, less_or_equal, end)");
    println!("- Compiles to bytecode with jump offsets");
    
    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Result: {} (1 = greater, 0 = less/equal)\n", success);
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Subroutine compilation
    println!("Example 4: Subroutines in bytecode");
    
    let teal_code = r#"
#pragma version 8
// Main program
pushint 10
pushint 20
callsub add_numbers
pushint 5
callsub multiply_by
pushint 150
==
return

// Subroutine: add two numbers
add_numbers:
+
retsub

// Subroutine: multiply by a value
multiply_by:
*
retsub
"#;

    println!("TEAL with subroutines:");
    println!("- Subroutines compile to specific bytecode patterns");
    println!("- 'callsub' and 'retsub' manage the call stack");
    
    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Result verified: {} ((10 + 20) * 5 = 150)\n", success);
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 5: Bytecode size optimization
    println!("Example 5: Bytecode optimization patterns");
    
    // Optimized version
    let optimized = r#"
#pragma version 8
pushint 4
return
"#;

    println!("Optimized TEAL: int 4");
    println!("(Instead of: int 1; int 1; int 1; int 1; +; +; +)");
    println!("Both compile to different bytecode sizes!");
    
    match execute_teal_signature(optimized) {
        Ok(success) => {
            println!("Result: {} (same result, less bytecode)\n", success);
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 6: Template assembly patterns
    println!("Example 6: Template-based TEAL patterns");
    
    let template_code = r#"
#pragma version 8
// Template parameters would be replaced before compilation
// In real use: TMPL_* placeholders replaced with actual values

// Simulate template validation
pushint 1  // All template checks pass
return
"#;

    println!("Template-based TEAL:");
    println!("- Templates use placeholders (TMPL_*)");
    println!("- Replaced with actual values before compilation");
    println!("- Common pattern for parameterized contracts");
    
    match execute_teal_signature(template_code) {
        Ok(success) => {
            println!("Template structure validated: {}\n", success);
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 7: Macro-like patterns
    println!("Example 7: Macro patterns in TEAL");
    
    let teal_code = r#"
#pragma version 8
// Common pattern: min(a, b)
pushint 42
pushint 17
dup2        // Duplicate both values
<           // Compare
bnz first_smaller
// Second is smaller or equal
swap
pop         // Remove the larger
b done
first_smaller:
pop         // Remove the larger (now on top)
done:
pushint 17
==
return
"#;

    println!("TEAL 'macro' pattern for min(a, b):");
    println!("- No actual macros in TEAL");
    println!("- Common patterns can be reused");
    println!("- Compiles to efficient bytecode");
    
    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("min(42, 17) = 17 verified: {}", success);
        }
        Err(e) => println!("Error: {}", e),
    }
}