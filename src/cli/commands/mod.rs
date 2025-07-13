//! Command implementations for the CLI

pub mod assemble;
pub mod disassemble;
pub mod execute;
pub mod repl;
pub mod validate;

// Re-export command handlers for easier access
pub use assemble::handle as assemble_handler;
pub use disassemble::handle as disassemble_handler;
pub use execute::handle as execute_handler;
pub use validate::handle as validate_handler;
