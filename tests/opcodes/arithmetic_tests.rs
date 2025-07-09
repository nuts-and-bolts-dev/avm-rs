//! Tests for arithmetic opcodes

use rust_avm::{opcodes::*, types::StackValue};

use crate::common::*;

#[test]
fn test_op_err() {
    // The err opcode should always fail
    let bytecode = vec![
        OP_ERR, // err opcode
    ];

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_plus() {
    // Test basic addition
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(5)], OP_PLUS),
        StackValue::Uint(15),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test addition with zero
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(0)], OP_PLUS),
        StackValue::Uint(42),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_plus_overflow() {
    // Test overflow detection
    let bytecode = build_simple_op_test(
        vec![StackValue::Uint(u64::MAX), StackValue::Uint(1)],
        OP_PLUS,
    );
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_plus_type_error() {
    // Test type mismatch (bytes + uint)
    let bytecode = build_simple_op_test(
        vec![StackValue::Bytes(vec![1, 2, 3]), StackValue::Uint(5)],
        OP_PLUS,
    );
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_minus() {
    // Test basic subtraction
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(15), StackValue::Uint(5)], OP_MINUS),
        StackValue::Uint(10),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test subtraction to zero
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(42)], OP_MINUS),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_minus_underflow() {
    // Test underflow detection
    let bytecode = build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(10)], OP_MINUS);
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_div() {
    // Test basic division
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(20), StackValue::Uint(4)], OP_DIV),
        StackValue::Uint(5),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test integer division (truncation)
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(17), StackValue::Uint(5)], OP_DIV),
        StackValue::Uint(3),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_div_by_zero() {
    // Test division by zero
    let bytecode = build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(0)], OP_DIV);
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_mul() {
    // Test basic multiplication
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(6), StackValue::Uint(7)], OP_MUL),
        StackValue::Uint(42),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test multiplication by zero
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(100), StackValue::Uint(0)], OP_MUL),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test multiplication by one
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(1)], OP_MUL),
        StackValue::Uint(42),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_mul_overflow() {
    // Test overflow detection
    let bytecode = build_simple_op_test(
        vec![StackValue::Uint(u64::MAX / 2 + 1), StackValue::Uint(2)],
        OP_MUL,
    );
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_mod() {
    // Test basic modulo
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(17), StackValue::Uint(5)], OP_MOD),
        StackValue::Uint(2),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test modulo with no remainder
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(20), StackValue::Uint(4)], OP_MOD),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_mod_by_zero() {
    // Test modulo by zero
    let bytecode = build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(0)], OP_MOD);
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_lt() {
    // Test less than with uints
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(10)], OP_LT),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(5)], OP_LT),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test equal values
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(5)], OP_LT),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test less than with bytes
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![
                StackValue::Bytes(vec![1, 2, 3]),
                StackValue::Bytes(vec![1, 2, 4]),
            ],
            OP_LT,
        ),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_gt() {
    // Test greater than with uints
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(5)], OP_GT),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(10)], OP_GT),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test with bytes
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![
                StackValue::Bytes(vec![1, 2, 4]),
                StackValue::Bytes(vec![1, 2, 3]),
            ],
            OP_GT,
        ),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_le() {
    // Test less than or equal with uints
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(10)], OP_LE),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(5)], OP_LE),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(5)], OP_LE),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ge() {
    // Test greater than or equal with uints
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(10), StackValue::Uint(5)], OP_GE),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(5)], OP_GE),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(5), StackValue::Uint(10)], OP_GE),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_eq() {
    // Test equality with uints
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(42)], OP_EQ),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(43)], OP_EQ),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test equality with bytes
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![
                StackValue::Bytes(vec![1, 2, 3]),
                StackValue::Bytes(vec![1, 2, 3]),
            ],
            OP_EQ,
        ),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test different types are not equal
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1), StackValue::Bytes(vec![1])], OP_EQ),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ne() {
    // Test inequality with uints
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(43)], OP_NE),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(42)], OP_NE),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Test different types are not equal
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1), StackValue::Bytes(vec![1])], OP_NE),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_and() {
    // Test logical AND
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1), StackValue::Uint(1)], OP_AND),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1), StackValue::Uint(0)], OP_AND),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0), StackValue::Uint(1)], OP_AND),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0), StackValue::Uint(0)], OP_AND),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Non-zero values are true
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42), StackValue::Uint(100)], OP_AND),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_or() {
    // Test logical OR
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1), StackValue::Uint(1)], OP_OR),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1), StackValue::Uint(0)], OP_OR),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0), StackValue::Uint(1)], OP_OR),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0), StackValue::Uint(0)], OP_OR),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_not() {
    // Test logical NOT
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0)], OP_NOT),
        StackValue::Uint(1),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(1)], OP_NOT),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();

    // Non-zero is false
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(42)], OP_NOT),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bitwise_or() {
    // Test bitwise OR
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Uint(0b1010), StackValue::Uint(0b0101)],
            OP_BITWISE_OR,
        ),
        StackValue::Uint(0b1111),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Uint(0xFF00), StackValue::Uint(0x00FF)],
            OP_BITWISE_OR,
        ),
        StackValue::Uint(0xFFFF),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bitwise_and() {
    // Test bitwise AND
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Uint(0b1110), StackValue::Uint(0b1011)],
            OP_BITWISE_AND,
        ),
        StackValue::Uint(0b1010),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Uint(0xFF00), StackValue::Uint(0xF0F0)],
            OP_BITWISE_AND,
        ),
        StackValue::Uint(0xF000),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bitwise_xor() {
    // Test bitwise XOR
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Uint(0b1100), StackValue::Uint(0b1010)],
            OP_BITWISE_XOR,
        ),
        StackValue::Uint(0b0110),
    );
    execute_and_check(&bytecode, true).unwrap();

    // XOR with itself should be 0
    let bytecode = with_assert_equals(
        build_simple_op_test(
            vec![StackValue::Uint(0xABCD), StackValue::Uint(0xABCD)],
            OP_BITWISE_XOR,
        ),
        StackValue::Uint(0),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_bitwise_not() {
    // Test bitwise NOT
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0)], OP_BITWISE_NOT),
        StackValue::Uint(u64::MAX),
    );
    execute_and_check(&bytecode, true).unwrap();

    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Uint(0xFF)], OP_BITWISE_NOT),
        StackValue::Uint(u64::MAX - 0xFF),
    );
    execute_and_check(&bytecode, true).unwrap();
}
