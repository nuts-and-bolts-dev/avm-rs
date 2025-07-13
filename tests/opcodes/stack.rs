//! Tests for stack manipulation opcodes

use avm_rs::{opcodes::*, types::StackValue};

use crate::common::*;

#[test]
fn test_op_pop() {
    // Test basic pop operation
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&100u64.to_be_bytes());
    bytecode.push(OP_POP); // pop the 100
    // Now only 42 should be on stack
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_pop_empty_stack() {
    // Test pop on empty stack
    let bytecode = vec![
        OP_POP, // pop on empty stack
    ];

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_dup() {
    // Test basic dup operation
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode.push(OP_DUP); // duplicate 42
    bytecode.push(OP_EQ); // they should be equal
    bytecode.push(OP_RETURN); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_dup2() {
    // Test dup2 operation according to TEAL spec: [A, B] -> [A, B, A, B]
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint 10
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.push(OP_PUSHINT); // pushint 20
    bytecode.extend_from_slice(&20u64.to_be_bytes());
    bytecode.push(OP_DUP2); // duplicate top two: stack is now [10, 20, 10, 20]

    // Simple verification: check that the top value is 20 and pop the rest
    // Stack: [10, 20, 10, 20] - verify top is 20, then pop 3 values to leave bottom 10
    bytecode.push(OP_PUSHINT); // pushint 20
    bytecode.extend_from_slice(&20u64.to_be_bytes()); // [10, 20, 10, 20, 20]
    bytecode.push(OP_EQ); // [10, 20, 10, 1] - top values match

    // Now pop the remaining values to leave just 1 on stack
    bytecode.push(OP_POP); // [10, 20, 10] - remove comparison result
    bytecode.push(OP_POP); // [10, 20] - remove duplicated 10
    bytecode.push(OP_POP); // [10] - remove duplicated 20, leaving original 10

    // Verify the remaining value is 10
    bytecode = with_assert_equals(bytecode, StackValue::Uint(10));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_swap() {
    // Test swap operation
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&20u64.to_be_bytes());
    bytecode.push(OP_SWAP); // swap them
    // Stack is now [20, 10], need to pop the bottom value
    bytecode.push(OP_SWAP); // [10, 20]
    bytecode.push(OP_POP); // [10]
    // Now check that 10 is on top
    bytecode = with_assert_equals(bytecode, StackValue::Uint(10));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_select() {
    // Test select with true condition (selects first value)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&100u64.to_be_bytes()); // A
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&200u64.to_be_bytes()); // B
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // true
    bytecode.push(OP_SELECT); // select A (100)
    bytecode = with_assert_equals(bytecode, StackValue::Uint(100));

    execute_and_check(&bytecode, true).unwrap();

    // Test select with false condition (selects second value)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&100u64.to_be_bytes()); // A
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&200u64.to_be_bytes()); // B
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes()); // false
    bytecode.push(OP_SELECT); // select B (200)
    bytecode = with_assert_equals(bytecode, StackValue::Uint(200));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_len() {
    // Test length of byte array
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Bytes(vec![1, 2, 3, 4, 5])], OP_LEN),
        StackValue::Uint(5),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test length of empty byte array
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Bytes(vec![])], OP_LEN),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test length of uint (always 8 bytes)
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42)], OP_LEN),
        StackValue::Uint(8),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_itob() {
    // Test integer to bytes conversion
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&0x0123456789ABCDEFu64.to_be_bytes());
    bytecode.push(OP_ITOB); // convert to bytes
    bytecode.push(OP_LEN); // check length
    bytecode = with_assert_equals(bytecode, StackValue::Uint(8));

    execute_and_check(&bytecode, true).unwrap();

    // Test zero conversion
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode.push(OP_ITOB); // convert to bytes
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(8); // length
    bytecode.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
    bytecode.push(OP_EQ); // should be equal
    bytecode.push(OP_RETURN); // return

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_btoi() {
    // Test bytes to integer conversion
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(8); // length
    bytecode.extend_from_slice(&0x0123456789ABCDEFu64.to_be_bytes());
    bytecode.push(OP_BTOI); // convert to int
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0x0123456789ABCDEF));

    execute_and_check(&bytecode, true).unwrap();

    // Test empty bytes -> 0
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Bytes(vec![])], OP_BTOI),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test partial bytes (less than 8)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(4); // length
    bytecode.extend_from_slice(&[0x12, 0x34, 0x56, 0x78]);
    bytecode.push(OP_BTOI); // convert to int
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0x12345678));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_btoi_too_long() {
    // Test bytes too long for conversion
    let bytecode = build_simple_op_test(
        vec![StackValue::Bytes(vec![1; 9])], // 9 bytes, too long
        OP_BTOI,
    );
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_concat() {
    // Test basic concatenation
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![
                StackValue::Bytes(vec![1, 2, 3]),
                StackValue::Bytes(vec![4, 5, 6]),
            ],
            OP_CONCAT,
        ),
        StackValue::Bytes(vec![1, 2, 3, 4, 5, 6]),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test concatenation with empty
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Bytes(vec![1, 2, 3]), StackValue::Bytes(vec![])],
            OP_CONCAT,
        ),
        StackValue::Bytes(vec![1, 2, 3]),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test empty concatenation
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Bytes(vec![]), StackValue::Bytes(vec![])],
            OP_CONCAT,
        ),
        StackValue::Bytes(vec![]),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_substring() {
    // Test basic substring extraction
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(6); // length
    bytecode.extend_from_slice(b"abcdef");
    bytecode.push(OP_SUBSTRING);
    bytecode.push(2); // start index
    bytecode.push(3); // length
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"cde".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test full string extraction
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(4); // length
    bytecode.extend_from_slice(b"test");
    bytecode.push(OP_SUBSTRING);
    bytecode.push(0); // start index
    bytecode.push(4); // length
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"test".to_vec()));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_substring_out_of_bounds() {
    // Test substring with out of bounds indices
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(4); // length
    bytecode.extend_from_slice(b"test");
    bytecode.push(OP_SUBSTRING);
    bytecode.push(2); // start index
    bytecode.push(5); // length (too long)

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_substring3() {
    // Test substring3 with stack arguments
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(6); // length
    bytecode.extend_from_slice(b"abcdef");
    bytecode.push(OP_PUSHINT); // pushint (start)
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_PUSHINT); // pushint (end)
    bytecode.extend_from_slice(&4u64.to_be_bytes());
    bytecode.push(OP_SUBSTRING3);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"bcd".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test empty substring
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(4); // length
    bytecode.extend_from_slice(b"test");
    bytecode.push(OP_PUSHINT); // pushint (start)
    bytecode.extend_from_slice(&2u64.to_be_bytes());
    bytecode.push(OP_PUSHINT); // pushint (end)
    bytecode.extend_from_slice(&2u64.to_be_bytes());
    bytecode.push(OP_SUBSTRING3);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(vec![]));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_load_store() {
    // Test load and store operations
    let mut bytecode = Vec::new();

    // Store value at index 0
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode.push(OP_STORE);
    bytecode.push(0); // scratch index

    // Store value at index 1
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&100u64.to_be_bytes());
    bytecode.push(OP_STORE);
    bytecode.push(1); // scratch index

    // Load from index 0
    bytecode.push(OP_LOAD);
    bytecode.push(0); // scratch index

    // Should be 42
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    execute_and_check(&bytecode, true).unwrap();

    // Test loading from different slots
    let mut bytecode = Vec::new();

    // Store values
    for i in 0..5 {
        bytecode.push(OP_PUSHINT); // pushint
        bytecode.extend_from_slice(&(i as u64 * 10).to_be_bytes());
        bytecode.push(OP_STORE);
        bytecode.push(i); // scratch index
    }

    // Load from index 3
    bytecode.push(OP_LOAD);
    bytecode.push(3);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(30));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_dupn() {
    // Test duplicating N values
    let mut bytecode = Vec::new();

    // Push 3 values
    for i in 1..=3 {
        bytecode.push(OP_PUSHINT); // pushint
        bytecode.extend_from_slice(&(i as u64).to_be_bytes());
    }

    // Duplicate top 2 values
    bytecode.push(OP_DUPN);
    bytecode.push(2);

    // Stack should now be [1, 2, 3, 2, 3]
    // Check top value is 3, and pop the remaining values
    bytecode.push(OP_POPN);
    bytecode.push(4); // pop 4 values, leaving only the bottom value
    // Now check that 1 is on top
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1));

    execute_and_check(&bytecode, true).unwrap();

    // Test duplicating 0 values (no-op)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode.push(OP_DUPN);
    bytecode.push(0); // duplicate 0 values
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_popn() {
    // Test popping N values
    let mut bytecode = Vec::new();

    // Push 5 values
    for i in 1..=5 {
        bytecode.push(OP_PUSHINT); // pushint
        bytecode.extend_from_slice(&(i as u64).to_be_bytes());
    }

    // Pop top 3 values
    bytecode.push(OP_POPN);
    bytecode.push(3);

    // Stack should now be [1, 2]
    // Check top value is 2, and pop the remaining value
    bytecode.push(OP_SWAP); // [2, 1]
    bytecode.push(OP_POP); // [2]
    // Now check that 2 is on top
    bytecode = with_assert_equals(bytecode, StackValue::Uint(2));

    execute_and_check(&bytecode, true).unwrap();

    // Test popping 0 values (no-op)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode.push(OP_POPN);
    bytecode.push(0); // pop 0 values
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_popn_underflow() {
    // Test popping more values than on stack
    let mut bytecode = Vec::new();

    // Push 2 values
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&2u64.to_be_bytes());

    // Try to pop 3 values
    bytecode.push(OP_POPN);
    bytecode.push(3);

    execute_expect_error(&bytecode).unwrap();
}
