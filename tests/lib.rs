#![allow(clippy::vec_init_then_push)]

pub mod common;
pub mod constant_blocks;

// Opcode category tests
pub mod opcodes {
    pub mod arithmetic;
    pub mod constants;
    pub mod crypto;
    pub mod flow;
    pub mod integration;
    pub mod stack;
    pub mod state;
    pub mod transaction;
}
