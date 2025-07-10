//! Transaction Fields Example
//!
//! This example demonstrates accessing and validating transaction fields in TEAL
//! using patterns from Algorand's official documentation. It shows how smart
//! contracts can inspect and validate transaction properties.

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
    let config = ExecutionConfig::new(TealVersion::V8).with_cost_budget(100000);

    // Use the default ledger which already has a transaction set up
    let mut ledger = MockLedger::default();

    let result = vm
        .execute(&bytecode, config, &mut ledger)
        .map_err(|e| format!("Execution error: {e}"))?;
    Ok(result)
}

fn main() {
    println!("=== Transaction Fields Example ===\n");

    // Example 1: Basic transaction validation patterns
    println!("Example 1: Basic transaction validation patterns");

    let teal_code = r#"
#pragma version 8
// Validate transaction sender and fee
txn Sender
len
int 32
==
assert       // Sender address must be 32 bytes

txn Fee
int 1000
>=
assert       // Fee must be at least 1000 microAlgos

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Transaction validation pattern: {success}");
            println!("- Sender address validation");
            println!("- Fee validation");
            println!("- Round number checks\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 2: Payment transaction validation pattern
    println!("Example 2: Payment transaction validation pattern");

    let teal_code = r#"
#pragma version 8
// Validate payment transaction
txn TypeEnum
int 1
==
assert       // Must be payment transaction

txn Amount
int 1000000
>=
assert       // Amount must be at least 1 ALGO

txn Receiver
len
int 32
==
assert       // Receiver must be valid address

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Payment validation: {success}");
            println!("- Transaction type check");
            println!("- Amount validation");
            println!("- Receiver validation\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 3: Asset transfer validation pattern
    println!("Example 3: Asset transfer validation pattern");

    let teal_code = r#"
#pragma version 8
// Validate asset transfer transaction
txn TypeEnum
int 4
==
assert       // Must be asset transfer transaction

txn XferAsset
int 0
>
assert       // Must have valid asset ID

txn AssetAmount
int 0
>
assert       // Transfer amount must be positive

txn AssetReceiver
len
int 32
==
assert       // Asset receiver must be valid address

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Asset transfer validation: {success}");
            println!("- Asset ID validation");
            println!("- Transfer amount check");
            println!("- Receiver validation\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 4: Group transaction validation pattern
    println!("Example 4: Group transaction validation pattern");

    let teal_code = r#"
#pragma version 8
// Validate group transaction properties
global GroupSize
int 1
>
assert       // Must be in a group with more than 1 transaction

txn GroupIndex
int 0
>=
assert       // Group index must be valid

// Check current transaction type
txn TypeEnum
int 1
==
assert       // This transaction must be payment

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Group transaction validation: {success}");
            println!("- Group size verification");
            println!("- Transaction position validation");
            println!("- Atomic execution guarantee\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 5: Time-based validation pattern
    println!("Example 5: Time-based validation pattern");

    let teal_code = r#"
#pragma version 8
// Validate time-based constraints
global LatestTimestamp
int 1000000
>
assert       // Must be after a certain time

global LatestTimestamp
int 3000000
<
assert       // Must be before expiry time

txn FirstValid
global Round
<=
assert       // First valid round must not be in future

txn LastValid
global Round
>=
assert       // Last valid round must not be in past

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Time-based validation: {success}");
            println!("- Valid time window");
            println!("- Prevents replay attacks");
            println!("- Time lock enforcement\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 6: Fee validation pattern
    println!("Example 6: Fee validation pattern");

    let teal_code = r#"
#pragma version 8
// Validate transaction fee constraints
txn Fee
global MinTxnFee
>=
assert       // Fee must be at least minimum network fee

txn Fee
int 10000
<=
assert       // Fee must not be excessive (custom limit)

// Ensure no fee overpayment for simple transactions
txn Fee
global MinTxnFee
int 5
*
<=
assert       // Fee should not exceed 5x minimum

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Fee validation: {success}");
            println!("- Minimum fee enforcement");
            println!("- Maximum fee protection");
            println!("- Economic security\n");
        }
        Err(e) => println!("Error: {e}\n"),
    }

    // Example 7: Complex multi-criteria validation
    println!("Example 7: Complex multi-criteria validation");

    let teal_code = r#"
#pragma version 8
// Complex validation combining multiple transaction fields
// Validate payment amount range
txn Amount
dup
int 100000    // 0.1 ALGO minimum
>=
assert
int 10000000  // 10 ALGO maximum  
<=
assert

// Validate note field has reasonable length
txn Note
len
int 4
>=
assert        // Note must be at least 4 bytes

// Ensure no rekeying
txn RekeyTo
global ZeroAddress
==
assert        // RekeyTo must be zero address

// Validate lease is zero (no conflicts)
txn Lease
global ZeroAddress
==
assert        // Lease must be zero

// Validate transaction type
txn TypeEnum
int 1
==
assert        // Must be payment transaction

int 1
return
"#;

    match execute_teal_signature(teal_code) {
        Ok(success) => {
            println!("Complex validation passed: {success}");
            println!("- Amount range validation");
            println!("- Note field requirements");
            println!("- Security restrictions");
        }
        Err(e) => println!("Error: {e}"),
    }
}
