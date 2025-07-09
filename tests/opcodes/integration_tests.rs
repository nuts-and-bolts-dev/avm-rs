//! Integration tests for multi-opcode scenarios

use rust_avm::{
    opcodes::*,
    types::{RunMode, StackValue},
};

use crate::common::*;

#[test]
fn test_factorial_computation() {
    // Compute factorial of 5 using subroutines and loops
    let mut bytecode = Vec::new();

    // Main program: compute 5!
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&5u64.to_be_bytes());
    bytecode.push(OP_CALLSUB);
    bytecode.extend_from_slice(&0x0006u16.to_be_bytes()); // call factorial
    bytecode = with_assert_equals(bytecode, StackValue::Uint(120)); // 5! = 120
    // Jump to end
    bytecode.push(OP_B);
    bytecode.extend_from_slice(&0x0030u16.to_be_bytes());

    // Factorial subroutine
    bytecode.push(OP_DUP); // n n
    bytecode.push(OP_INTC_1); // n n 1
    bytecode.push(OP_LE); // n (n <= 1)
    bytecode.push(OP_BNZ); // n
    bytecode.extend_from_slice(&0x001Cu16.to_be_bytes()); // jump to base case

    // Recursive case: n * factorial(n-1)
    bytecode.push(OP_DUP); // n n
    bytecode.push(OP_INTC_1); // n n 1
    bytecode.push(OP_MINUS); // n (n-1)
    bytecode.push(OP_CALLSUB);
    bytecode.extend_from_slice(&0xFFE6u16.to_be_bytes()); // recursive call (negative offset)
    bytecode.push(OP_MUL); // n * factorial(n-1)
    bytecode.push(OP_RETSUB);

    // Base case: return 1
    bytecode.push(OP_POP); // remove n
    bytecode.push(OP_INTC_1); // return 1
    bytecode.push(OP_RETSUB);

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_fibonacci_iterative() {
    // Compute 10th Fibonacci number iteratively
    let mut bytecode = Vec::new();

    // Initialize: fib(0) = 0, fib(1) = 1
    bytecode.push(OP_INTC_0); // a = 0
    bytecode.push(OP_STORE);
    bytecode.push(0);
    bytecode.push(OP_INTC_1); // b = 1
    bytecode.push(OP_STORE);
    bytecode.push(1);
    bytecode.push(OP_INTC_2); // i = 2
    bytecode.push(OP_STORE);
    bytecode.push(2);

    // Loop until i == 10
    // Loop start
    bytecode.push(OP_LOAD); // i
    bytecode.push(2);
    bytecode.push(0x81); // pushint 10
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.push(OP_LT); // i < 10
    bytecode.push(OP_BZ); // exit if i >= 10
    bytecode.extend_from_slice(&0x0028u16.to_be_bytes());

    // temp = a + b
    bytecode.push(OP_LOAD); // a
    bytecode.push(0);
    bytecode.push(OP_LOAD); // b
    bytecode.push(1);
    bytecode.push(OP_PLUS); // temp = a + b

    // a = b
    bytecode.push(OP_LOAD); // b
    bytecode.push(1);
    bytecode.push(OP_STORE); // a = b
    bytecode.push(0);

    // b = temp
    bytecode.push(OP_STORE); // b = temp
    bytecode.push(1);

    // i++
    bytecode.push(OP_LOAD); // i
    bytecode.push(2);
    bytecode.push(OP_INTC_1); // 1
    bytecode.push(OP_PLUS); // i + 1
    bytecode.push(OP_STORE); // i++
    bytecode.push(2);

    // Jump back to loop start
    bytecode.push(OP_B);
    bytecode.extend_from_slice(&0xFFDEu16.to_be_bytes()); // negative offset

    // Return fib(10) = b
    bytecode.push(OP_LOAD);
    bytecode.push(1);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(55)); // fib(10) = 55

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_string_manipulation_pipeline() {
    // Complex string manipulation: concatenate, hash, convert to int
    let mut bytecode = Vec::new();

    // Create "Hello" + "World"
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(5);
    bytecode.extend_from_slice(b"Hello");
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(5);
    bytecode.extend_from_slice(b"World");
    bytecode.push(OP_CONCAT);

    // Extract substring "llo"
    bytecode.push(OP_DUP);
    bytecode.push(OP_SUBSTRING);
    bytecode.push(2); // start
    bytecode.push(3); // length

    // Should be "llo"
    bytecode.push(0x80); // pushbytes
    bytecode.push(3);
    bytecode.extend_from_slice(b"llo");
    bytecode.push(OP_EQ);
    bytecode.push(OP_ASSERT); // Assert substring is correct

    // Hash the full string
    bytecode.push(OP_SHA256);

    // Take first 8 bytes and convert to int
    bytecode.push(OP_SUBSTRING);
    bytecode.push(0); // start
    bytecode.push(8); // length
    bytecode.push(OP_BTOI);

    // Result should be non-zero
    bytecode.push(OP_INTC_0);
    bytecode.push(OP_GT);
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_stack_stress_test() {
    // Stress test stack operations with many values
    let mut bytecode = Vec::new();

    // Push 100 consecutive integers
    for i in 1..=100 {
        bytecode.push(0x81); // pushint
        bytecode.extend_from_slice(&(i as u64).to_be_bytes());
    }

    // Sum all values using a loop
    bytecode.push(OP_INTC_0); // sum = 0
    bytecode.push(OP_STORE);
    bytecode.push(0);

    // Loop to add all values
    for _ in 0..100 {
        bytecode.push(OP_LOAD); // sum
        bytecode.push(0);
        bytecode.push(OP_PLUS); // sum + value
        bytecode.push(OP_STORE); // store new sum
        bytecode.push(0);
    }

    // Load final sum
    bytecode.push(OP_LOAD);
    bytecode.push(0);

    // Sum of 1..100 = 5050
    bytecode = with_assert_equals(bytecode, StackValue::Uint(5050));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_cryptographic_verification_flow() {
    // Simulate a complex cryptographic verification workflow
    let mut bytecode = Vec::new();

    // Create test data
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(11);
    bytecode.extend_from_slice(b"test_message");

    // Duplicate for multiple hash checks
    bytecode.push(OP_DUP);
    bytecode.push(OP_DUP);

    // SHA256 hash
    bytecode.push(OP_SHA256);
    bytecode.push(OP_LEN);
    bytecode.push(0x81); // pushint 32
    bytecode.extend_from_slice(&32u64.to_be_bytes());
    bytecode.push(OP_EQ);
    bytecode.push(OP_ASSERT); // Assert SHA256 produces 32 bytes

    // Keccak256 hash
    bytecode.push(OP_KECCAK256);
    bytecode.push(OP_LEN);
    bytecode.push(0x81); // pushint 32
    bytecode.extend_from_slice(&32u64.to_be_bytes());
    bytecode.push(OP_EQ);
    bytecode.push(OP_ASSERT); // Assert Keccak256 produces 32 bytes

    // Verify they produce different results
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(11);
    bytecode.extend_from_slice(b"test_message");
    bytecode.push(OP_DUP);
    bytecode.push(OP_SHA256);
    bytecode.push(OP_SWAP);
    bytecode.push(OP_KECCAK256);
    bytecode.push(OP_NE); // Should be different
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_conditional_state_access() {
    // Test conditional state access based on transaction fields
    let mut ledger = setup_mock_ledger();
    let mut tx = test_transaction();
    tx.amount = Some(50000); // Modify amount for test
    ledger.clear_transactions();
    ledger.add_transaction(tx);
    ledger.set_current_transaction_index(0);

    let mut bytecode = Vec::new();

    // Get transaction amount
    bytecode.push(OP_TXN);
    bytecode.push(8); // Amount field

    // If amount > 25000, check global state
    bytecode.push(0x81); // pushint 25000
    bytecode.extend_from_slice(&25000u64.to_be_bytes());
    bytecode.push(OP_GT);
    bytecode.push(OP_BNZ);
    bytecode.extend_from_slice(&0x0014u16.to_be_bytes()); // jump to state check

    // Amount <= 25000: return false
    bytecode.push(OP_INTC_0);
    bytecode.push(0x43); // return

    // Amount > 25000: check if counter > 40
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(7);
    bytecode.extend_from_slice(b"counter");
    bytecode.push(OP_APP_GLOBAL_GET);
    bytecode.push(OP_POP); // Remove exists flag
    bytecode.push(0x81); // pushint 40
    bytecode.extend_from_slice(&40u64.to_be_bytes());
    bytecode.push(OP_GT); // counter > 40
    bytecode.push(0x43); // return

    let vm = setup_vm();
    let config = test_config().with_run_mode(RunMode::Application);
    let result = vm.execute(&bytecode, config, &ledger).unwrap();
    assert!(result); // 50000 > 25000 and 42 > 40
}

#[test]
fn test_multi_asset_balance_check() {
    // Check balances across multiple assets and accounts
    let mut bytecode = Vec::new();

    // Check balance of account 1
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&[1u8; 32]);
    bytecode.push(OP_BALANCE);

    // Check balance of account 2
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&[2u8; 32]);
    bytecode.push(OP_BALANCE);

    // Add them together
    bytecode.push(OP_PLUS);

    // Should be 1,500,000 (1M + 500K)
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1_500_000));

    let vm = setup_vm();
    let ledger = setup_mock_ledger();
    let config = test_config().with_run_mode(RunMode::Application);
    let result = vm.execute(&bytecode, config, &ledger).unwrap();
    assert!(result);
}
