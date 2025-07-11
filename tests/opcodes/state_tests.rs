//! Tests for state access opcodes (application mode only)

use rust_avm::{
    opcodes::*,
    types::{RunMode, StackValue},
    vm::ExecutionConfig,
};

use crate::common::*;

fn app_mode_config() -> ExecutionConfig {
    test_config().with_run_mode(RunMode::Application)
}

#[test]
fn test_op_app_global_get() {
    // Test getting existing global state
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(7); // length
    bytecode.extend_from_slice(b"counter");
    bytecode.push(OP_APP_GLOBAL_GET);

    // app_global_get returns two values: [value, exists_flag] with exists_flag on top
    // We want to test exists_flag, so swap to move value to top and pop it
    bytecode.push(OP_SWAP); // [exists_flag, value]  
    bytecode.push(OP_POP); // [exists_flag]
    // Check exists flag is 1
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1));

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);

    // Test getting the actual value
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(7); // length
    bytecode.extend_from_slice(b"counter");
    bytecode.push(OP_APP_GLOBAL_GET);
    bytecode.push(OP_POP); // Pop exists flag
    // Value should be 42 (from mock ledger)
    bytecode = with_assert_equals(bytecode, StackValue::Uint(42));

    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_app_global_get_nonexistent() {
    // Test getting non-existent global state
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(7); // length
    bytecode.extend_from_slice(b"missing");
    bytecode.push(OP_APP_GLOBAL_GET);

    // Should return 0 for both value and exists flag
    bytecode.push(OP_PLUS); // Add them together
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0));

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_app_global_get_ex() {
    // Test getting global state from specific app
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&123u64.to_be_bytes()); // App ID
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(7); // length
    bytecode.extend_from_slice(b"message");
    bytecode.push(OP_APP_GLOBAL_GET_EX);

    // app_global_get_ex returns two values: [value, exists_flag] with exists_flag on top
    // We want to test exists_flag, so swap to move value to top and pop it
    bytecode.push(OP_SWAP); // [exists_flag, value]  
    bytecode.push(OP_POP); // [exists_flag]
    // Check exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1));

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_app_global_put() {
    // Test putting global state (currently returns error in implementation)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(3); // length
    bytecode.extend_from_slice(b"key");
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&100u64.to_be_bytes());
    bytecode.push(OP_APP_GLOBAL_PUT);

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm.execute(&bytecode, app_mode_config(), &mut ledger);

    // Currently not implemented, should error
    assert!(result.is_err());
}

#[test]
fn test_op_app_global_del() {
    // Test deleting global state (currently returns error in implementation)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(3); // length
    bytecode.extend_from_slice(b"key");
    bytecode.push(OP_APP_GLOBAL_DEL);

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm.execute(&bytecode, app_mode_config(), &mut ledger);

    // Currently not implemented, should error
    assert!(result.is_err());
}

#[test]
fn test_op_app_local_get() {
    // Test getting local state
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[2u8; 32]); // Account 2
    bytecode.push(OP_PUSHBYTES); // pushbytes (key)
    bytecode.push(10); // length
    bytecode.extend_from_slice(b"user_count");
    bytecode.push(OP_APP_LOCAL_GET);

    // app_local_get returns two values: [value, exists_flag] with exists_flag on top
    // We want to test exists_flag, so swap to move value to top and pop it
    bytecode.push(OP_SWAP); // [exists_flag, value]  
    bytecode.push(OP_POP); // [exists_flag]
    // Check exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1));

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_app_local_get_ex() {
    // Test getting local state from specific app
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[2u8; 32]); // Account 2
    bytecode.push(OP_PUSHINT); // pushint (app ID)
    bytecode.extend_from_slice(&123u64.to_be_bytes());
    bytecode.push(OP_PUSHBYTES); // pushbytes (key)
    bytecode.push(10); // length
    bytecode.extend_from_slice(b"user_count");
    bytecode.push(OP_APP_LOCAL_GET_EX);

    // Pop exists flag and check value
    bytecode.push(OP_POP);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(5)); // Mock value

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_app_opted_in() {
    // Test checking if account opted into app
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[2u8; 32]); // Account 2 (opted in)
    bytecode.push(OP_PUSHINT); // pushint (app ID)
    bytecode.extend_from_slice(&123u64.to_be_bytes());
    bytecode.push(OP_APP_OPTED_IN);

    bytecode = with_assert_equals(bytecode, StackValue::Uint(1)); // Opted in

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);

    // Test account not opted in
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[3u8; 32]); // Account 3 (not opted in)
    bytecode.push(OP_PUSHINT); // pushint (app ID)
    bytecode.extend_from_slice(&123u64.to_be_bytes());
    bytecode.push(OP_APP_OPTED_IN);

    bytecode = with_assert_equals(bytecode, StackValue::Uint(0)); // Not opted in

    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_balance() {
    // Test getting account balance
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]); // Account 1
    bytecode.push(OP_BALANCE);

    bytecode = with_assert_equals(bytecode, StackValue::Uint(1_000_000)); // Mock balance

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_min_balance() {
    // Test getting minimum balance
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]);
    bytecode.push(OP_MIN_BALANCE);

    bytecode = with_assert_equals(bytecode, StackValue::Uint(100_000)); // Mock min balance

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_asset_holding_get() {
    // Test getting asset holding - balance field
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]);
    bytecode.push(OP_PUSHINT); // pushint (asset ID)
    bytecode.extend_from_slice(&456u64.to_be_bytes());
    bytecode.push(OP_ASSET_HOLDING_GET);
    bytecode.push(0); // AssetBalance field

    // asset_holding_get returns two values: [value, exists_flag] with exists_flag on top
    // We want to test exists_flag, so swap to move value to top and pop it
    bytecode.push(OP_SWAP); // [exists_flag, value]  
    bytecode.push(OP_POP); // [exists_flag]
    // Check exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1));

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);

    // Test frozen field
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]);
    bytecode.push(OP_PUSHINT); // pushint (asset ID)
    bytecode.extend_from_slice(&456u64.to_be_bytes());
    bytecode.push(OP_ASSET_HOLDING_GET);
    bytecode.push(1); // AssetFrozen field

    bytecode.push(OP_POP); // Pop exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0)); // Not frozen

    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_asset_params_get() {
    // Test getting asset parameters
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint (asset ID)
    bytecode.extend_from_slice(&456u64.to_be_bytes());
    bytecode.push(OP_ASSET_PARAMS_GET);
    bytecode.push(0); // AssetTotal field

    bytecode.push(OP_POP); // Pop exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1_000_000)); // Total supply

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);

    // Test decimals field
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint (asset ID)
    bytecode.extend_from_slice(&456u64.to_be_bytes());
    bytecode.push(OP_ASSET_PARAMS_GET);
    bytecode.push(1); // AssetDecimals field

    bytecode.push(OP_POP); // Pop exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(6)); // 6 decimals

    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_app_params_get() {
    // Test getting app parameters
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint (app ID)
    bytecode.extend_from_slice(&123u64.to_be_bytes());
    bytecode.push(OP_APP_PARAMS_GET);
    bytecode.push(2); // AppGlobalNumUint field

    bytecode.push(OP_POP); // Pop exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(5)); // Global uints

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_op_acct_params_get() {
    // Test getting account parameters
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes (account)
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]);
    bytecode.push(OP_ACCT_PARAMS_GET);
    bytecode.push(0); // AcctBalance field

    bytecode.push(OP_POP); // Pop exists flag
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1_000_000)); // Balance

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let result = vm
        .execute(&bytecode, app_mode_config(), &mut ledger)
        .unwrap();
    assert!(result);
}

#[test]
fn test_state_opcodes_require_app_mode() {
    // Test that state opcodes fail in signature mode
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES); // pushbytes
    bytecode.push(7); // length
    bytecode.extend_from_slice(b"counter");
    bytecode.push(OP_APP_GLOBAL_GET);

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let sig_mode_config = test_config().with_run_mode(RunMode::Signature);
    let result = vm.execute(&bytecode, sig_mode_config, &mut ledger);

    // Should fail in signature mode
    assert!(result.is_err());
}
