//! Error types for the Algorand Virtual Machine

use thiserror::Error;

/// Result type for AVM operations
pub type AvmResult<T> = Result<T, AvmError>;

/// Comprehensive error types for AVM operations
#[derive(Debug, Error)]
pub enum AvmError {
    #[error("Stack underflow: attempted to pop from empty stack")]
    StackUnderflow,

    #[error("Stack overflow: stack size exceeded limit of {limit}")]
    StackOverflow { limit: usize },

    #[error("Type error: expected {expected}, got {actual}")]
    TypeError { expected: String, actual: String },

    #[error("Invalid opcode: {opcode:#04x} at program counter {pc}")]
    InvalidOpcode { opcode: u8, pc: usize },

    #[error("Program counter out of bounds: {pc} >= {program_len}")]
    ProgramCounterOutOfBounds { pc: usize, program_len: usize },

    #[error("Cost budget exceeded: {actual} > {limit}")]
    CostBudgetExceeded { actual: u64, limit: u64 },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Integer overflow in arithmetic operation")]
    IntegerOverflow,

    #[error("Integer underflow in arithmetic operation")]
    IntegerUnderflow,

    #[error("Invalid branch target: {target}")]
    InvalidBranchTarget { target: i32 },

    #[error("Call stack overflow: maximum depth {limit} exceeded")]
    CallStackOverflow { limit: usize },

    #[error("Call stack underflow: attempted to return from empty call stack")]
    CallStackUnderflow,

    #[error("Scratch space index out of bounds: {index} >= {limit}")]
    ScratchIndexOutOfBounds { index: u8, limit: u8 },

    #[error("Invalid byte array length: expected {expected}, got {actual}")]
    InvalidByteArrayLength { expected: usize, actual: usize },

    #[error("Invalid transaction field: {field}")]
    InvalidTransactionField { field: String },

    #[error("Invalid global field: {field}")]
    InvalidGlobalField { field: String },

    #[error("Ledger access error: {0}")]
    LedgerError(String),

    #[error("State access error: {0}")]
    StateError(String),

    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),

    #[error("Invalid program: {0}")]
    InvalidProgram(String),

    #[error("Execution halted: {reason}")]
    ExecutionHalted { reason: String },

    #[error("Invalid TEAL version: {version}")]
    InvalidTealVersion { version: u8 },

    #[error("Unsupported TEAL version: {0}")]
    UnsupportedVersion(u8),

    #[error("Opcode not available in version {version}: {opcode}")]
    OpcodeNotAvailable { version: u8, opcode: String },

    #[error("Assembly error: {0}")]
    AssemblyError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl AvmError {
    /// Create a new ledger error
    pub fn ledger_error(msg: impl Into<String>) -> Self {
        Self::LedgerError(msg.into())
    }

    /// Create a new state error
    pub fn state_error(msg: impl Into<String>) -> Self {
        Self::StateError(msg.into())
    }

    /// Create a new crypto error
    pub fn crypto_error(msg: impl Into<String>) -> Self {
        Self::CryptoError(msg.into())
    }

    /// Create a new invalid program error
    pub fn invalid_program(msg: impl Into<String>) -> Self {
        Self::InvalidProgram(msg.into())
    }

    /// Create a new execution halted error
    pub fn execution_halted(reason: impl Into<String>) -> Self {
        Self::ExecutionHalted {
            reason: reason.into(),
        }
    }

    /// Create a new assembly error
    pub fn assembly_error(msg: impl Into<String>) -> Self {
        Self::AssemblyError(msg.into())
    }

    /// Create a new parse error
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::ParseError(msg.into())
    }
}
