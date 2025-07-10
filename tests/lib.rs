//! AVM Opcode Test Suite
//!
//! Comprehensive test suite for all Algorand Virtual Machine opcodes.
//! Tests are organized by opcode category for better maintainability.

#![allow(clippy::vec_init_then_push)]

pub mod common;
pub mod constant_blocks;

// Opcode category tests
pub mod opcodes {
    pub mod arithmetic_tests;
    pub mod constants_tests;
    pub mod crypto_tests;
    pub mod flow_tests;
    pub mod integration_tests;
    pub mod stack_tests;
    pub mod state_tests;
    pub mod transaction_tests;
}
