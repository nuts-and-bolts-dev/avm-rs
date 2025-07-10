//! Assemble command implementation

use crate::assembler::Assembler;
use crate::cli::{AssembleCommand, GlobalOptions, BytecodeFormat};
use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use std::fs;

/// Handle the assemble command
pub fn handle(cmd: AssembleCommand, global: &GlobalOptions) -> Result<()> {
    if !global.quiet && global.verbose {
        println!("🔧 Assembling TEAL source...");
        println!("Input: {:?}", cmd.input);
        println!("Output format: {:?}", cmd.output_format);
    }

    // Read TEAL source
    let source = fs::read_to_string(&cmd.input)
        .with_context(|| format!("Failed to read TEAL file: {:?}", cmd.input))?;

    // Assemble to bytecode
    let mut assembler = Assembler::new();
    let bytecode = assembler.assemble(&source)
        .map_err(|e| anyhow::anyhow!("Assembly failed: {}", e))?;

    // Format bytecode
    let formatted = format_bytecode(&bytecode, &cmd.output_format)?;

    // Output result
    if let Some(output_path) = &cmd.output {
        fs::write(output_path, &formatted)
            .with_context(|| format!("Failed to write output: {:?}", output_path))?;
        
        if !global.quiet {
            println!("✅ Assembled {} bytes to {:?}", bytecode.len(), output_path);
        }
    } else {
        // Output to stdout
        println!("{}", formatted);
    }

    // Show statistics if requested
    if cmd.stats && !global.quiet {
        show_assembly_stats(&bytecode, &source, global)?;
    }

    Ok(())
}

/// Format bytecode according to specified format
fn format_bytecode(bytecode: &[u8], format: &BytecodeFormat) -> Result<String> {
    match format {
        BytecodeFormat::Hex => Ok(hex::encode(bytecode)),
        BytecodeFormat::Base64 => Ok(BASE64_STANDARD.encode(bytecode)),
        BytecodeFormat::Binary => {
            // For binary output to stdout, we'll use hex representation
            // True binary output only makes sense when writing to a file
            Ok(hex::encode(bytecode))
        }
        BytecodeFormat::Auto => Ok(hex::encode(bytecode)), // Default to hex
    }
}

/// Show assembly statistics
fn show_assembly_stats(bytecode: &[u8], source: &str, global: &GlobalOptions) -> Result<()> {
    let source_lines = source.lines().filter(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with("//") && !line.starts_with(";") && !line.starts_with("#pragma")
    }).count();

    match global.format {
        crate::cli::OutputFormat::Text => {
            println!("\n📊 Assembly Statistics:");
            println!("  Source lines: {}", source_lines);
            println!("  Bytecode size: {} bytes", bytecode.len());
            println!("  Compression ratio: {:.2}x", source.len() as f64 / bytecode.len() as f64);
            
            // Estimate cost (rough approximation)
            let estimated_cost = estimate_execution_cost(bytecode);
            println!("  Estimated cost: ~{} units", estimated_cost);
        }
        crate::cli::OutputFormat::Json => {
            let stats = serde_json::json!({
                "source_lines": source_lines,
                "bytecode_size": bytecode.len(),
                "compression_ratio": source.len() as f64 / bytecode.len() as f64,
                "estimated_cost": estimate_execution_cost(bytecode)
            });
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
    }

    Ok(())
}

/// Estimate execution cost (rough approximation)
fn estimate_execution_cost(bytecode: &[u8]) -> u64 {
    // Very rough estimation: each byte approximately costs 1 unit
    // Real cost depends on actual opcodes being executed
    bytecode.len() as u64
}