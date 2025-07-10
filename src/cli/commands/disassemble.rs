//! Disassemble command implementation

use crate::cli::{DisassembleCommand, GlobalOptions, BytecodeFormat};
use anyhow::{anyhow, Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use std::fs;
use std::path::Path;

/// Handle the disassemble command
pub fn handle(cmd: DisassembleCommand, global: &GlobalOptions) -> Result<()> {
    if !global.quiet && global.verbose {
        println!("🔍 Disassembling bytecode...");
        println!("Input: {}", cmd.input);
        println!("Format: {:?}", cmd.input_format);
    }

    // Load and decode bytecode
    let bytecode = load_bytecode(&cmd.input, &cmd.input_format)?;

    // Disassemble to TEAL
    let teal_source = disassemble_bytecode(&bytecode, &cmd)?;

    // Output result
    if let Some(output_path) = &cmd.output {
        fs::write(output_path, &teal_source)
            .with_context(|| format!("Failed to write output: {:?}", output_path))?;
        
        if !global.quiet {
            println!("✅ Disassembled {} bytes to {:?}", bytecode.len(), output_path);
        }
    } else {
        // Output to stdout
        println!("{}", teal_source);
    }

    // Show analysis if requested
    if cmd.analyze && !global.quiet {
        show_program_analysis(&bytecode, global)?;
    }

    Ok(())
}

/// Load and decode bytecode from input
fn load_bytecode(input: &str, format: &BytecodeFormat) -> Result<Vec<u8>> {
    // Check if input is a file path
    if Path::new(input).exists() {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read file: {}", input))?;
        decode_bytecode_content(&content, format)
    } else {
        // Treat as direct bytecode string
        decode_bytecode_content(input, format)
    }
}

/// Decode bytecode content based on format
fn decode_bytecode_content(content: &str, format: &BytecodeFormat) -> Result<Vec<u8>> {
    let content = content.trim();
    
    match format {
        BytecodeFormat::Auto => {
            // Try different formats
            if let Ok(decoded) = hex::decode(content.replace(' ', "")) {
                Ok(decoded)
            } else if let Ok(decoded) = BASE64_STANDARD.decode(content) {
                Ok(decoded)
            } else {
                Err(anyhow!("Could not auto-detect bytecode format"))
            }
        }
        BytecodeFormat::Hex => {
            hex::decode(content.replace(' ', ""))
                .with_context(|| "Invalid hex bytecode")
        }
        BytecodeFormat::Base64 => {
            BASE64_STANDARD.decode(content)
                .with_context(|| "Invalid base64 bytecode")
        }
        BytecodeFormat::Binary => {
            // For binary input from stdin/file, we expect the raw bytes
            // This is not commonly used in CLI context
            Ok(content.as_bytes().to_vec())
        }
    }
}

/// Disassemble bytecode to TEAL source
fn disassemble_bytecode(bytecode: &[u8], cmd: &DisassembleCommand) -> Result<String> {
    // TODO: Implement actual disassembler
    // For now, return a placeholder that shows the bytecode structure
    
    let mut output = String::new();
    
    if cmd.comments {
        output.push_str("// Disassembled from bytecode\n");
        output.push_str(&format!("// Bytecode size: {} bytes\n", bytecode.len()));
        output.push_str("// Note: This is a simplified disassembly\n\n");
    }
    
    // Add version pragma (we'll assume latest for now)
    output.push_str("#pragma version 11\n\n");
    
    // Simple bytecode analysis
    let mut pc = 0;
    while pc < bytecode.len() {
        let opcode = bytecode[pc];
        
        if cmd.comments {
            output.push_str(&format!("// PC: {}, Opcode: 0x{:02x}\n", pc, opcode));
        }
        
        // Basic opcode recognition (simplified)
        let instruction = match opcode {
            0x01 => "int 1",
            0x02 => "int 2", 
            0x08 => "+",
            0x09 => "-",
            0x0A => "*",
            0x0B => "/",
            0x43 => "return",
            _ => &format!("// Unknown opcode: 0x{:02x}", opcode),
        };
        
        output.push_str(instruction);
        output.push('\n');
        
        pc += 1;
        
        // Handle multi-byte instructions (simplified)
        if opcode == 0x20 {  // int opcode with immediate value
            if pc + 8 <= bytecode.len() {
                pc += 8; // Skip 8-byte immediate
            }
        }
    }
    
    if cmd.comments {
        output.push_str("\n// End of disassembly\n");
    }
    
    Ok(output)
}

/// Show program analysis
fn show_program_analysis(bytecode: &[u8], global: &GlobalOptions) -> Result<()> {
    // Count different instruction types
    let mut instruction_count = 0;
    let mut unknown_opcodes = 0;
    let mut pc = 0;
    
    while pc < bytecode.len() {
        let opcode = bytecode[pc];
        instruction_count += 1;
        
        // Check if it's a known opcode (simplified)
        match opcode {
            0x01..=0x50 => {}, // Known opcodes range (simplified)
            _ => unknown_opcodes += 1,
        }
        
        pc += 1;
        
        // Handle multi-byte instructions
        if opcode == 0x20 && pc + 8 <= bytecode.len() {
            pc += 8;
        }
    }
    
    match global.format {
        crate::cli::OutputFormat::Text => {
            println!("\n📊 Program Analysis:");
            println!("  Bytecode size: {} bytes", bytecode.len());
            println!("  Instructions: {}", instruction_count);
            println!("  Unknown opcodes: {}", unknown_opcodes);
            
            if bytecode.len() > 0 {
                println!("  First opcode: 0x{:02x}", bytecode[0]);
                println!("  Last opcode: 0x{:02x}", bytecode[bytecode.len() - 1]);
            }
        }
        crate::cli::OutputFormat::Json => {
            let analysis = serde_json::json!({
                "bytecode_size": bytecode.len(),
                "instruction_count": instruction_count,
                "unknown_opcodes": unknown_opcodes,
                "first_opcode": if bytecode.len() > 0 { Some(format!("0x{:02x}", bytecode[0])) } else { None },
                "last_opcode": if bytecode.len() > 0 { Some(format!("0x{:02x}", bytecode[bytecode.len() - 1])) } else { None }
            });
            println!("{}", serde_json::to_string_pretty(&analysis)?);
        }
    }
    
    Ok(())
}