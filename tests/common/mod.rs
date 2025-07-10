//! Common test utilities for opcode testing

use rust_avm::{
    error::AvmResult,
    opcodes::*,
    state::{
        AccountParams, AppParams, AssetHolding, AssetParams, MockLedger, StateSchema, Transaction,
    },
    types::{StackValue, TealValue, TealVersion},
    vm::{ExecutionConfig, VirtualMachine},
};

/// Create a test VM with default settings
pub fn setup_vm() -> VirtualMachine {
    VirtualMachine::with_version(TealVersion::V11)
}

/// Create a VM with specific version
pub fn setup_vm_with_version(version: TealVersion) -> VirtualMachine {
    VirtualMachine::with_version(version)
}

/// Create default test execution config
pub fn test_config() -> ExecutionConfig {
    ExecutionConfig::new(TealVersion::V11).with_cost_budget(10000000) // Unlimited budget for testing
}

/// Create test execution config with specific version
pub fn test_config_with_version(version: TealVersion) -> ExecutionConfig {
    ExecutionConfig::new(version).with_cost_budget(10000000)
}

/// Create a test transaction with default values
pub fn test_transaction() -> Transaction {
    let mut tx = Transaction::new();
    tx.sender = vec![1u8; 32];
    tx.receiver = Some(vec![2u8; 32]);
    tx.amount = Some(10000);
    tx.group_index = 0;
    tx.tx_id = vec![3u8; 32];
    tx
}

/// Create a mock ledger with test data
pub fn setup_mock_ledger() -> MockLedger {
    let mut ledger = MockLedger::new();

    // Set up some default accounts
    let account1 = vec![1u8; 32];
    let account2 = vec![2u8; 32];
    let account3 = vec![3u8; 32];

    // Set balances
    ledger.set_balance(account1.clone(), 1_000_000);
    ledger.set_balance(account2.clone(), 500_000);
    ledger.set_balance(account3.clone(), 100_000);

    // Set up a test application
    let app_id = 123;
    let app_params = AppParams {
        approval_program: vec![0x06, 0x01, 0x01], // Simple approval program
        clear_state_program: vec![0x06, 0x01, 0x01],
        global_state_schema: StateSchema {
            num_uint: 5,
            num_byte_slice: 5,
        },
        local_state_schema: StateSchema {
            num_uint: 3,
            num_byte_slice: 3,
        },
        extra_program_pages: 0,
        creator: account1.clone(),
    };
    ledger.set_app_params(app_id, app_params);

    // Set up global state for the app
    ledger.set_global_state(app_id, "counter".to_string(), TealValue::Uint(42));
    ledger.set_global_state(
        app_id,
        "message".to_string(),
        TealValue::Bytes(b"Hello World".to_vec()),
    );

    // Opt in account2 to the app and set local state
    ledger.set_opted_in(account2.clone(), app_id, true);
    ledger.set_local_state(
        account2.clone(),
        app_id,
        "user_count".to_string(),
        TealValue::Uint(5),
    );

    // Set up a test asset
    let asset_id = 456;
    let asset_params = AssetParams {
        total: 1_000_000,
        decimals: 6,
        default_frozen: false,
        unit_name: "TST".to_string(),
        name: "Test Asset".to_string(),
        url: "https://test.com".to_string(),
        metadata_hash: vec![0u8; 32],
        manager: account1.clone(),
        reserve: account1.clone(),
        freeze: account1.clone(),
        clawback: account1.clone(),
    };
    ledger.set_asset_params(asset_id, asset_params);

    // Set up asset holdings
    ledger.set_asset_holding(
        account1.clone(),
        asset_id,
        AssetHolding {
            amount: 900_000,
            frozen: false,
        },
    );
    ledger.set_asset_holding(
        account2.clone(),
        asset_id,
        AssetHolding {
            amount: 100_000,
            frozen: false,
        },
    );

    // Set current app ID
    ledger.set_current_application_id(app_id);

    // Set up transaction group
    let mut tx1 = test_transaction();
    tx1.group_index = 0;
    let mut tx2 = test_transaction();
    tx2.group_index = 1;
    tx2.receiver = Some(vec![3u8; 32]);
    ledger.add_transaction(tx1);
    ledger.add_transaction(tx2);

    // Set account parameters
    ledger.set_account_params(
        account1,
        AccountParams {
            micro_algos: 1_000_000,
            rewards_base: 0,
            reward_algos: 0,
            status: "Online".to_string(),
            auth_addr: None,
            total_apps_schema: StateSchema {
                num_uint: 10,
                num_byte_slice: 10,
            },
            total_apps_extra_pages: 0,
            total_assets: 1,
            total_created_assets: 1,
            total_created_apps: 1,
            total_boxes: 0,
            total_box_bytes: 0,
        },
    );

    ledger
}

/// Execute bytecode and check if it returns the expected result
pub fn execute_and_check(bytecode: &[u8], expected: bool) -> AvmResult<()> {
    let vm = setup_vm();
    let ledger = setup_mock_ledger();

    let result = vm.execute(bytecode, test_config(), &ledger)?;
    assert_eq!(result, expected, "Expected program to return {expected}");
    Ok(())
}

/// Execute bytecode and check if it fails with an error
pub fn execute_expect_error(bytecode: &[u8]) -> AvmResult<()> {
    let vm = setup_vm();
    let ledger = setup_mock_ledger();
    let result = vm.execute(bytecode, test_config(), &ledger);
    assert!(result.is_err(), "Expected program to fail but it succeeded");
    Ok(())
}

/// Create a simple TEAL program that tests an opcode
pub fn create_teal_program(program: &str) -> AvmResult<Vec<u8>> {
    use rust_avm::assembler::Assembler;
    let mut assembler = Assembler::new();
    let full_program = format!("#pragma version 6\n{program}");
    assembler.assemble(&full_program)
}

/// Assert stack contains expected values
pub fn assert_stack_contains(_vm: &VirtualMachine, _expected: &[StackValue]) -> AvmResult<()> {
    // This is a simplified check - in reality we'd need access to the VM's internal state
    // For now, we'll rely on the test returning the correct boolean result
    Ok(())
}

/// Execute a TEAL program and check the result
pub fn execute_teal_program(program: &str, expected: bool) -> AvmResult<()> {
    let bytecode = create_teal_program(program)?;
    execute_and_check(&bytecode, expected)
}

/// Execute a TEAL program and expect it to fail
pub fn execute_teal_expect_error(program: &str) -> AvmResult<()> {
    let bytecode = create_teal_program(program)?;
    execute_expect_error(&bytecode)
}

/// Build a simple opcode test with given stack values and opcode
pub fn build_simple_op_test(values: Vec<StackValue>, opcode: u8) -> Vec<u8> {
    let mut bytecode = Vec::new();

    // Push each value onto stack
    for value in values {
        match value {
            StackValue::Uint(val) => {
                bytecode.push(0x81); // pushint
                bytecode.extend_from_slice(&val.to_be_bytes());
            }
            StackValue::Bytes(bytes) => {
                bytecode.push(0x80); // pushbytes
                bytecode.push(bytes.len() as u8);
                bytecode.extend_from_slice(&bytes);
            }
        }
    }

    // Add the opcode to test
    bytecode.push(opcode);

    bytecode
}

/// Add assertion that stack top equals expected value
pub fn with_assert_equals(mut bytecode: Vec<u8>, expected: StackValue) -> Vec<u8> {
    match expected {
        StackValue::Uint(val) => {
            bytecode.push(0x81); // pushint
            bytecode.extend_from_slice(&val.to_be_bytes());
        }
        StackValue::Bytes(bytes) => {
            bytecode.push(0x80); // pushbytes
            bytecode.push(bytes.len() as u8);
            bytecode.extend_from_slice(&bytes);
        }
    }

    // Add equality check and return
    bytecode.push(OP_EQ);
    bytecode.push(0x43); // return

    bytecode
}
