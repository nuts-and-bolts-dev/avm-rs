//! Execute command implementation

use crate::assembler::Assembler;
#[cfg(feature = "tracing")]
use crate::cli::TracingLevel;
use crate::cli::{ExecuteCommand, ExecutionMode, GlobalOptions, InputType};
use crate::state::MockLedger;
#[cfg(feature = "tracing")]
use crate::tracing::{TraceLevel, TracingConfig};
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

        #[cfg(feature = "tracing")]
        if cmd.trace_level.is_some() || cmd.trace_opcodes || cmd.trace_stack {
            println!("Tracing: enabled");
            if let Some(trace_level) = &cmd.trace_level {
                println!("Trace level: {trace_level:?}");
            } else if cmd.trace_opcodes || cmd.trace_stack {
                println!("Trace level: Debug (auto-enabled by trace flags)");
            }
            println!("Trace opcodes: {}", cmd.trace_opcodes);
            println!("Trace stack: {}", cmd.trace_stack);
        }
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

    #[cfg(feature = "tracing")]
    let config = if cmd.trace_level.is_some() || cmd.trace_opcodes || cmd.trace_stack {
        let tracing_config = build_tracing_config(&cmd)?;
        ExecutionConfig::new(version)
            .with_cost_budget(cmd.budget)
            .with_run_mode(run_mode)
            .with_tracing(tracing_config)
    } else {
        ExecutionConfig::new(version)
            .with_cost_budget(cmd.budget)
            .with_run_mode(run_mode)
    };

    #[cfg(not(feature = "tracing"))]
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
    use std::io::{self, Write};

    if !global.quiet {
        println!("🔍 Step-by-step execution mode");
        println!("Commands: [Enter] step, 'c' continue, 'q' quit, 'h' help");
        println!("{}", "─".repeat(60));
    }

    let start = std::time::Instant::now();

    // Create evaluation context for stepping
    let mut eval_ctx = vm
        .create_eval_context(bytecode, config.clone(), ledger)
        .map_err(|e| anyhow::anyhow!("Failed to create evaluation context: {}", e))?;

    let mut step_count = 0;
    let mut continue_mode = false;

    while !eval_ctx.is_finished() {
        if !continue_mode && !global.quiet {
            // Display current state
            let opcode_info = eval_ctx
                .current_opcode_spec(vm)
                .map(|spec| format!("{} (cost: {})", spec.name, spec.cost))
                .unwrap_or_else(|_| "Invalid opcode".to_string());

            println!(
                "Step {}: PC={:04} | {}",
                step_count,
                eval_ctx.pc(),
                opcode_info
            );

            // Display stack
            let stack = eval_ctx.stack();
            if stack.is_empty() {
                println!("Stack: (empty)");
            } else {
                println!(
                    "Stack: [{}]",
                    stack
                        .iter()
                        .enumerate()
                        .map(|(i, val)| format!("{i}: {val:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }

            print!("vm> ");
            io::stdout().flush().unwrap();

            // Read user input
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "q" | "quit" => {
                    println!("Execution interrupted by user");
                    return Ok(());
                }
                "c" | "continue" => {
                    continue_mode = true;
                    println!("Continuing execution...");
                }
                "h" | "help" => {
                    println!("Commands:");
                    println!("  [Enter] - Execute next instruction");
                    println!("  c       - Continue execution without stepping");
                    println!("  q       - Quit execution");
                    println!("  h       - Show this help");
                    continue;
                }
                "" => {
                    // Step (default action)
                }
                _ => {
                    println!("Unknown command '{input}'. Type 'h' for help.");
                    continue;
                }
            }
        }

        // Execute one step
        eval_ctx
            .step(vm, &config)
            .map_err(|e| anyhow::anyhow!("Execution failed at step {}: {}", step_count, e))?;

        step_count += 1;
    }

    // Extract final result
    let result = if eval_ctx.is_finished() {
        // Check final result
        let stack = eval_ctx.stack();
        if stack.is_empty() {
            return Err(anyhow::anyhow!(
                "Program ended with 0 values on stack, expected 1"
            ));
        }
        if stack.len() > 1 {
            return Err(anyhow::anyhow!(
                "Program ended with {} values on stack, expected 1",
                stack.len()
            ));
        }
        stack[0]
            .as_bool()
            .map_err(|e| anyhow::anyhow!("Invalid final result: {}", e))?
    } else {
        return Err(anyhow::anyhow!("Program execution incomplete"));
    };

    let duration = start.elapsed();

    if !global.quiet {
        println!("{}", "─".repeat(60));
        match global.format {
            crate::cli::OutputFormat::Text => {
                println!("✅ Execution completed successfully");
                println!("Result: {result}");
                println!("Steps: {step_count}");
                println!("Duration: {duration:?}");
            }
            crate::cli::OutputFormat::Json => {
                let output = serde_json::json!({
                    "success": true,
                    "result": result,
                    "steps": step_count,
                    "duration_ms": duration.as_millis()
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
    }

    Ok(())
}

/// Build tracing configuration from CLI options
#[cfg(feature = "tracing")]
fn build_tracing_config(cmd: &ExecuteCommand) -> Result<TracingConfig> {
    // If trace-opcodes or trace-stack are specified but no explicit level, default to debug
    let default_level = if (cmd.trace_opcodes || cmd.trace_stack) && cmd.trace_level.is_none() {
        TracingLevel::Debug
    } else {
        TracingLevel::Info
    };

    let level = match cmd.trace_level.as_ref().unwrap_or(&default_level) {
        TracingLevel::Trace => TraceLevel::Trace,
        TracingLevel::Debug => TraceLevel::Debug,
        TracingLevel::Info => TraceLevel::Info,
        TracingLevel::Warn => TraceLevel::Warn,
        TracingLevel::Error => TraceLevel::Error,
    };

    Ok(TracingConfig::new()
        .with_level(level)
        .with_opcodes(cmd.trace_opcodes)
        .with_stack(cmd.trace_stack))
}
