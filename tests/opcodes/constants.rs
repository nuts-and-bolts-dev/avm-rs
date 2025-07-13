//! Tests for constant loading opcodes

use avm_rs::{opcodes::*, types::StackValue};

use crate::common::*;

#[test]
fn test_op_pushint() {
    // Test pushing various integer values
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    execute_and_check(&bytecode, true).unwrap();

    // Test pushing zero
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0));

    execute_and_check(&bytecode, true).unwrap();

    // Test pushing max value
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&u64::MAX.to_be_bytes());
    bytecode = with_assert_equals(bytecode, StackValue::Uint(u64::MAX));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_pushbytes() {
    // Test pushing byte arrays
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"hello".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test empty bytes
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // length
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(vec![]));

    execute_and_check(&bytecode, true).unwrap();

    // Test max length (255 bytes)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(255); // max length
    let data = vec![0xAB; 255];
    bytecode.extend_from_slice(&data);
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(255));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_pushints() {
    // Test pushing multiple integers
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINTS);
    bytecode.push(3); // count
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.extend_from_slice(&20u64.to_be_bytes());
    bytecode.extend_from_slice(&30u64.to_be_bytes());

    // Stack should have [10, 20, 30] with 30 on top
    // Pop the bottom two values
    bytecode.push(OP_SWAP); // [10, 30, 20]
    bytecode.push(OP_POP); // [10, 30]
    bytecode.push(OP_SWAP); // [30, 10]
    bytecode.push(OP_POP); // [30]
    bytecode = with_assert_equals(bytecode, StackValue::Uint(30));

    execute_and_check(&bytecode, true).unwrap();

    // Test pushing zero integers (no-op)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&42u64.to_be_bytes());
    bytecode.push(OP_PUSHINTS);
    bytecode.push(0); // count = 0
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_pushbytess() {
    // Test pushing multiple byte arrays
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTESS);
    bytecode.push(2); // count
    bytecode.push(3); // length of first
    bytecode.extend_from_slice(b"foo");
    bytecode.push(3); // length of second
    bytecode.extend_from_slice(b"bar");

    // Stack should have [foo, bar] with bar on top
    // Pop the bottom value
    bytecode.push(OP_SWAP); // [bar, foo]
    bytecode.push(OP_POP); // [bar]
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"bar".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test with mixed lengths
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTESS);
    bytecode.push(3); // count
    bytecode.push(0); // empty
    bytecode.push(1); // single byte
    bytecode.push(0xFF);
    bytecode.push(5); // longer
    bytecode.extend_from_slice(b"hello");

    // Stack has 3 values, need to pop bottom 2
    bytecode.push(OP_SWAP); // swap top 2
    bytecode.push(OP_POP); // pop one
    bytecode.push(OP_SWAP); // swap again
    bytecode.push(OP_POP); // pop another
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"hello".to_vec()));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_intc_shortcuts() {
    use avm_rs::varuint::encode_varuint;

    // Set up integer constant block first
    let mut bytecode = Vec::new();
    bytecode.push(OP_INTCBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(100)); // constant 0
    bytecode.extend_from_slice(&encode_varuint(200)); // constant 1
    bytecode.extend_from_slice(&encode_varuint(300)); // constant 2
    bytecode.extend_from_slice(&encode_varuint(400)); // constant 3

    // Test intc_0
    bytecode.push(OP_INTC_0);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(100));

    execute_and_check(&bytecode, true).unwrap();

    // Test intc_1
    let mut bytecode = Vec::new();
    bytecode.push(OP_INTCBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(100)); // constant 0
    bytecode.extend_from_slice(&encode_varuint(200)); // constant 1
    bytecode.extend_from_slice(&encode_varuint(300)); // constant 2
    bytecode.extend_from_slice(&encode_varuint(400)); // constant 3

    bytecode.push(OP_INTC_1);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(200));

    execute_and_check(&bytecode, true).unwrap();

    // Test intc_2
    let mut bytecode = Vec::new();
    bytecode.push(OP_INTCBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(100)); // constant 0
    bytecode.extend_from_slice(&encode_varuint(200)); // constant 1
    bytecode.extend_from_slice(&encode_varuint(300)); // constant 2
    bytecode.extend_from_slice(&encode_varuint(400)); // constant 3

    bytecode.push(OP_INTC_2);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(300));

    execute_and_check(&bytecode, true).unwrap();

    // Test intc_3
    let mut bytecode = Vec::new();
    bytecode.push(OP_INTCBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(100)); // constant 0
    bytecode.extend_from_slice(&encode_varuint(200)); // constant 1
    bytecode.extend_from_slice(&encode_varuint(300)); // constant 2
    bytecode.extend_from_slice(&encode_varuint(400)); // constant 3

    bytecode.push(OP_INTC_3);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(400));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bytec_shortcuts() {
    use avm_rs::varuint::encode_varuint;

    // Set up byte constant block first
    let mut bytecode = Vec::new();
    bytecode.push(OP_BYTECBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "foo"
    bytecode.extend_from_slice(b"foo");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "bar"
    bytecode.extend_from_slice(b"bar");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "baz"
    bytecode.extend_from_slice(b"baz");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "qux"
    bytecode.extend_from_slice(b"qux");

    // Test bytec_0
    bytecode.push(OP_BYTEC_0);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"foo".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test bytec_1
    let mut bytecode = Vec::new();
    bytecode.push(OP_BYTECBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "foo"
    bytecode.extend_from_slice(b"foo");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "bar"
    bytecode.extend_from_slice(b"bar");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "baz"
    bytecode.extend_from_slice(b"baz");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "qux"
    bytecode.extend_from_slice(b"qux");

    bytecode.push(OP_BYTEC_1);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"bar".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test bytec_2
    let mut bytecode = Vec::new();
    bytecode.push(OP_BYTECBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "foo"
    bytecode.extend_from_slice(b"foo");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "bar"
    bytecode.extend_from_slice(b"bar");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "baz"
    bytecode.extend_from_slice(b"baz");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "qux"
    bytecode.extend_from_slice(b"qux");

    bytecode.push(OP_BYTEC_2);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"baz".to_vec()));

    execute_and_check(&bytecode, true).unwrap();

    // Test bytec_3
    let mut bytecode = Vec::new();
    bytecode.push(OP_BYTECBLOCK);
    bytecode.extend_from_slice(&encode_varuint(4)); // count
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "foo"
    bytecode.extend_from_slice(b"foo");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "bar"
    bytecode.extend_from_slice(b"bar");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "baz"
    bytecode.extend_from_slice(b"baz");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "qux"
    bytecode.extend_from_slice(b"qux");

    bytecode.push(OP_BYTEC_3);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"qux".to_vec()));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bzero() {
    // Test creating zero-filled byte arrays
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&10u64.to_be_bytes());
    bytecode.push(OP_BZERO);
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(10));

    execute_and_check(&bytecode, true).unwrap();

    // Test all bytes are zero
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&5u64.to_be_bytes());
    bytecode.push(OP_BZERO);
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(5);
    bytecode.extend_from_slice(&[0, 0, 0, 0, 0]);
    bytecode.push(OP_EQ);
    bytecode.push(OP_RETURN); // return

    execute_and_check(&bytecode, true).unwrap();

    // Test empty array
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode.push(OP_BZERO);
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(vec![]));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bzero_too_large() {
    // Test bzero with size > 4096
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&4097u64.to_be_bytes());
    bytecode.push(OP_BZERO);

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_constant_blocks() {
    use avm_rs::varuint::encode_varuint;

    // Test intcblock with varuint encoding
    let mut bytecode = Vec::new();
    bytecode.push(OP_INTCBLOCK);
    bytecode.extend_from_slice(&encode_varuint(3)); // count
    bytecode.extend_from_slice(&encode_varuint(100)); // constant 0
    bytecode.extend_from_slice(&encode_varuint(200)); // constant 1
    bytecode.extend_from_slice(&encode_varuint(300)); // constant 2

    // Now intc returns the actual constant value
    bytecode.push(OP_INTC);
    bytecode.push(1); // index
    bytecode = with_assert_equals(bytecode, StackValue::Uint(200));

    execute_and_check(&bytecode, true).unwrap();

    // Test bytecblock with varuint encoding
    let mut bytecode = Vec::new();
    bytecode.push(OP_BYTECBLOCK);
    bytecode.extend_from_slice(&encode_varuint(2)); // count
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "foo"
    bytecode.extend_from_slice(b"foo");
    bytecode.extend_from_slice(&encode_varuint(3)); // length of "bar"
    bytecode.extend_from_slice(b"bar");

    // Now bytec returns the actual constant value
    bytecode.push(OP_BYTEC);
    bytecode.push(0); // index
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"foo".to_vec()));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_mixed_constants() {
    use avm_rs::varuint::encode_varuint;

    // Test mixing different constant operations
    let mut bytecode = Vec::new();

    // Set up integer constant block
    bytecode.push(OP_INTCBLOCK);
    bytecode.extend_from_slice(&encode_varuint(2)); // count
    bytecode.extend_from_slice(&encode_varuint(0)); // constant 0
    bytecode.extend_from_slice(&encode_varuint(1)); // constant 1

    // Push some values
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&42u64.to_be_bytes());

    bytecode.push(OP_PUSHBYTES);
    bytecode.push(4);
    bytecode.extend_from_slice(b"test");

    bytecode.push(OP_INTC_1); // This now pushes 1 (the actual constant value)

    // Stack: [42, "test", 1]
    // To add 42 + 1, we need to remove "test" from between them
    // First swap to get [42, 1, "test"]
    bytecode.push(OP_SWAP); // [42, 1, "test"]
    // Pop "test"
    bytecode.push(OP_POP); // [42, 1]
    // Now add
    bytecode.push(OP_PLUS); // [43]

    // Push "test" back for the length check
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(4);
    bytecode.extend_from_slice(b"test"); // [43, "test"]

    // Check length of bytes
    bytecode.push(OP_LEN);

    // Stack: [43, 4]
    // Multiply
    bytecode.push(OP_MUL);

    bytecode = with_assert_equals(bytecode, StackValue::Uint(172)); // 43 * 4

    execute_and_check(&bytecode, true).unwrap();
}
