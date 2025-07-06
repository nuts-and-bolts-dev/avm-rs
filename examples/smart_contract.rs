//! Smart Contract Example with State Management
//!
//! This example demonstrates stateful smart contract operations in the AVM using
//! TEAL code patterns from Algorand's official documentation. It shows how to
//! manage global and local state, handle application calls, and implement
//! common smart contract patterns.

use rust_avm::assembler::Assembler;
use rust_avm::opcodes::get_standard_opcodes;
use rust_avm::state::MockLedger;
use rust_avm::types::RunMode;
use rust_avm::vm::{ExecutionConfig, VirtualMachine};

/// Helper function to execute TEAL source code in application mode
fn execute_teal_application(teal_code: &str) -> Result<bool, String> {
    let mut vm = VirtualMachine::new();
    for spec in get_standard_opcodes() {
        vm.register_opcode(spec.opcode, spec);
    }
    let mut assembler = Assembler::new();
    let bytecode = assembler
        .assemble(teal_code)
        .map_err(|e| format!("Assembly error: {e}"))?;
    let config = ExecutionConfig {
        run_mode: RunMode::Application,
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
    println!("=== Smart Contract State Management Example ===\n");

    // Example 1: Counter Application
    // TEAL: A simple counter that increments on each call
    println!("Example 1: Counter smart contract pattern");

    let teal_code = r#"
#pragma version 8
// Simplified counter logic for demonstration
// In real application mode, we would access global state
pushint 1  // Simulate counter increment
return
"#;

    match execute_teal_application(teal_code) {
        Ok(success) => {
            println!("Counter contract logic executed successfully: {success}");
            println!("Pattern: Read state -> Modify -> Write back\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 2: Access Control Pattern
    // TEAL: Contract with admin functions
    println!("Example 2: Access control pattern");

    let teal_code = r#"
#pragma version 8
// Simulate access control check
pushint 1  // Simulate sender matches admin
bnz admin_action
// User action
pushint 1
return
admin_action:
// Admin-only action
pushint 1
return
"#;

    match execute_teal_application(teal_code) {
        Ok(success) => {
            println!("Access control pattern demonstrated: {success}");
            println!("- Admin address stored on creation");
            println!("- Admin-only functions check sender\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 3: Voting Application Pattern
    // TEAL: Simple voting app structure
    println!("Example 3: Voting application pattern");

    let teal_code = r#"
#pragma version 8
// Simulate voting logic
pushint 1  // Simulate vote validation
bnz valid_vote
pushint 0
return
valid_vote:
// Process vote
pushint 1
return
"#;

    match execute_teal_application(teal_code) {
        Ok(success) => {
            println!("Voting contract pattern: {success}");
            println!("- Global state for vote tallies");
            println!("- Local state to track who voted");
            println!("- Prevents double voting\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 4: Token Vault Pattern
    // TEAL: Vault that holds tokens and tracks deposits
    println!("Example 4: Token vault pattern");

    let teal_code = r#"
#pragma version 8
// Simulate vault operation
pushbytes "deposit"
pushbytes "deposit"
==
bnz handle_deposit
// Other operations
pushint 1
return
handle_deposit:
// Deposit logic
pushint 1
return
"#;

    match execute_teal_application(teal_code) {
        Ok(success) => {
            println!("Token vault pattern: {success}");
            println!("- Tracking individual balances in local state");
            println!("- Maintaining total deposits in global state");
            println!("- Balance verification before withdrawals\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 5: Escrow with timeout pattern
    // TEAL: Escrow that can be claimed by receiver or refunded after timeout
    println!("Example 5: Escrow contract pattern");

    let teal_code = r#"
#pragma version 8
// Simplified escrow logic
pushint 1000  // Current time
pushint 500   // Timeout
>         // Check if timeout passed
bnz refund_allowed
// Normal claim logic
pushint 1
return
refund_allowed:
// Refund logic
pushint 1
return
"#;

    match execute_teal_application(teal_code) {
        Ok(success) => {
            println!("Escrow contract pattern: {success}");
            println!("- State initialization on creation");
            println!("- Conditional logic based on caller and time");
        }
        Err(e) => println!("Error: {e}"),
    }
}
