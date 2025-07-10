use rust_avm::state::MockLedger;
use rust_avm::{TealVersion, VirtualMachine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust AVM Test");

    // Create a simple test program: pushint 1, return
    let program = vec![
        0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // pushint 1
        0x43, // return
    ];

    // Set up VM with ergonomic API
    let vm = VirtualMachine::with_version(TealVersion::V2);

    // Create mock ledger
    let mut ledger = MockLedger::new();

    // Execute program with simple API
    let result = vm.execute_simple(&program, TealVersion::V2, &mut ledger)?;

    println!("Program executed successfully: {result}");

    Ok(())
}
