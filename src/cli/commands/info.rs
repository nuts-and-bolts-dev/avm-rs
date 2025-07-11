//! Info command implementation

use crate::cli::{GlobalOptions, InfoCommand};
use crate::types::TealVersion;
use anyhow::Result;

/// Handle the info command
pub fn handle(cmd: InfoCommand, global: &GlobalOptions) -> Result<()> {
    if cmd.versions {
        show_versions(global)?;
    }

    if cmd.opcodes {
        show_opcodes(cmd.version, cmd.details, global)?;
    }

    if cmd.system {
        show_system_info(global)?;
    }

    // If no specific info requested, show general info
    if !cmd.versions && !cmd.opcodes && !cmd.system {
        show_general_info(global)?;
    }

    Ok(())
}

/// Show supported TEAL versions
fn show_versions(global: &GlobalOptions) -> Result<()> {
    if global.quiet {
        return Ok(());
    }

    match global.format {
        crate::cli::OutputFormat::Text => {
            println!("🔢 Supported TEAL Versions:\n");

            for version in TealVersion::all() {
                let features = get_version_features(*version);
                println!("  Version {} - {}", version.as_u8(), features);
            }

            println!("\n  Latest: Version {}", TealVersion::latest().as_u8());
        }
        crate::cli::OutputFormat::Json => {
            let versions: Vec<_> = TealVersion::all()
                .iter()
                .map(|v| {
                    serde_json::json!({
                        "version": v.as_u8(),
                        "features": get_version_features(*v)
                    })
                })
                .collect();

            let output = serde_json::json!({
                "supported_versions": versions,
                "latest_version": TealVersion::latest().as_u8()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

/// Show available opcodes
fn show_opcodes(
    version_filter: Option<u8>,
    show_details: bool,
    global: &GlobalOptions,
) -> Result<()> {
    if global.quiet {
        return Ok(());
    }

    let target_version = version_filter
        .map(TealVersion::from_u8)
        .transpose()?
        .unwrap_or(TealVersion::latest());

    let opcodes = get_available_opcodes(target_version);

    match global.format {
        crate::cli::OutputFormat::Text => {
            println!(
                "⚙️  Available Opcodes (TEAL v{}):\n",
                target_version.as_u8()
            );

            let categories = categorize_opcodes(&opcodes);

            for (category, ops) in categories {
                println!("{category}:");
                for opcode in ops {
                    if show_details {
                        println!("  {} - {}", opcode.name, opcode.description);
                    } else {
                        print!("  {} ", opcode.name);
                    }
                }
                if !show_details {
                    println!();
                }
                println!();
            }

            println!("Total opcodes: {}", opcodes.len());
        }
        crate::cli::OutputFormat::Json => {
            let opcode_data: Vec<_> = opcodes
                .iter()
                .map(|op| {
                    serde_json::json!({
                        "name": op.name,
                        "description": op.description,
                        "category": op.category,
                        "version": op.min_version
                    })
                })
                .collect();

            let output = serde_json::json!({
                "version": target_version.as_u8(),
                "total_opcodes": opcodes.len(),
                "opcodes": opcode_data
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

/// Show system information
fn show_system_info(global: &GlobalOptions) -> Result<()> {
    if global.quiet {
        return Ok(());
    }

    match global.format {
        crate::cli::OutputFormat::Text => {
            println!("🖥️  System Information:\n");
            println!("  Rust AVM Version: {}", env!("CARGO_PKG_VERSION"));
            println!("  Build Target: {}", std::env::consts::ARCH);
            println!("  Operating System: {}", std::env::consts::OS);
            println!("  Latest TEAL Version: {}", TealVersion::latest().as_u8());

            // Runtime information
            println!("\n  Runtime:");
            println!("    Stack size: Configurable (default: 1000)");
            println!("    Cost budget: Configurable (default: 100000)");
            println!("    Memory limit: Available system memory");
        }
        crate::cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "avm_rs_version": env!("CARGO_PKG_VERSION"),
                "build_target": std::env::consts::ARCH,
                "operating_system": std::env::consts::OS,
                "latest_teal_version": TealVersion::latest().as_u8(),
                "runtime": {
                    "default_stack_size": 1000,
                    "default_cost_budget": 100000
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

/// Show general AVM information
fn show_general_info(global: &GlobalOptions) -> Result<()> {
    if global.quiet {
        return Ok(());
    }

    match global.format {
        crate::cli::OutputFormat::Text => {
            println!("🦀 Rust AVM - Algorand Virtual Machine\n");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Description: {}\n", env!("CARGO_PKG_DESCRIPTION"));

            println!("Supported Features:");
            println!("  ✅ TEAL versions 1-{}", TealVersion::latest().as_u8());
            println!("  ✅ Assembly and disassembly");
            println!("  ✅ Signature mode execution");
            println!("  ✅ Application mode execution");
            println!("  ✅ Built-in examples");
            println!("  ✅ Validation and analysis");
            println!("  🚧 Interactive REPL (coming soon)");
            println!("  🚧 Step-by-step debugging (coming soon)");

            println!("\nQuick Start:");
            println!("  avm-rs examples hello           # Run a simple example");
            println!("  avm-rs execute 'int 1; return' # Execute inline TEAL");
            println!("  avm-rs assemble program.teal   # Assemble TEAL to bytecode");
            println!("  avm-rs info --opcodes          # Show available opcodes");
        }
        crate::cli::OutputFormat::Json => {
            let output = serde_json::json!({
                "name": "Rust AVM",
                "version": env!("CARGO_PKG_VERSION"),
                "description": env!("CARGO_PKG_DESCRIPTION"),
                "supported_features": {
                    "teal_versions": format!("1-{}", TealVersion::latest().as_u8()),
                    "assembly": true,
                    "disassembly": true,
                    "signature_mode": true,
                    "application_mode": true,
                    "examples": true,
                    "validation": true,
                    "repl": false,
                    "stepping": false
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

/// Get version features description
fn get_version_features(version: TealVersion) -> String {
    match version {
        TealVersion::V1 => "Basic arithmetic and logic",
        TealVersion::V2 => "Added more opcodes",
        TealVersion::V3 => "Asset operations",
        TealVersion::V4 => "Subroutines and more crypto",
        TealVersion::V5 => "Inner transactions and applications",
        TealVersion::V6 => "Additional opcodes",
        TealVersion::V7 => "Enhanced inner transactions",
        TealVersion::V8 => "Box storage operations",
        TealVersion::V9 => "Extended box operations",
        TealVersion::V10 => "Elliptic curve operations",
        TealVersion::V11 => "MIMC hash and block operations",
    }
    .to_string()
}

/// Opcode information structure
#[derive(Debug, Clone)]
struct OpcodeInfo {
    name: &'static str,
    description: &'static str,
    category: &'static str,
    min_version: u8,
}

/// Get available opcodes for a version
fn get_available_opcodes(version: TealVersion) -> Vec<OpcodeInfo> {
    let mut opcodes = vec![
        // Basic opcodes (v1)
        OpcodeInfo {
            name: "int",
            description: "Push integer constant",
            category: "Constants",
            min_version: 1,
        },
        OpcodeInfo {
            name: "byte",
            description: "Push byte constant",
            category: "Constants",
            min_version: 1,
        },
        OpcodeInfo {
            name: "+",
            description: "Addition",
            category: "Arithmetic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "-",
            description: "Subtraction",
            category: "Arithmetic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "*",
            description: "Multiplication",
            category: "Arithmetic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "/",
            description: "Division",
            category: "Arithmetic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "%",
            description: "Modulo",
            category: "Arithmetic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "<",
            description: "Less than",
            category: "Comparison",
            min_version: 1,
        },
        OpcodeInfo {
            name: ">",
            description: "Greater than",
            category: "Comparison",
            min_version: 1,
        },
        OpcodeInfo {
            name: "<=",
            description: "Less or equal",
            category: "Comparison",
            min_version: 1,
        },
        OpcodeInfo {
            name: ">=",
            description: "Greater or equal",
            category: "Comparison",
            min_version: 1,
        },
        OpcodeInfo {
            name: "==",
            description: "Equal",
            category: "Comparison",
            min_version: 1,
        },
        OpcodeInfo {
            name: "!=",
            description: "Not equal",
            category: "Comparison",
            min_version: 1,
        },
        OpcodeInfo {
            name: "!",
            description: "Logical NOT",
            category: "Logic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "&&",
            description: "Logical AND",
            category: "Logic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "||",
            description: "Logical OR",
            category: "Logic",
            min_version: 1,
        },
        OpcodeInfo {
            name: "&",
            description: "Bitwise AND",
            category: "Bitwise",
            min_version: 1,
        },
        OpcodeInfo {
            name: "|",
            description: "Bitwise OR",
            category: "Bitwise",
            min_version: 1,
        },
        OpcodeInfo {
            name: "^",
            description: "Bitwise XOR",
            category: "Bitwise",
            min_version: 1,
        },
        OpcodeInfo {
            name: "~",
            description: "Bitwise NOT",
            category: "Bitwise",
            min_version: 1,
        },
        OpcodeInfo {
            name: "return",
            description: "Return from program",
            category: "Flow Control",
            min_version: 1,
        },
        OpcodeInfo {
            name: "pop",
            description: "Remove top stack item",
            category: "Stack",
            min_version: 1,
        },
        OpcodeInfo {
            name: "dup",
            description: "Duplicate top stack item",
            category: "Stack",
            min_version: 1,
        },
        // Crypto opcodes
        OpcodeInfo {
            name: "sha256",
            description: "SHA256 hash",
            category: "Cryptography",
            min_version: 1,
        },
        OpcodeInfo {
            name: "keccak256",
            description: "Keccak256 hash",
            category: "Cryptography",
            min_version: 1,
        },
        OpcodeInfo {
            name: "sha512_256",
            description: "SHA512/256 hash",
            category: "Cryptography",
            min_version: 1,
        },
        // Flow control
        OpcodeInfo {
            name: "bnz",
            description: "Branch if not zero",
            category: "Flow Control",
            min_version: 1,
        },
        OpcodeInfo {
            name: "bz",
            description: "Branch if zero",
            category: "Flow Control",
            min_version: 2,
        },
        OpcodeInfo {
            name: "b",
            description: "Unconditional branch",
            category: "Flow Control",
            min_version: 2,
        },
        // Subroutines (v4+)
        OpcodeInfo {
            name: "callsub",
            description: "Call subroutine",
            category: "Subroutines",
            min_version: 4,
        },
        OpcodeInfo {
            name: "retsub",
            description: "Return from subroutine",
            category: "Subroutines",
            min_version: 4,
        },
        // More stack operations
        OpcodeInfo {
            name: "dup2",
            description: "Duplicate top two items",
            category: "Stack",
            min_version: 2,
        },
        OpcodeInfo {
            name: "swap",
            description: "Swap top two items",
            category: "Stack",
            min_version: 3,
        },
        // Constants
        OpcodeInfo {
            name: "intcblock",
            description: "Integer constant block",
            category: "Constants",
            min_version: 1,
        },
        OpcodeInfo {
            name: "intc",
            description: "Integer constant",
            category: "Constants",
            min_version: 1,
        },
        OpcodeInfo {
            name: "bytecblock",
            description: "Byte constant block",
            category: "Constants",
            min_version: 1,
        },
        OpcodeInfo {
            name: "bytec",
            description: "Byte constant",
            category: "Constants",
            min_version: 1,
        },
    ];

    // Filter by version
    opcodes.retain(|op| op.min_version <= version.as_u8());
    opcodes.sort_by_key(|op| op.name);

    opcodes
}

/// Categorize opcodes by type
fn categorize_opcodes(opcodes: &[OpcodeInfo]) -> Vec<(&str, Vec<&OpcodeInfo>)> {
    use std::collections::HashMap;

    let mut categories: HashMap<&str, Vec<&OpcodeInfo>> = HashMap::new();

    for opcode in opcodes {
        categories.entry(opcode.category).or_default().push(opcode);
    }

    let mut sorted_categories: Vec<_> = categories.into_iter().collect();
    sorted_categories.sort_by_key(|(name, _)| *name);

    sorted_categories
}
