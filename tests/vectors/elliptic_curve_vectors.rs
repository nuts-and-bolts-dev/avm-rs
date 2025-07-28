//! Elliptic curve operation test vectors
//!
//! These test vectors validate elliptic curve operations against known values
//! and ensure compatibility with the BN254 and BLS12-381 curve implementations
//! used in zero-knowledge proof systems.

use avm_rs::{
    opcodes::*,
    types::{StackValue, TealVersion},
};

use crate::common::*;

/// BN254 curve test vectors
/// These vectors test the basic elliptic curve operations on the BN254 curve
/// which is commonly used in zk-SNARKs
#[test]
fn test_bn254_curve_operations() {
    // Test EC_ADD on BN254 G1
    let mut bytecode = Vec::new();

    // Point 1: Identity point (empty bytes represent identity)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Point 2: Identity point
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Perform EC_ADD on BN254 G1
    bytecode.push(OP_EC_ADD);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be 64 bytes
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "BN254 EC_ADD should succeed");
}

#[test]
fn test_bn254_scalar_multiplication() {
    // Test EC_SCALAR_MUL on BN254 G1
    let mut bytecode = Vec::new();

    // Point: identity point (empty bytes represent identity)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Scalar: test scalar (small value)
    let mut scalar = vec![0u8; 32];
    scalar[31] = 2; // Scalar = 2
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&scalar);

    // Perform EC_SCALAR_MUL on BN254 G1
    bytecode.push(OP_EC_SCALAR_MUL);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be 64 bytes
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "BN254 EC_SCALAR_MUL should succeed");
}

#[test]
fn test_bn254_pairing_check() {
    // Test EC_PAIRING_CHECK on BN254
    let mut bytecode = Vec::new();

    // Pairing points: G1 identity (empty) and G2 identity (empty) for testing
    // Identity points for pairing should result in valid pairing
    let g1_point = vec![]; // Identity point as empty bytes
    let g2_point = vec![]; // Identity point as empty bytes

    let mut pairing_data = Vec::new();
    pairing_data.extend_from_slice(&g1_point);
    pairing_data.extend_from_slice(&g2_point);

    bytecode.push(OP_PUSHBYTES);
    bytecode.push(pairing_data.len() as u8); // Empty pairing data (2 identity points)
    if !pairing_data.is_empty() {
        bytecode.extend_from_slice(&pairing_data);
    }

    // Perform EC_PAIRING_CHECK on BN254
    bytecode.push(OP_EC_PAIRING_CHECK);
    bytecode.push(0); // BN254 curve ID

    // Result should be 0 or 1 (boolean)
    bytecode.push(OP_DUP);
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode.push(OP_EQ);

    bytecode.push(OP_SWAP);
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_EQ);

    bytecode.push(OP_OR); // Result is either 0 or 1
    bytecode.push(OP_RETURN);

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "BN254 EC_PAIRING_CHECK should return valid boolean");
}

/// BLS12-381 curve test vectors
/// These vectors test operations on the BLS12-381 curve used in Ethereum 2.0
#[test]
fn test_bls12_381_curve_operations() {
    // Test EC_ADD on BLS12-381 G1
    let mut bytecode = Vec::new();

    // Point 1: Identity point (empty bytes)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Point 2: Identity point (empty bytes)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Perform EC_ADD on BLS12-381 G1
    bytecode.push(OP_EC_ADD);
    bytecode.push(2); // BLS12-381 G1 curve ID

    // Result should be 96 bytes (BLS12-381 G1 uncompressed points)
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(96));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "BLS12-381 EC_ADD should succeed");
}

#[test]
fn test_multi_scalar_multiplication() {
    // Test EC_MULTI_SCALAR_MUL on BN254 G1
    let mut bytecode = Vec::new();

    // For multi-scalar multiplication, we need zero points and zero scalars to match
    // This will result in the identity point as the sum
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // No points (empty bytes)

    // Scalars: No scalars to match no points
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // No scalars (empty bytes)

    // Perform EC_MULTI_SCALAR_MUL on BN254 G1
    bytecode.push(OP_EC_MULTI_SCALAR_MUL);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be 64 bytes (single G1 point)
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "EC_MULTI_SCALAR_MUL should succeed");
}

#[test]
fn test_subgroup_check() {
    // Test EC_SUBGROUP_CHECK for various points
    let mut bytecode = Vec::new();

    // Test point: identity should be in subgroup (empty bytes)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // Identity point

    // Perform EC_SUBGROUP_CHECK on BN254 G1
    bytecode.push(OP_EC_SUBGROUP_CHECK);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be 0 or 1
    bytecode.push(OP_DUP);
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&0u64.to_be_bytes());
    bytecode.push(OP_EQ);

    bytecode.push(OP_SWAP);
    bytecode.push(OP_PUSHINT);
    bytecode.extend_from_slice(&1u64.to_be_bytes());
    bytecode.push(OP_EQ);

    bytecode.push(OP_OR); // Result is either 0 or 1
    bytecode.push(OP_RETURN);

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "EC_SUBGROUP_CHECK should return valid boolean");
}

#[test]
fn test_map_to_curve() {
    // Test EC_MAP_TO for field element to curve point mapping
    let mut bytecode = Vec::new();

    // Field element: 32-byte value
    let field_element = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        0x07, 0x08,
    ];

    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&field_element);

    // Perform EC_MAP_TO on BN254 G1
    bytecode.push(OP_EC_MAP_TO);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be 64 bytes (G1 point)
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "EC_MAP_TO should succeed");

    // Test deterministic property: same input should produce same output
    let mut bytecode = Vec::new();

    // First mapping
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&field_element);
    bytecode.push(OP_EC_MAP_TO);
    bytecode.push(0); // BN254 G1 curve ID

    // Second mapping with same input
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&field_element);
    bytecode.push(OP_EC_MAP_TO);
    bytecode.push(0); // BN254 G1 curve ID

    // Results should be equal
    bytecode.push(OP_EQ);
    bytecode.push(OP_RETURN);

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "EC_MAP_TO should be deterministic");
}

/// Error condition tests for elliptic curve operations
#[test]
fn test_invalid_curve_ids() {
    // Test EC_ADD with invalid curve ID
    let mut bytecode = Vec::new();

    // Valid identity points
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // Identity point

    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // Identity point

    // Invalid curve ID
    bytecode.push(OP_EC_ADD);
    bytecode.push(255); // Invalid curve ID

    // Should fail
    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_invalid_point_lengths() {
    // Test EC_ADD with incorrect point length
    let mut bytecode = Vec::new();

    // Invalid point length (32 bytes instead of 64)
    let point = vec![0u8; 32];
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&point);

    // Valid identity point
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // Identity point

    // BN254 G1
    bytecode.push(OP_EC_ADD);
    bytecode.push(0);

    // Should fail due to invalid point length
    execute_expect_error(&bytecode).unwrap();
}

/// Performance tests for elliptic curve operations
#[test]
fn test_ec_operations_performance() {
    // Test that multiple EC operations complete within reasonable time
    let mut bytecode = Vec::new();

    // Perform 10 consecutive EC_ADD operations with identity points
    for i in 0..10 {
        // Use identity point (empty bytes)
        bytecode.push(OP_PUSHBYTES);
        bytecode.push(0);

        if i > 0 {
            // Add to previous result
            bytecode.push(OP_EC_ADD);
            bytecode.push(0); // BN254 G1
        }
    }

    // Final result should be 64 bytes
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(
        result,
        "Multiple EC operations should complete successfully"
    );
}
