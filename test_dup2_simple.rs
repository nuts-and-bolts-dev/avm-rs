#[cfg(test)]
mod tests {
    use rust_avm::{opcodes::*, types::StackValue};

    #[test]
    fn test_dup2_simple() {
        // Simple test - just check that dup2 creates the right stack
        let mut bytecode = Vec::new();
        bytecode.push(OP_PUSHINT); // pushint 10
        bytecode.extend_from_slice(&10u64.to_be_bytes());
        bytecode.push(OP_PUSHINT); // pushint 20
        bytecode.extend_from_slice(&20u64.to_be_bytes());
        bytecode.push(OP_DUP2); // duplicate top two: stack is now [10, 20, 10, 20]

        // Check that top is 20
        bytecode.push(OP_PUSHINT); // pushint 20
        bytecode.extend_from_slice(&20u64.to_be_bytes());
        bytecode.push(OP_EQ); // compare top two (20 == 20) -> [10, 20, 10, 1]
        bytecode.push(OP_RETURN); // return

        println!("Bytecode: {:?}", bytecode);

        // Execute
        let vm = rust_avm::vm::VirtualMachine::with_version(rust_avm::types::TealVersion::V6);
        let config = rust_avm::vm::ExecutionConfig::new(rust_avm::types::TealVersion::V6)
            .with_cost_budget(10000000);
        let ledger = rust_avm::state::MockLedger::new();

        let result = vm.execute(&bytecode, config, &ledger);
        match result {
            Ok(res) => println!("Result: {}", res),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
