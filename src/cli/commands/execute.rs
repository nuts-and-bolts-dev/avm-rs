//! Execute command implementation

use crate::assembler::Assembler;
use crate::cli::{ExecuteCommand, ExecutionMode, GlobalOptions, InputType};
use crate::state::MockLedger;
use crate::types::TealVersion;
use crate::{ExecutionConfig, VirtualMachine};
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::Path;

/// Handle the execute command
pub fn handle(cmd: ExecuteCommand, global: &GlobalOptions) -> Result<()> {
    if !global.quiet && global.verbose {
        println!("🚀 Executing TEAL program...");
        println!("Input: {}", cmd.input);
        println!("Type: {:?}", cmd.input_type);
        println!("Mode: {:?}", cmd.mode);
    }

    // Determine input type and load bytecode
    let bytecode = load_input(&cmd)?;

    // Create VM with specified version
    let version = cmd
        .version
        .map(TealVersion::from_u8)
        .transpose()
        .context("Invalid TEAL version")?
        .unwrap_or(TealVersion::latest());

    let vm = VirtualMachine::with_version(version);

    // Configure execution
    let run_mode = match cmd.mode {
        ExecutionMode::Signature => crate::types::RunMode::Signature,
        ExecutionMode::Application => crate::types::RunMode::Application,
    };

    let config = ExecutionConfig::new(version)
        .with_cost_budget(cmd.budget)
        .with_run_mode(run_mode);

    // Setup mock ledger
    let mut ledger = setup_ledger(&cmd)?;

    // Execute the program
    if cmd.step {
        execute_with_stepping(&vm, &bytecode, config, &mut ledger, global)
    } else {
        execute_normal(&vm, &bytecode, config, &mut ledger, global)
    }
}

/// Load input based on type
fn load_input(cmd: &ExecuteCommand) -> Result<Vec<u8>> {
    match cmd.input_type {
        InputType::Auto => auto_detect_and_load(&cmd.input),
        InputType::File => load_from_file(&cmd.input),
        InputType::Bytecode => decode_bytecode(&cmd.input),
        InputType::Inline => assemble_inline(&cmd.input),
    }
}

/// Auto-detect input type and load accordingly
fn auto_detect_and_load(input: &str) -> Result<Vec<u8>> {
    // Check if it's a file path
    if Path::new(input).exists() {
        return load_from_file(input);
    }

    // Check if it looks like hex bytecode
    if input
        .chars()
        .all(|c| c.is_ascii_hexdigit() || c.is_whitespace())
        && input.len() > 10
    {
        if let Ok(bytecode) = decode_bytecode(input) {
            return Ok(bytecode);
        }
    }

    // Treat as inline TEAL
    assemble_inline(input)
}

/// Load bytecode from file
fn load_from_file(path: &str) -> Result<Vec<u8>> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {path}"))?;

    // Check if file contains TEAL source or bytecode
    if content.trim_start().starts_with("#pragma") || content.contains("int ") {
        // TEAL source file
        assemble_inline(&content)
    } else {
        // Assume bytecode file
        decode_bytecode(&content)
    }
}

/// Decode hex bytecode string
fn decode_bytecode(hex: &str) -> Result<Vec<u8>> {
    let hex = hex
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    hex::decode(&hex).with_context(|| "Invalid hex bytecode")
}

/// Assemble inline TEAL source
fn assemble_inline(source: &str) -> Result<Vec<u8>> {
    let mut assembler = Assembler::new();
    assembler
        .assemble(source)
        .map_err(|e| anyhow!("Assembly failed: {}", e))
}

/// Setup mock ledger with optional data
fn setup_ledger(cmd: &ExecuteCommand) -> Result<MockLedger> {
    let ledger = MockLedger::default();

    // Load ledger data if provided
    if let Some(ledger_file) = &cmd.ledger {
        let _content = fs::read_to_string(ledger_file)
            .with_context(|| format!("Failed to read ledger file: {ledger_file:?}"))?;

        // TODO: Implement JSON deserialization for ledger data
        // For now, use default ledger
    }

    // Load transaction data if provided
    if let Some(txn_file) = &cmd.transaction {
        let _content = fs::read_to_string(txn_file)
            .with_context(|| format!("Failed to read transaction file: {txn_file:?}"))?;

        // TODO: Implement JSON deserialization for transaction data
        // For now, use default transaction
    }

    Ok(ledger)
}

/// Execute program normally
fn execute_normal(
    vm: &VirtualMachine,
    bytecode: &[u8],
    config: ExecutionConfig,
    ledger: &mut MockLedger,
    global: &GlobalOptions,
) -> Result<()> {
    let start = std::time::Instant::now();

    let result = vm
        .execute(bytecode, config.clone(), ledger)
        .map_err(|e| anyhow!("Execution failed: {}", e))?;

    let duration = start.elapsed();

    if !global.quiet {
        match global.format {
            crate::cli::OutputFormat::Text => {
                println!("✅ Execution completed successfully");
                println!("Result: {result}");

                if global.verbose {
                    println!("Duration: {duration:?}");
                    println!("Cost budget: {}", config.cost_budget);
                }
            }
            crate::cli::OutputFormat::Json => {
                let output = serde_json::json!({
                    "success": true,
                    "result": result,
                    "duration_ms": duration.as_millis(),
                    "cost_budget": config.cost_budget
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
    }

    Ok(())
}

/// Execute program with step-by-step debugging
fn execute_with_stepping(
    vm: &VirtualMachine,
    bytecode: &[u8],
    config: ExecutionConfig,
    ledger: &mut MockLedger,
    global: &GlobalOptions,
) -> Result<()> {
    if !global.quiet {
        println!("🔍 Step-by-step execution mode");
        println!("Press Enter to step, 'c' to continue, 'q' to quit");
    }

    // TODO: Implement step-by-step execution
    // This would require modifications to the VM to support stepping
    // For now, fall back to normal execution
    println!("⚠️  Step-by-step execution not yet implemented, running normally...");
    execute_normal(vm, bytecode, config.clone(), ledger, global)
}
