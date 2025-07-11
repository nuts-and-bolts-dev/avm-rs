//! Integration tests for multi-opcode scenarios
//!
//! Status: 6 integration tests remain failing with complex opcode combination issues
//! Most core opcode functionality is now working (95% test pass rate achieved)
//!
//! **FIXED Categories:**
//! ✅ 2. **State Operation Return Values** - Tests updated to handle 2-value returns
//! ✅ 3. **Parameter Parsing Order** - PC advancement order fixed in transaction opcodes  
//! ✅ 4. **Stack Operation Bugs** - DUP2 correctly implements TEAL spec
//! ✅ 5. **Transaction Group Issues** - Group sizes properly configured in tests
//!
//! **Remaining Issue Categories:**
//! 1. **Complex Branching Logic** (OP_BNZ, OP_BZ, OP_B, OP_CALLSUB)
//!    - Jump offset calculations in complex loops may have edge cases
//!    - Negative offsets in iterative scenarios need verification
//!    - Subroutine call/return logic in complex scenarios
//!
//! **Remaining Work:**
//! 1. Complex branching logic edge cases in multi-opcode scenarios
//! 2. Advanced cryptographic operations (SHA256, Keccak256, VRF, etc.)
//! 3. String manipulation with hash operations
//! 4. Large stack stress test scenarios

use rust_avm::{
    opcodes::*,
    types::{RunMode, StackValue},
};

use crate::common::*;

/// TODO: Test fails with "err opcode executed" - complex branching logic errors
/// Root causes:
/// 1. Branching offsets may be calculated incorrectly in OP_BNZ/OP_BZ/OP_B operations
/// 2. Subroutine call/return logic (OP_CALLSUB/OP_RETSUB) may have stack management issues
/// 3. Recursive calls with negative offsets may cause execution path errors
/// 4. Stack underflow may occur during complex arithmetic operations (5-10=underflow)
///    This test combines multiple problematic opcodes: branching, subroutines, arithmetic
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

/// TODO: Test fails with "err opcode executed" - iterative loop with branching errors
/// Root causes:
/// 1. OP_BZ branching logic may calculate wrong jump targets causing invalid execution paths
/// 2. OP_B negative offset jumps may cause PC underflow or wrong branch targets  
/// 3. Scratch space operations (OP_LOAD/OP_STORE) may have parameter parsing issues
/// 4. Complex loop with multiple stack operations may trigger arithmetic underflow
///    This test combines: branching, scratch space, loops, arithmetic - all problematic areas
#[test]
fn test_fibonacci_iterative() {
    // Compute 10th Fibonacci number iteratively
    let mut bytecode = Vec::new();

    // Set up integer constants block: [0, 1, 2]
    bytecode.push(OP_INTCBLOCK);
    bytecode.push(3); // 3 constants
    bytecode.extend_from_slice(&0u64.to_be_bytes()); // constant 0
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // constant 1
    bytecode.extend_from_slice(&2u64.to_be_bytes()); // constant 2

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

/// TODO: Test fails with "err opcode executed" - string operations and hashing errors
/// Root causes:
/// 1. OP_SUBSTRING immediate parameter parsing may be incorrect (reading parameters in wrong order)
/// 2. OP_SHA256/OP_KECCAK256 may not be implemented or have incorrect output format
/// 3. OP_BTOI conversion may fail on hash outputs or have length validation issues
/// 4. OP_ASSERT may trigger due to incorrect string comparison results
///    This test combines: string ops, hashing, type conversion - multiple unimplemented areas
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

/// TODO: Test fails with "err opcode executed" - large stack operations stress test
/// Root causes:
/// 1. Stack may have size limits that cause overflow with 100+ values
/// 2. Scratch space operations (OP_LOAD/OP_STORE) may fail with many iterations
/// 3. Arithmetic operations may cause integer overflow when summing 1..100
/// 4. VM may have execution limits or cost budget exceeded with 100+ operations
///    This test stresses: stack size, scratch space, arithmetic, execution limits
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

/// TODO: Test fails with "err opcode executed" - cryptographic operations not implemented
/// Root causes:
/// 1. OP_SHA256 and OP_KECCAK256 may not be fully implemented or return wrong format
/// 2. Hash length validation may fail causing OP_ASSERT to trigger
/// 3. Hash comparison logic (OP_NE) may fail due to incorrect hash outputs
/// 4. Multiple OP_DUP operations may cause stack management issues
///    This test requires: working hash functions, length checks, stack management
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

/// TODO: Test fails due to combination of transaction field access and state operations
/// Root causes:
/// 1. OP_APP_GLOBAL_GET returns 2 values (value + exists flag) but test expects 1
/// 2. Transaction field access (OP_TXN) may have parameter parsing issues
/// 3. Branching logic (OP_BNZ) may calculate wrong offsets causing execution errors
/// 4. Combination of transaction fields + state operations exposes multiple bugs
///    This test combines: transaction fields, state access, branching - all problematic
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
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result); // 50000 > 25000 and 42 > 40
}

/// TODO: Test may fail due to balance operation implementation issues
/// Root causes:
/// 1. OP_BALANCE operations may not be fully implemented in mock ledger
/// 2. Multiple balance checks may expose ledger state management bugs
/// 3. Large number arithmetic (1,500,000) may trigger overflow checks
/// 4. Mock ledger balance setup may not match expected test values
///    This test requires: working balance operations, proper ledger state, arithmetic
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
    let mut ledger = setup_mock_ledger();
    let config = test_config().with_run_mode(RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result);
}
