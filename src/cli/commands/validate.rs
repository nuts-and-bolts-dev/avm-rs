//! Validate command implementation

use crate::assembler::Assembler;
use crate::cli::{ValidateCommand, GlobalOptions, ExecutionMode};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Handle the validate command
pub fn handle(cmd: ValidateCommand, global: &GlobalOptions) -> Result<()> {
    if !global.quiet && global.verbose {
        println!("🔍 Validating TEAL programs...");
        println!("Files: {:?}", cmd.files);
    }

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut warnings = 0;
    let mut errors = 0;

    for file_path in &cmd.files {
        total_files += 1;
        
        if !global.quiet && global.verbose {
            println!("\nValidating: {:?}", file_path);
        }

        match validate_file(file_path, &cmd, global) {
            Ok(file_result) => {
                valid_files += 1;
                warnings += file_result.warnings;
                
                if !global.quiet {
                    if file_result.warnings > 0 {
                        println!("⚠️  {:?}: Valid with {} warnings", file_path, file_result.warnings);
                    } else {
                        println!("✅ {:?}: Valid", file_path);
                    }
                }
            }
            Err(e) => {
                errors += 1;
                if !global.quiet {
                    println!("❌ {:?}: {}", file_path, e);
                }
            }
        }
    }

    // Summary
    if !global.quiet {
        match global.format {
            crate::cli::OutputFormat::Text => {
                println!("\n📊 Validation Summary:");
                println!("  Total files: {}", total_files);
                println!("  Valid files: {}", valid_files);
                println!("  Failed files: {}", errors);
                println!("  Total warnings: {}", warnings);
                
                if errors > 0 {
                    println!("❌ Validation failed for {} files", errors);
                } else if warnings > 0 && cmd.strict {
                    println!("⚠️  Validation passed but {} warnings in strict mode", warnings);
                } else {
                    println!("✅ All files validated successfully");
                }
            }
            crate::cli::OutputFormat::Json => {
                let summary = serde_json::json!({
                    "total_files": total_files,
                    "valid_files": valid_files,
                    "failed_files": errors,
                    "total_warnings": warnings,
                    "success": errors == 0 && (!cmd.strict || warnings == 0)
                });
                println!("{}", serde_json::to_string_pretty(&summary)?);
            }
        }
    }

    // Exit with error if validation failed
    if errors > 0 || (cmd.strict && warnings > 0) {
        std::process::exit(1);
    }

    Ok(())
}

/// Result of validating a single file
struct FileValidationResult {
    warnings: usize,
}

/// Validate a single TEAL file
fn validate_file(file_path: &Path, cmd: &ValidateCommand, global: &GlobalOptions) -> Result<FileValidationResult> {
    // Read the file
    let source = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;

    // Parse and validate syntax
    let mut assembler = Assembler::new();
    let _bytecode = assembler.assemble(&source)
        .map_err(|e| anyhow::anyhow!("Assembly failed: {}", e))?;

    // Additional validation checks
    let mut warnings = 0;

    // Check TEAL version compatibility
    if let Some(target_version) = cmd.version {
        warnings += check_version_compatibility(&source, target_version)?;
    }

    // Check execution mode compatibility
    if let Some(target_mode) = &cmd.mode {
        warnings += check_mode_compatibility(&source, target_mode)?;
    }

    // Perform detailed analysis if requested
    if cmd.detailed {
        warnings += perform_detailed_analysis(&source, global)?;
    }

    Ok(FileValidationResult { warnings })
}

/// Check version compatibility
fn check_version_compatibility(source: &str, target_version: u8) -> Result<usize> {
    let mut warnings = 0;
    
    // Extract pragma version from source
    let declared_version = extract_pragma_version(source)?;
    
    if let Some(declared) = declared_version {
        if declared > target_version {
            warnings += 1;
            eprintln!("Warning: File declares version {} but target is {}", declared, target_version);
        }
    } else {
        warnings += 1;
        eprintln!("Warning: No version pragma found, assuming latest version");
    }
    
    Ok(warnings)
}

/// Check execution mode compatibility
fn check_mode_compatibility(source: &str, target_mode: &ExecutionMode) -> Result<usize> {
    let mut warnings = 0;
    
    // Check for mode-specific opcodes
    match target_mode {
        ExecutionMode::Signature => {
            if source.contains("app_global_get") || source.contains("app_local_get") {
                warnings += 1;
                eprintln!("Warning: Application opcodes found in signature mode validation");
            }
        }
        ExecutionMode::Application => {
            // Application mode is more permissive
        }
    }
    
    Ok(warnings)
}

/// Perform detailed analysis
fn perform_detailed_analysis(source: &str, global: &GlobalOptions) -> Result<usize> {
    let mut warnings = 0;
    
    // Check for common issues
    let lines: Vec<&str> = source.lines().collect();
    
    for (line_num, line) in lines.iter().enumerate() {
        let line = line.trim();
        
        // Check for potential issues
        if line.contains("int 0") && line.contains("==") {
            warnings += 1;
            if !global.quiet {
                eprintln!("Warning: Line {}: Consider using '!' instead of '== 0'", line_num + 1);
            }
        }
        
        if line.contains("b ") && !line.contains("bnz") && !line.contains("bz") {
            if !global.quiet {
                eprintln!("Info: Line {}: Unconditional branch found", line_num + 1);
            }
        }
    }
    
    // Check program structure
    if !source.contains("return") {
        warnings += 1;
        if !global.quiet {
            eprintln!("Warning: No 'return' statement found");
        }
    }
    
    Ok(warnings)
}

/// Extract pragma version from source
fn extract_pragma_version(source: &str) -> Result<Option<u8>> {
    for line in source.lines() {
        let line = line.trim();
        if line.starts_with("#pragma version") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                return Ok(Some(parts[2].parse()?));
            }
        }
    }
    Ok(None)
}