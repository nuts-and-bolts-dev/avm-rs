//! Control Flow Example
//! 
//! This example demonstrates control flow constructs in TEAL using patterns
//! from Algorand's official documentation. It shows branching, loops,
//! subroutines, and complex control structures.

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
    println!("=== Control Flow Example ===\n");

    // Example 1: Basic if-then-else
    println!("Example 1: If-then-else pattern");
    
    let teal_code = r#"
#pragma version 8
// Check if a number is even or odd
pushint 42
pushint 2
%               // 42 % 2
pushint 0
==              // Check if remainder is 0
return          // Return true for even, false for odd
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("If-then-else structure: {}", success);
            println!("42 is even (remainder 0 when divided by 2)\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: Multi-way branching (switch-like)
    println!("Example 2: Multi-way branching");
    
    let teal_code = r#"
#pragma version 8
// Simulate a switch statement on a value
pushint 2           // Value to switch on
dup
pushint 1
==
bnz case_one
dup
pushint 2
==
bnz case_two
dup
pushint 3
==
bnz case_three
b default_case

case_one:
pop             // Remove the duplicated value
pushint 100
b end_switch

case_two:
pop
pushint 200
b end_switch

case_three:
pop
pushint 300
b end_switch

default_case:
pop
pushint 999

end_switch:
pushint 200
==
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Switch on value 2 returned correct result: {}", success);
            println!("Demonstrates multi-way branching pattern\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 3: Factorial using subroutines
    println!("Example 3: Factorial calculation with subroutines");
    
    let teal_code = r#"
#pragma version 8
// Calculate factorial of 4 = 24
pushint 4
callsub factorial
pushint 24
==
return

factorial:
// Simple factorial: if n <= 1 return 1, else return n * factorial(n-1)
// For demo, we'll use iterative approach
dup
pushint 1
<=
bnz factorial_base
// n > 1, calculate iteratively
pushint 1           // accumulator
swap
factorial_loop:
dup
pushint 1
<=
bnz factorial_done
swap
dig 1
*               // acc = acc * n
swap
pushint 1
-               // n = n - 1
b factorial_loop
factorial_done:
pop             // remove n
retsub
factorial_base:
pop
pushint 1
retsub
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("4! = 24 calculated correctly: {}", success);
            println!("Loop implemented using labels and branches\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 4: Short-circuit evaluation
    println!("Example 4: Short-circuit AND evaluation");
    
    let teal_code = r#"
#pragma version 8
// Implement: (false && expensive_check()) || true
// Using short-circuit evaluation

// First check (false)
pushint 0
dup
bnz check_second  // Only check second if first is true

// First was false, skip to OR
pop
b try_fallback

check_second:
pop
// Second check (expensive operation)
pushint 1
dup
bnz and_success

// AND failed
pop
b try_fallback

and_success:
// Both checks passed, no need for fallback
b done

try_fallback:
// Fallback check
pushint 1           // True

done:
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Short-circuit result: {}", success);
            println!("Avoided unnecessary expensive operations\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 5: Error handling pattern
    println!("Example 5: Error handling with bounds checking");
    
    let teal_code = r#"
#pragma version 8
// Safe division with zero check
pushint 100         // Dividend
pushint 5           // Divisor

// Check for division by zero
dup
pushint 0
==
bnz error_handler

// Safe to divide
/
pushint 20          // Expected result
==
return

error_handler:
// Handle division by zero
pop
pop
pushint 0           // Error case
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Safe division result verified: {}", success);
            println!("Error handling prevents division by zero\n");
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 6: Complex nested conditions
    println!("Example 6: Complex nested conditions");
    
    let teal_code = r#"
#pragma version 8
// Check if number is in range [10, 50] and even
pushint 24          // Test value

// First check: is it >= 10?
dup
pushint 10
>=
bz out_of_range

// Second check: is it <= 50?
dup
pushint 50
<=
bz out_of_range

// Third check: is it even?
dup
pushint 2
%
pushint 0
==
bz not_even

// All checks passed
pop
pushint 1
return

out_of_range:
pop
pushint 0
return

not_even:
pop
pushint 0
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Complex condition check (24 in [10,50] and even): {}", success);
            println!("Multiple nested conditions with early exits");
        }
        Err(e) => println!("Error: {}", e),
    }
}