//! Rust implementation of the Algorand Virtual Machine (AVM)
//!
//! This library provides a complete implementation of the AVM that executes
//! TEAL (Transaction Execution Approval Language) bytecode for smart contract
//! logic and transaction validation.

pub mod assembler;
pub mod crypto;
pub mod error;
pub mod opcodes;
pub mod state;
pub mod types;
pub mod vm;

// Re-export main types
pub use error::{AvmError, AvmResult};
pub use types::{StackValue, TealValue};
pub use vm::{EvalContext, ExecutionConfig, VirtualMachine};
