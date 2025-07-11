//! Tests for transaction field access opcodes

use avm_rs::{opcodes::*, types::StackValue};

use crate::common::*;

#[test]
fn test_op_txn_sender() {
    // Test accessing sender field
    let mut bytecode = Vec::new();
    bytecode.push(OP_TXN);
    bytecode.push(0); // Sender field ID
    bytecode.push(OP_LEN); // Check it's 32 bytes
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_txn_fee() {
    // Test accessing fee field
    let mut bytecode = Vec::new();
    bytecode.push(OP_TXN);
    bytecode.push(1); // Fee field ID
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1000)); // Default test transaction fee

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_txn_amount() {
    // Test accessing amount field
    let mut bytecode = Vec::new();
    bytecode.push(OP_TXN);
    bytecode.push(8); // Amount field ID
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1000000)); // Current transaction amount (default payment)

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_txn_type_enum() {
    // Test accessing type field
    let mut bytecode = Vec::new();
    bytecode.push(OP_TXN);
    bytecode.push(16); // TypeEnum field ID
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1)); // Payment type

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_txn_invalid_field() {
    // Test accessing invalid field
    let mut bytecode = Vec::new();
    bytecode.push(OP_TXN);
    bytecode.push(255); // Invalid field ID

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_txna_application_args() {
    // Test accessing application args array
    let mut ledger = setup_mock_ledger();
    let mut tx = test_transaction();
    tx.application_args = vec![b"arg0".to_vec(), b"arg1".to_vec(), b"arg2".to_vec()];
    ledger.clear_transactions();
    ledger.add_transaction(tx);
    ledger.set_current_transaction_index(0);

    let mut bytecode = Vec::new();
    bytecode.push(OP_TXNA);
    bytecode.push(26); // ApplicationArgs field ID
    bytecode.push(1); // Index 1
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"arg1".to_vec()));

    let vm = setup_vm();
    let result = vm.execute(&bytecode, test_config(), &mut ledger).unwrap();
    assert!(result);
}

#[test]
fn test_op_txna_out_of_bounds() {
    // Test accessing out of bounds array index
    let mut bytecode = Vec::new();
    bytecode.push(OP_TXNA);
    bytecode.push(26); // ApplicationArgs field ID
    bytecode.push(10); // Out of bounds index

    // Should return empty bytes
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(vec![]));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_txnas_with_stack_index() {
    // Test accessing array with stack index
    let mut ledger = setup_mock_ledger();
    let mut tx = test_transaction();
    tx.application_args = vec![b"first".to_vec(), b"second".to_vec()];
    ledger.clear_transactions();
    ledger.add_transaction(tx);
    ledger.set_current_transaction_index(0);

    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // Index 1
    bytecode.push(OP_TXNAS);
    bytecode.push(26); // ApplicationArgs field ID
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"second".to_vec()));

    let vm = setup_vm();
    let result = vm.execute(&bytecode, test_config(), &mut ledger).unwrap();
    assert!(result);
}

#[test]
fn test_op_gtxn_group_index() {
    // Test accessing transaction in group - let's test with the correct amount
    let mut bytecode = Vec::new();
    bytecode.push(OP_GTXN);
    bytecode.push(0); // Group index 0 (test first transaction)
    bytecode.push(8); // Amount field
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1000000)); // Original default amount

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let config = test_config().with_group(0, 3); // Set group size to 3 to match mock ledger
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result);
}

#[test]
fn test_op_gtxn_out_of_group() {
    // Test accessing invalid group index
    let mut bytecode = Vec::new();
    bytecode.push(OP_GTXN);
    bytecode.push(10); // Out of bounds group index
    bytecode.push(8); // Amount field

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_gtxna_group_array() {
    // Test accessing array field in group transaction
    let mut ledger = setup_mock_ledger();
    let mut tx1 = test_transaction();
    tx1.application_args = vec![b"tx1arg".to_vec()];
    let mut tx2 = test_transaction();
    tx2.application_args = vec![b"tx2arg0".to_vec(), b"tx2arg1".to_vec()];
    ledger.clear_transactions();
    ledger.add_transaction(tx1);
    ledger.add_transaction(tx2);

    let mut bytecode = Vec::new();
    bytecode.push(OP_GTXNA);
    bytecode.push(1); // Group index 1
    bytecode.push(26); // ApplicationArgs field
    bytecode.push(0); // Array index 0
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"tx2arg0".to_vec()));

    let vm = setup_vm();
    let config = test_config().with_group(0, 2); // Set group size to 2 to match cleared+added transactions
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result);
}

#[test]
fn test_op_gtxns_with_stack_index() {
    // Test accessing group transaction with stack index
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // Group index 1
    bytecode.push(OP_GTXNS);
    bytecode.push(7); // Receiver field
    bytecode.push(OP_LEN); // Check receiver is 32 bytes
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    let vm = setup_vm();
    let mut ledger = setup_mock_ledger();
    let config = test_config().with_group(0, 3); // Set group size to 3 to match mock ledger
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result);
}

#[test]
fn test_op_gtxnsa_with_stack_indices() {
    // Test accessing group transaction array with stack indices
    let mut ledger = setup_mock_ledger();
    let tx1 = test_transaction();
    let mut tx2 = test_transaction();
    tx2.application_args = vec![b"arg0".to_vec(), b"arg1".to_vec(), b"arg2".to_vec()];
    ledger.clear_transactions();
    ledger.add_transaction(tx1);
    ledger.add_transaction(tx2);

    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&1u64.to_be_bytes()); // Group index
    bytecode.push(OP_PUSHINT); // pushint
    bytecode.extend_from_slice(&2u64.to_be_bytes()); // Array index
    bytecode.push(OP_GTXNSA);
    bytecode.push(26); // ApplicationArgs field
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(b"arg2".to_vec()));

    let vm = setup_vm();
    let config = test_config().with_group(0, 2); // Set group size to 2 to match cleared+added transactions
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result);
}

#[test]
fn test_op_global_min_txn_fee() {
    // Test accessing global MinTxnFee
    let mut bytecode = Vec::new();
    bytecode.push(OP_GLOBAL);
    bytecode.push(0); // MinTxnFee field
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1000)); // Mock ledger default

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_global_min_balance() {
    // Test accessing global MinBalance
    let mut bytecode = Vec::new();
    bytecode.push(OP_GLOBAL);
    bytecode.push(1); // MinBalance field
    bytecode = with_assert_equals(bytecode, StackValue::Uint(100000)); // Mock ledger default

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_global_group_size() {
    // Test accessing global GroupSize
    let mut bytecode = Vec::new();
    bytecode.push(OP_GLOBAL);
    bytecode.push(4); // GroupSize field
    bytecode = with_assert_equals(bytecode, StackValue::Uint(3)); // Mock ledger has 3 txns (1 default + 2 added)

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_global_current_application_id() {
    // Test accessing global CurrentApplicationID
    let mut bytecode = Vec::new();
    bytecode.push(OP_GLOBAL);
    bytecode.push(8); // CurrentApplicationID field
    bytecode = with_assert_equals(bytecode, StackValue::Uint(123)); // Mock ledger app ID

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_global_zero_address() {
    // Test accessing global ZeroAddress
    let mut bytecode = Vec::new();
    bytecode.push(OP_GLOBAL);
    bytecode.push(3); // ZeroAddress field
    bytecode.push(OP_LEN); // Should be 32 bytes
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_global_invalid_field() {
    // Test accessing invalid global field
    let mut bytecode = Vec::new();
    bytecode.push(OP_GLOBAL);
    bytecode.push(255); // Invalid field ID

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_transaction_field_combinations() {
    // Test multiple transaction field accesses
    let mut bytecode = Vec::new();

    // Get sender
    bytecode.push(OP_TXN);
    bytecode.push(0); // Sender

    // Get receiver
    bytecode.push(OP_TXN);
    bytecode.push(7); // Receiver

    // Compare them (should be different)
    bytecode.push(OP_EQ);
    bytecode.push(OP_NOT); // They should NOT be equal
    bytecode.push(OP_RETURN); // return

    execute_and_check(&bytecode, true).unwrap();
}
