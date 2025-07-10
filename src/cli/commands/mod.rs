//! Command implementations for the CLI

pub mod execute;
pub mod assemble;
pub mod disassemble;
pub mod validate;
pub mod examples;
pub mod info;
pub mod repl;

// Re-export command handlers for easier access
pub use execute::handle as execute_handler;
pub use assemble::handle as assemble_handler;
pub use disassemble::handle as disassemble_handler;
pub use validate::handle as validate_handler;
pub use examples::handle as examples_handler;
pub use info::handle as info_handler;
pub use repl::handle as repl_handler;