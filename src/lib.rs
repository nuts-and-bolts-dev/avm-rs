//! Rust implementation of the Algorand Virtual Machine (AVM)
//!
//! This library provides a complete implementation of the AVM that executes
//! TEAL (Transaction Execution Approval Language) bytecode for smart contract
//! logic and transaction validation.

pub mod assembler;
pub mod cli;
pub mod crypto;
pub mod error;
pub mod opcodes;
pub mod state;
pub mod types;
pub mod varuint;
pub mod vm;

// Re-export main types
pub use error::{AvmError, AvmResult};
pub use types::{StackValue, TealValue, TealVersion};
pub use vm::{EvalContext, ExecutionConfig, VirtualMachine, VirtualMachineBuilder};

#[cfg(test)]
mod tests {
    use super::*;
    use assembler::Assembler;
    use state::MockLedger;

    fn setup_vm() -> VirtualMachine {
        VirtualMachine::with_version(TealVersion::V6)
    }

    fn test_config() -> ExecutionConfig {
        ExecutionConfig::new(TealVersion::V6).with_cost_budget(1000) // Reasonable limit for tests
    }

    #[test]
    fn test_basic_teal_program() {
        let teal_program = r#"
#pragma version 6
// This is a comment
int 10      ; Push 10 onto stack
int 5       ; Push 5 onto stack
+           ; Add them
int 15      ; Push expected result
==          ; Check equality
return      ; Return the result (true/false)
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        let vm = setup_vm();
        let mut ledger = MockLedger::new();
        let result = vm
            .execute(&bytecode, test_config(), &mut ledger)
            .expect("Execution failed");

        assert!(result);
    }

    #[test]
    fn test_integer_formats() {
        let teal_program = r#"
#pragma version 6
int 42          // Decimal
int 0x2A        // Hexadecimal
==              // Should be equal
int 0o52        // Octal
int 0b101010    // Binary
==              // Should be equal
&&              // Both should be true
return
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        let vm = setup_vm();
        let mut ledger = MockLedger::new();
        let result = vm
            .execute(&bytecode, test_config(), &mut ledger)
            .expect("Execution failed");

        assert!(result);
    }

    #[test]
    fn test_byte_constants() {
        let teal_program = r#"
#pragma version 6
byte "hello"     // String literal
byte "hello"     // String literal
==               // Check equality
return
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        let vm = setup_vm();
        let mut ledger = MockLedger::new();
        let result = vm
            .execute(&bytecode, test_config(), &mut ledger)
            .expect("Execution failed");

        assert!(result);
    }

    #[test]
    fn test_labels_and_branching() {
        let teal_program = r#"
#pragma version 6
int 1
bnz success
err
success:
int 1
return
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        let vm = setup_vm();
        let mut ledger = MockLedger::new();
        let result = vm
            .execute(&bytecode, test_config(), &mut ledger)
            .expect("Execution failed");

        assert!(result);
    }

    #[test]
    fn test_comment_styles() {
        let teal_program = r#"
#pragma version 6
// This is a line comment
int 1       ; This is an inline comment
int 2       // Another inline comment
+           ; Add them
int 3       ; Expected result
==          ; Check equality
return      ; Return the result
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        let vm = setup_vm();
        let mut ledger = MockLedger::new();
        let result = vm
            .execute(&bytecode, test_config(), &mut ledger)
            .expect("Execution failed");

        assert!(result);
    }

    #[test]
    fn test_pragma_typetrack() {
        let teal_program = r#"
#pragma version 6
#pragma typetrack true
int 42
int 42
==
return
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        // Should assemble without error
        assert!(!bytecode.is_empty());
    }

    #[test]
    fn test_assembler_syntax_compliance() {
        // Test that we correctly handle official TEAL syntax
        let teal_program = r#"
#pragma version 6
// Test basic opcodes match official syntax
int 1           // Not "pushint 1"
int 1           // Second int
==              // Compare
return          // Return result
"#;

        let mut assembler = Assembler::new();
        let bytecode = assembler.assemble(teal_program).expect("Assembly failed");

        let vm = setup_vm();
        let mut ledger = MockLedger::new();
        let result = vm
            .execute(&bytecode, test_config(), &mut ledger)
            .expect("Execution failed");

        assert!(result);
    }
}
