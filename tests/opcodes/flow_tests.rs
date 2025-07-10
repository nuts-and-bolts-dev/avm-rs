//! Tests for flow control opcodes

use rust_avm::{opcodes::*, types::StackValue};

use crate::common::*;

#[test]
fn test_op_bnz_branch_taken() {
    // Test branch if not zero - branch taken
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // non-zero value
    bytecode.push(OP_BNZ);
    bytecode.extend_from_slice(&0x0001u16.to_be_bytes()); // offset to skip err
    bytecode.push(OP_ERR); // This should be skipped
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bnz_branch_not_taken() {
    // Test branch if not zero - branch not taken
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes()); // zero value
    bytecode.push(OP_BNZ);
    bytecode.extend_from_slice(&0x0001u16.to_be_bytes()); // offset (not taken)
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return
    bytecode.push(OP_ERR); // This would cause error if reached

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bz_branch_taken() {
    // Test branch if zero - branch taken
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes()); // zero value
    bytecode.push(OP_BZ);
    bytecode.extend_from_slice(&0x0001u16.to_be_bytes()); // offset to skip err
    bytecode.push(OP_ERR); // This should be skipped
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bz_branch_not_taken() {
    // Test branch if zero - branch not taken
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes()); // non-zero value
    bytecode.push(OP_BZ);
    bytecode.extend_from_slice(&0x0001u16.to_be_bytes()); // offset (not taken)
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return
    bytecode.push(OP_ERR); // This would cause error if reached

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_b_unconditional() {
    // Test unconditional branch
    let mut bytecode = Vec::new();
    bytecode.push(OP_B);
    bytecode.extend_from_slice(&0x0001u16.to_be_bytes()); // offset to skip err
    bytecode.push(OP_ERR); // This should be skipped
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_b_backward_jump() {
    // Test backward jump (simple loop that increments counter)
    let mut bytecode = Vec::new();

    // Initialize counter to 0
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode.push(OP_STORE); // store at scratch[0]
    bytecode.push(0);

    // Loop start (PC = 13)
    bytecode.push(OP_LOAD); // load counter
    bytecode.push(0);
    bytecode.push(0x81); // pushint 1
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_PLUS); // increment
    bytecode.push(OP_DUP); // duplicate for comparison
    bytecode.push(OP_STORE); // store back
    bytecode.push(0);

    // Check if counter < 3
    bytecode.push(0x81); // pushint 3
    bytecode.extend_from_slice(&3u64.to_be_bytes());
    bytecode.push(OP_LT); // counter < 3?

    // Calculate offset for backward jump
    // We need to jump back to "Loop start"
    let jump_offset = -28i16; // Negative offset to jump back
    bytecode.push(OP_BNZ);
    bytecode.extend_from_slice(&jump_offset.to_be_bytes());

    // After loop, load final counter value
    bytecode.push(OP_LOAD);
    bytecode.push(0);

    // Check it equals 3
    bytecode = with_assert_equals(bytecode, StackValue::Uint(3));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_return() {
    // Test return opcode
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_RETURN); // return with 1 on stack
    bytecode.push(OP_ERR); // This should never execute

    execute_and_check(&bytecode, true).unwrap();

    // Test return with 0
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode.push(OP_RETURN); // return with 0 on stack

    execute_and_check(&bytecode, false).unwrap();
}

#[test]
fn test_op_assert_success() {
    // Test assert with true condition
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // true
    bytecode.push(OP_ASSERT); // assert succeeds
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_assert_failure() {
    // Test assert with false condition
    let mut bytecode = Vec::new();
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes()); // false
    bytecode.push(OP_ASSERT); // assert fails
    bytecode.push(0x81); // This should not execute
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(0x43); // return

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_callsub_retsub() {
    // Test subroutine call and return
    let mut bytecode = Vec::new();

    // Main program
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.push(OP_CALLSUB);
    bytecode.extend_from_slice(&0x000Eu16.to_be_bytes()); // offset to subroutine
    // After return, result should be doubled
    bytecode = with_assert_equals(bytecode, StackValue::Uint(20));
    // Skip over subroutine
    bytecode.push(OP_B);
    bytecode.extend_from_slice(&0x0003u16.to_be_bytes());

    // Subroutine: doubles the value on stack
    bytecode.push(OP_DUP);
    bytecode.push(OP_PLUS);
    bytecode.push(OP_RETSUB);

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_nested_subroutines() {
    // Test nested subroutine calls
    let mut bytecode = Vec::new();

    // Main program
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&5u64.to_be_bytes());
    bytecode.push(OP_CALLSUB);
    bytecode.extend_from_slice(&0x000Eu16.to_be_bytes()); // call sub1
    // Result should be (5 * 2) + 1 = 11
    bytecode = with_assert_equals(bytecode, StackValue::Uint(11));
    // Jump to end
    bytecode.push(OP_B);
    bytecode.extend_from_slice(&0x0011u16.to_be_bytes());

    // Subroutine 1: doubles then adds 1
    bytecode.push(OP_CALLSUB);
    bytecode.extend_from_slice(&0x000Bu16.to_be_bytes()); // call sub2
    bytecode.push(0x81); // pushint 1
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_PLUS);
    bytecode.push(OP_RETSUB);

    // Subroutine 2: doubles the value
    bytecode.push(OP_DUP);
    bytecode.push(OP_PLUS);
    bytecode.push(OP_RETSUB);

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_conditional_logic_complex() {
    // Test complex conditional logic: if (a > b) { result = a - b } else { result = b - a }
    let mut bytecode = Vec::new();

    // Test case 1: a=10, b=5
    bytecode.push(0x81); // pushint a
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.push(0x81); // pushint b
    bytecode.extend_from_slice(&5u64.to_be_bytes());

    // Duplicate for comparison
    bytecode.push(OP_DUP2); // Stack: [10, 5, 5, 5] (current implementation)
    bytecode.push(OP_GT); // Stack: [10, 5, 0] (5 > 5 = false)

    // Branch if a > b
    bytecode.push(OP_BNZ);
    bytecode.extend_from_slice(&0x0005u16.to_be_bytes()); // jump to a-b

    // Else: b - a
    bytecode.push(OP_SWAP);
    bytecode.push(OP_MINUS);
    bytecode.push(OP_B); // jump to end
    bytecode.extend_from_slice(&0x0001u16.to_be_bytes());

    // If: a - b
    bytecode.push(OP_MINUS);

    // Result should be 5
    bytecode = with_assert_equals(bytecode, StackValue::Uint(5));

    execute_and_check(&bytecode, true).unwrap();
}
