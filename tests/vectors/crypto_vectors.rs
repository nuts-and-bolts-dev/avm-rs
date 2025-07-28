//! Comprehensive cryptographic test vectors
//!
//! These test vectors are derived from the go-algorand reference implementation
//! and standard cryptographic test suites to ensure compatibility and correctness.

use avm_rs::{
    opcodes::*,
    types::{StackValue, TealVersion},
};
use hex;

use crate::common::*;

/// Basic VRF functionality test
/// Test that VRF_VERIFY can be called and returns expected stack structure
#[test]
fn test_vrf_basic_functionality() {
    // Simple test that VRF_VERIFY works and returns proper stack format
    let mut bytecode = Vec::new();

    // Alpha (input message) - simple test message
    let test_message = b"test";
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_message.len() as u8);
    bytecode.extend_from_slice(test_message);

    // Pi (VRF proof) - random 80 bytes for testing (will likely fail verification)
    let test_proof = vec![0x42u8; 80];
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_proof.len() as u8);
    bytecode.extend_from_slice(&test_proof);

    // Public key - random 32 bytes for testing
    let test_pubkey = vec![0x01u8; 32];
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_pubkey.len() as u8);
    bytecode.extend_from_slice(&test_pubkey);

    bytecode.push(OP_VRF_VERIFY);

    // VRF verify should push two values: output (64 bytes) and verification result (0/1)
    // Check stack depth and clean up
    bytecode.push(OP_SWAP); // [result, output]
    bytecode.push(OP_POP); // [result] - remove output
    // The result will likely be 0 (verification failed) but that's expected for random data
    bytecode.push(OP_RETURN); // Return the verification result

    // VRF_VERIFY requires TEAL version 7 or higher and Application mode
    let vm = setup_vm_with_version(TealVersion::V7);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V7)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger);

    // The test should succeed (program executes without error) even if VRF verification fails
    assert!(result.is_ok(), "VRF_VERIFY should execute without errors");
}

#[test]
fn test_vrf_error_handling() {
    // Test VRF_VERIFY error handling with invalid inputs
    let mut bytecode = Vec::new();

    // Alpha (input message) - valid message
    let test_message = b"error_test";
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_message.len() as u8);
    bytecode.extend_from_slice(test_message);

    // Pi (VRF proof) - invalid short proof (will cause error)
    let invalid_proof = vec![0x99u8; 32]; // Too short, should be 64-96 bytes
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(invalid_proof.len() as u8);
    bytecode.extend_from_slice(&invalid_proof);

    // Public key - valid length but random data
    let test_pubkey = vec![0x88u8; 32];
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_pubkey.len() as u8);
    bytecode.extend_from_slice(&test_pubkey);

    bytecode.push(OP_VRF_VERIFY);

    // VRF_VERIFY requires TEAL version 7 or higher and Application mode
    let vm = setup_vm_with_version(TealVersion::V7);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V7)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger);

    // This should fail due to invalid proof length
    assert!(
        result.is_err(),
        "VRF_VERIFY should fail with invalid proof length"
    );
}

/// Hash function test vectors derived from standard test suites
/// These match the expected behavior in go-algorand
#[test]
fn test_sha256_standard_vectors() {
    // Test vector 1: empty string
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // empty string
    bytecode.push(OP_SHA256);

    let expected_hash =
        hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 2: "abc"
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(3);
    bytecode.extend_from_slice(b"abc");
    bytecode.push(OP_SHA256);

    let expected_hash =
        hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 3: "message digest"
    let mut bytecode = Vec::new();
    let message = b"message digest";
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(message.len() as u8);
    bytecode.extend_from_slice(message);
    bytecode.push(OP_SHA256);

    let expected_hash =
        hex::decode("f7846f55cf23e14eebeab5b4e1550cad5b509e3348fbc4efa3a1413d393cb650").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 4: "a" repeated 1000 times (performance test)
    let mut bytecode = Vec::new();
    let message = vec![b'a'; 1000]; // Reduced size to avoid stack overflow

    // Split into chunks due to bytecode size limits
    for chunk in message.chunks(255) {
        bytecode.push(OP_PUSHBYTES);
        bytecode.push(chunk.len() as u8);
        bytecode.extend_from_slice(chunk);
    }

    // Concatenate all chunks
    let concat_count = message.len().div_ceil(255) - 1;
    bytecode.extend(std::iter::repeat_n(OP_CONCAT, concat_count));

    bytecode.push(OP_SHA256);

    // Expected hash for 1000 'a's
    let expected_hash =
        hex::decode("41edece42d63e8d9bf515a9ba6932e1c20cbc9f5a5d134645adb5db1b9737ea3").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_keccak256_standard_vectors() {
    // Test vector 1: empty string
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // empty string
    bytecode.push(OP_KECCAK256);

    let expected_hash =
        hex::decode("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 2: "abc"
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(3);
    bytecode.extend_from_slice(b"abc");
    bytecode.push(OP_KECCAK256);

    let expected_hash =
        hex::decode("4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 3: "hello" (matches go-algorand test)
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(5);
    bytecode.extend_from_slice(b"hello");
    bytecode.push(OP_KECCAK256);

    let expected_hash =
        hex::decode("1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_sha3_256_standard_vectors() {
    // Test vector 1: empty string
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // empty string
    bytecode.push(OP_SHA3_256);

    let expected_hash =
        hex::decode("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 2: "abc"
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(3);
    bytecode.extend_from_slice(b"abc");
    bytecode.push(OP_SHA3_256);

    let expected_hash =
        hex::decode("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_sha512_256_standard_vectors() {
    // Test vector 1: empty string
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0); // empty string
    bytecode.push(OP_SHA512_256);

    let expected_hash =
        hex::decode("c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();

    // Test vector 2: "abc"
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(3);
    bytecode.extend_from_slice(b"abc");
    bytecode.push(OP_SHA512_256);

    let expected_hash =
        hex::decode("53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));
    execute_and_check(&bytecode, true).unwrap();
}

/// MiMC test vectors for zero-knowledge proof compatibility
/// Based on standard MiMC test suites and ZK circuit requirements
#[test]
fn test_mimc_bn254_vectors() {
    // Test vector 1: Zero key, zero message
    let mut bytecode = Vec::new();

    // Key (32 bytes of zeros)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&[0u8; 32]);

    // Message (32 bytes of zeros)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&[0u8; 32]);

    // MiMC with 220 rounds (standard security)
    bytecode.push(OP_MIMC);
    bytecode.push(220);

    // Check result is 32 bytes (field element)
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    let vm = setup_vm_with_version(TealVersion::V11);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V11)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "MiMC zero test should pass");

    // Test vector 2: Known key and message
    let mut bytecode = Vec::new();

    // Key (test vector key)
    let test_key = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&test_key);

    // Message (test vector message)
    let test_message = [
        0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
        0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe,
        0xbf, 0xc0,
    ];
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&test_message);

    // MiMC with 220 rounds
    bytecode.push(OP_MIMC);
    bytecode.push(220);

    // Check result is 32 bytes
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    let vm = setup_vm_with_version(TealVersion::V11);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V11)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "MiMC deterministic test should pass");
}

/// Ed25519 signature verification test vectors
/// Based on standard test vectors from RFC 8032
#[test]
fn test_ed25519_rfc8032_vectors() {
    // Test Vector 1 from RFC 8032
    let mut bytecode = Vec::new();

    // Message (empty)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Signature (64 bytes)
    let signature_hex = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b";
    let signature = hex::decode(signature_hex).unwrap();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(64);
    bytecode.extend_from_slice(&signature);

    // Public key (32 bytes)
    let pubkey_hex = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
    let pubkey = hex::decode(pubkey_hex).unwrap();
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&pubkey);

    bytecode.push(OP_ED25519VERIFY);

    // Should return 1 for valid signature
    bytecode = with_assert_equals(bytecode, StackValue::Uint(1));
    execute_and_check(&bytecode, true).unwrap();
}

/// Elliptic curve operation test vectors  
/// These test the basic arithmetic operations on supported curves
#[test]
fn test_ec_add_identity_elements() {
    // Test adding identity element (point at infinity) to itself
    // Identity in uncompressed format is represented as empty bytes in our implementation
    let mut bytecode = Vec::new();

    // First point: identity (empty bytes)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Second point: identity
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // EC_ADD for BN254 G1
    bytecode.push(OP_EC_ADD);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be identity (64 bytes for serialized point)
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "EC_ADD identity test should pass");
}

#[test]
fn test_ec_scalar_mul_zero() {
    // Test scalar multiplication by zero
    let mut bytecode = Vec::new();

    // Point: identity point (empty bytes represent identity)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(0);

    // Scalar: zero (32 bytes of zeros)
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(32);
    bytecode.extend_from_slice(&[0u8; 32]);

    // EC_SCALAR_MUL for BN254 G1
    bytecode.push(OP_EC_SCALAR_MUL);
    bytecode.push(0); // BN254 G1 curve ID

    // Result should be 64 bytes (point representation)
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    let vm = setup_vm_with_version(TealVersion::V10);
    let mut ledger = setup_mock_ledger();
    let config = test_config_with_version(TealVersion::V10)
        .with_run_mode(avm_rs::types::RunMode::Application);
    let result = vm.execute(&bytecode, config, &mut ledger).unwrap();
    assert!(result, "EC_SCALAR_MUL zero test should pass");
}

/// Cross-validation tests that ensure our implementations produce
/// consistent results across different operations
#[test]
fn test_hash_consistency() {
    // Test that different hash functions produce different outputs for same input
    let mut bytecode = Vec::new();

    let test_input = b"consistency test";

    // SHA256
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_input.len() as u8);
    bytecode.extend_from_slice(test_input);
    bytecode.push(OP_SHA256);

    // Keccak256
    bytecode.push(OP_PUSHBYTES);
    bytecode.push(test_input.len() as u8);
    bytecode.extend_from_slice(test_input);
    bytecode.push(OP_KECCAK256);

    // They should be different
    bytecode.push(OP_EQ);
    bytecode.push(OP_NOT); // NOT equal
    bytecode.push(OP_RETURN);

    execute_and_check(&bytecode, true).unwrap();
}

/// Performance and edge case tests
#[test]
fn test_large_input_handling() {
    // Test hash functions with maximum reasonable input size
    let mut bytecode = Vec::new();

    // Create a 1KB input by concatenating smaller chunks
    let chunk = vec![0x42u8; 255]; // Maximum single pushbytes size

    // Push 4 chunks to get ~1KB
    for _ in 0..4 {
        bytecode.push(OP_PUSHBYTES);
        bytecode.push(255);
        bytecode.extend_from_slice(&chunk);
    }

    // Concatenate all chunks
    bytecode.extend(std::iter::repeat_n(OP_CONCAT, 3));

    // Apply SHA256
    bytecode.push(OP_SHA256);

    // Result should be 32 bytes
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    execute_and_check(&bytecode, true).unwrap();
}
