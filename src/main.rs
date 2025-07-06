use rust_avm::opcodes::get_standard_opcodes;
use rust_avm::state::MockLedger;
use rust_avm::types::RunMode;
use rust_avm::vm::{ExecutionConfig, VirtualMachine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust AVM Test");

    // Create a simple test program: pushint 1, return
    let program = vec![
        0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // pushint 1
        0x43, // return
    ];

    // Set up VM
    let mut vm = VirtualMachine::new();

    // Register all standard opcodes
    for spec in get_standard_opcodes() {
        vm.register_opcode(spec.opcode, spec);
    }

    // Create mock ledger
    let ledger = MockLedger::new();

    // Execute program
    let config = ExecutionConfig {
        run_mode: RunMode::Signature,
        cost_budget: 1000,
        version: 2,
        group_index: 0,
        group_size: 1,
    };
    let result = vm.execute(&program, config, &ledger)?;

    println!("Program executed successfully: {result}");

    Ok(())
}
