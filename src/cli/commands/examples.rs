//! Examples command implementation

use crate::cli::{ExamplesCommand, GlobalOptions};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Handle the examples command
pub fn handle(cmd: ExamplesCommand, global: &GlobalOptions) -> Result<()> {
    let examples = get_built_in_examples();

    if cmd.list {
        list_examples(&examples, global)?;
        return Ok(());
    }

    if let Some(example_name) = &cmd.example {
        if let Some(example) = examples.get(example_name) {
            if cmd.show {
                show_example(example_name, example, global)?;
            }

            if cmd.run {
                run_example(example_name, example, global)?;
            }

            if !cmd.show && !cmd.run {
                // Default behavior: show and run
                show_example(example_name, example, global)?;
                println!("\n{}\n", "=".repeat(50));
                run_example(example_name, example, global)?;
            }
        } else {
            return Err(anyhow!(
                "Example '{}' not found. Use --list to see available examples.",
                example_name
            ));
        }
    } else {
        // No specific example requested, show list
        list_examples(&examples, global)?;
    }

    Ok(())
}

/// Built-in example structure
#[derive(Debug, Clone)]
struct Example {
    description: String,
    teal_code: String,
    explanation: String,
}

/// Get all built-in examples
fn get_built_in_examples() -> HashMap<String, Example> {
    let mut examples = HashMap::new();

    examples.insert("hello".to_string(), Example {
        description: "Simple hello world program".to_string(),
        teal_code: r#"#pragma version 11
// Simple program that always succeeds
int 1
return"#.to_string(),
        explanation: "This is the simplest TEAL program. It pushes 1 onto the stack and returns, indicating success.".to_string(),
    });

    examples.insert(
        "arithmetic".to_string(),
        Example {
            description: "Basic arithmetic operations".to_string(),
            teal_code: r#"#pragma version 11
// Calculate: (10 + 20) * 3 / 2
int 10
int 20
+           // Stack: [30]
int 3
*           // Stack: [90] 
int 2
/           // Stack: [45]
int 45
==          // Verify result
return"#
                .to_string(),
            explanation: "Demonstrates basic arithmetic operations and stack manipulation."
                .to_string(),
        },
    );

    examples.insert(
        "conditional".to_string(),
        Example {
            description: "Conditional logic with branching".to_string(),
            teal_code: r#"#pragma version 11
// Check if a number is greater than 5
int 7
int 5
>           // Check if 7 > 5
bnz success // Branch if true

// Failure path
int 0
return

success:
int 1
return"#
                .to_string(),
            explanation: "Shows how to use conditional branching with bnz (branch if not zero)."
                .to_string(),
        },
    );

    examples.insert(
        "subroutine".to_string(),
        Example {
            description: "Subroutine usage example".to_string(),
            teal_code: r#"#pragma version 11
// Main program
int 5
int 3
callsub add_numbers
int 8
==
return

// Subroutine to add two numbers
add_numbers:
+
retsub"#
                .to_string(),
            explanation: "Demonstrates how to define and call subroutines in TEAL.".to_string(),
        },
    );

    examples.insert(
        "crypto".to_string(),
        Example {
            description: "Cryptographic hash example".to_string(),
            teal_code: r#"#pragma version 11
// Hash the string "hello" and verify
byte "hello"
sha256
byte base64 LPJNul+wow4m6DsqxbninhsWHlwfp0JecwQzYpOLmCQ=
==
return"#
                .to_string(),
            explanation:
                "Shows how to use the SHA256 hash function and compare against expected values."
                    .to_string(),
        },
    );

    examples.insert(
        "minimum".to_string(),
        Example {
            description: "Find minimum of two numbers".to_string(),
            teal_code: r#"#pragma version 11
// Find min(42, 17)
int 42
int 17
dup2        // Duplicate both values: [42, 17, 42, 17]
<           // Compare: [42, 17, 0] (42 < 17 is false)
bnz first_smaller

// Second number is smaller or equal
swap        // [17, 42]
pop         // [17]
b done

first_smaller:
pop         // Remove the larger number

done:
int 17      // Expected result
==
return"#
                .to_string(),
            explanation: "Implements a min(a,b) function using stack operations and branching."
                .to_string(),
        },
    );

    examples
}

/// List all available examples
fn list_examples(examples: &HashMap<String, Example>, global: &GlobalOptions) -> Result<()> {
    if !global.quiet {
        match global.format {
            crate::cli::OutputFormat::Text => {
                println!("📚 Available Examples:\n");

                let mut sorted_examples: Vec<_> = examples.iter().collect();
                sorted_examples.sort_by_key(|(name, _)| *name);

                for (name, example) in sorted_examples {
                    println!("  {} - {}", name, example.description);
                }

                println!("\nUsage:");
                println!("  rust-avm examples <name>          # Show and run example");
                println!("  rust-avm examples <name> --show   # Show source only");
                println!("  rust-avm examples <name> --run    # Run example only");
            }
            crate::cli::OutputFormat::Json => {
                let example_list: Vec<_> = examples
                    .iter()
                    .map(|(name, example)| {
                        serde_json::json!({
                            "name": name,
                            "description": example.description
                        })
                    })
                    .collect();

                let output = serde_json::json!({
                    "examples": example_list
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
    }

    Ok(())
}

/// Show example source code
fn show_example(name: &str, example: &Example, global: &GlobalOptions) -> Result<()> {
    if !global.quiet {
        match global.format {
            crate::cli::OutputFormat::Text => {
                println!("📖 Example: {name}");
                println!("Description: {}\n", example.description);
                println!("TEAL Source:");
                println!("{}", "─".repeat(40));
                println!("{}", example.teal_code);
                println!("{}", "─".repeat(40));
                println!("\nExplanation:");
                println!("{}", example.explanation);
            }
            crate::cli::OutputFormat::Json => {
                let output = serde_json::json!({
                    "name": name,
                    "description": example.description,
                    "teal_code": example.teal_code,
                    "explanation": example.explanation
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
    }

    Ok(())
}

/// Run example and show execution result
fn run_example(name: &str, example: &Example, global: &GlobalOptions) -> Result<()> {
    if !global.quiet {
        match global.format {
            crate::cli::OutputFormat::Text => {
                println!("🚀 Running example: {name}");
            }
            crate::cli::OutputFormat::Json => {
                // JSON output will be handled by the execute command
            }
        }
    }

    // Use the execute command to run the example
    let execute_cmd = crate::cli::ExecuteCommand {
        input: example.teal_code.clone(),
        input_type: crate::cli::InputType::Inline,
        version: Some(11),
        mode: crate::cli::ExecutionMode::Signature,
        budget: 100000,
        step: false,
        show_stack: false,
        ledger: None,
        transaction: None,
        args: vec![],
    };

    crate::cli::commands::execute::handle(execute_cmd, global)
}
