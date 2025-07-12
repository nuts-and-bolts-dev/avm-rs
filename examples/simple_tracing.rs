//! Simple example demonstrating minimal tracing functionality
//!
//! This example shows how the simplified tracing system tracks:
//! - Opcode execution
//! - Stack state before and after each operation

use avm_rs::assembler::Assembler;
use avm_rs::state::MockLedger;
use avm_rs::types::TealVersion;
use avm_rs::{ExecutionConfig, VirtualMachine};

#[cfg(feature = "tracing")]
use avm_rs::tracing::{TraceLevel, TracingConfig, init_tracing};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple TEAL program that performs arithmetic operations
    let teal_source = r#"
        #pragma version 11
        int 10
        int 5
        +
        int 3
        *
        int 2
        -
        return
    "#;

    // Assemble the TEAL source to bytecode
    let mut assembler = Assembler::new();
    let bytecode = assembler
        .assemble(teal_source)
        .map_err(|e| format!("Assembly failed: {e}"))?;

    println!("🚀 Simple Tracing Example");
    println!("=========================");
    println!();
    println!("TEAL Program:");
    println!("{teal_source}");
    println!("Expected result: ((10 + 5) * 3) - 2 = 43");
    println!();

    // Configure tracing
    #[cfg(feature = "tracing")]
    {
        let tracing_config = TracingConfig::new()
            .with_level(TraceLevel::Debug)
            .with_opcodes(true)
            .with_stack(true)
            .with_max_stack_depth(5);

        // Initialize tracing
        let _guard = init_tracing(&tracing_config)?;

        println!("📊 Tracing enabled - showing opcode execution and stack state");
        println!();

        // Create execution configuration with tracing
        let config = ExecutionConfig::new(TealVersion::V11).with_tracing(tracing_config);

        // Execute with tracing
        execute_program(&bytecode, config)?;
    }

    #[cfg(not(feature = "tracing"))]
    {
        println!(
            "⚠️  Tracing feature not enabled - compile with --features tracing to see trace output"
        );
        println!();

        // Execute without tracing
        let config = ExecutionConfig::new(TealVersion::V11);
        execute_program(&bytecode, config)?;
    }

    Ok(())
}

fn execute_program(
    bytecode: &[u8],
    config: ExecutionConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create VM and ledger
    let vm = VirtualMachine::with_version(TealVersion::V11);
    let mut ledger = MockLedger::default();

    // Execute the program
    let result = vm
        .execute(bytecode, config, &mut ledger)
        .map_err(|e| format!("Execution failed: {e}"))?;

    println!();
    println!("✅ Execution completed successfully!");
    println!("Final result: {result}");

    Ok(())
}
