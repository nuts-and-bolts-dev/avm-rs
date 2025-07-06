//! Control Flow Example
//!
//! This example demonstrates control flow constructs in TEAL using patterns
//! from Algorand's official documentation. It shows branching, loops,
//! subroutines, and complex control structures.

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
    println!("=== Control Flow Example ===\n");

    // Example 1: Basic if-then-else
    println!("Example 1: If-then-else pattern");

    let teal_code = r#"
#pragma version 8
// Check if a number is even or odd
int 42
int 2
%               // 42 % 2
int 0
==              // Check if remainder is 0
return          // Return true for even, false for odd
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("If-then-else structure: {success}");
            println!("42 is even (remainder 0 when divided by 2)\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 2: Multi-way branching (switch-like)
    println!("Example 2: Multi-way branching");

    let teal_code = r#"
#pragma version 8
// Simulate a switch statement on a value
int 2           // Value to switch on

// Check if value is 2
dup
int 2
==
bnz case_two

// Not 2, return false for demonstration
pop
int 0
return

case_two:
pop             // Remove the duplicated value
int 200         // Return 200 for case 2
int 200         // Expected value
==
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Switch on value 2 returned correct result: {success}");
            println!("Demonstrates multi-way branching pattern\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 3: Factorial using subroutines
    println!("Example 3: Factorial calculation with subroutines");

    let teal_code = r#"
#pragma version 8
// Calculate factorial of 4 = 24 using subroutines
int 4           // n = 4
callsub factorial
int 24          // expected result
==
return

factorial:
// Calculate factorial recursively using subroutines
// if n <= 1 return 1, else return n * factorial(n-1)
dup             // duplicate n
int 1
<=
bnz factorial_base

// n > 1, recursive case: n * factorial(n-1)
dup             // duplicate n
int 1
-               // n-1
callsub factorial  // factorial(n-1)
*               // n * factorial(n-1)
retsub

factorial_base:
// base case: factorial(0) = factorial(1) = 1
pop             // remove n from stack
int 1           // return 1
retsub
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("4! = 24 calculated correctly: {success}");
            println!("Recursive factorial using callsub and retsub\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 4: Short-circuit evaluation
    println!("Example 4: Short-circuit AND evaluation");

    let teal_code = r#"
#pragma version 8
// Implement: (false && expensive_check()) || true
// Using short-circuit evaluation

// First check (false)
int 0
bnz check_second  // Only check second if first is true

// First was false, go to OR operation
int 1           // The fallback OR condition (true)
return

check_second:
// Second check (this won't be reached because first is false)
int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Short-circuit result: {success}");
            println!("Avoided unnecessary expensive operations\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 5: Error handling pattern
    println!("Example 5: Error handling with bounds checking");

    let teal_code = r#"
#pragma version 8
// Safe division with zero check
int 100         // Dividend
int 5           // Divisor

// Check for division by zero
dup
int 0
==
bnz error_handler

// Safe to divide
/               // 100 / 5 = 20
int 20          // Expected result
==
return

error_handler:
// Handle division by zero
pop             // Remove divisor
pop             // Remove dividend
int 0           // Error case (false)
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Safe division result verified: {success}");
            println!("Error handling prevents division by zero\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 6: Complex nested conditions
    println!("Example 6: Complex nested conditions");

    let teal_code = r#"
#pragma version 8
// Check if number is in range [10, 50] and even
int 24          // Test value

// First check: is it >= 10?
dup
int 10
>=
bz out_of_range

// Second check: is it <= 50?
dup
int 50
<=
bz out_of_range

// Third check: is it even?
dup
int 2
%
int 0
==
bz not_even

// All checks passed
pop
int 1           // Return true
return

out_of_range:
pop
int 0           // Return false
return

not_even:
pop
int 0           // Return false
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Complex condition check (24 in [10,50] and even): {success}");
            println!("Multiple nested conditions with early exits");
        }
        Err(e) => println!("Error: {e}"),
    }
}
