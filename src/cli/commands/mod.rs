//! Command implementations for the CLI

pub mod assemble;
pub mod disassemble;
pub mod execute;
pub mod info;
pub mod repl;
pub mod validate;

// Re-export command handlers for easier access
pub use assemble::handle as assemble_handler;
pub use disassemble::handle as disassemble_handler;
pub use execute::handle as execute_handler;
pub use info::handle as info_handler;
pub use repl::handle as repl_handler;
pub use validate::handle as validate_handler;
