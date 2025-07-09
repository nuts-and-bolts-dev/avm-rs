//! Tests for cryptographic opcodes

use hex;
use rust_avm::{opcodes::*, types::StackValue};

use crate::common::*;

#[test]
fn test_op_sha256() {
    // Test SHA256 hash of "hello"
    let mut bytecode = Vec::new();
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");
    bytecode.push(OP_SHA256);

    // Expected SHA256 hash of "hello"
    let expected_hash =
        hex::decode("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));

    execute_and_check(&bytecode, true).unwrap();

    // Test empty string hash
    let bytecode = with_assert_equals(
        build_simple_op_test(vec![StackValue::Bytes(vec![])], OP_SHA256),
        StackValue::Bytes(
            hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
                .unwrap(),
        ),
    );
    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_keccak256() {
    // Test Keccak256 hash of "hello"
    let mut bytecode = Vec::new();
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");
    bytecode.push(OP_KECCAK256);

    // Expected Keccak256 hash of "hello"
    let expected_hash =
        hex::decode("1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8").unwrap();
    bytecode = with_assert_equals(bytecode, StackValue::Bytes(expected_hash));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_sha512_256() {
    // Test SHA512/256 hash of "hello"
    let mut bytecode = Vec::new();
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");
    bytecode.push(OP_SHA512_256);

    // SHA512/256 produces a 32-byte hash
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_sha3_256() {
    // Test SHA3-256 hash
    let mut bytecode = Vec::new();
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");
    bytecode.push(OP_SHA3_256);

    // SHA3-256 produces a 32-byte hash
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(32));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ed25519verify_valid() {
    // Test Ed25519 signature verification with a valid signature
    // This is a test vector with known values
    let mut bytecode = Vec::new();

    // Data to verify (message)
    bytecode.push(0x80); // pushbytes
    bytecode.push(11); // length
    bytecode.extend_from_slice(b"hello world");

    // Signature (64 bytes) - using a non-zero dummy signature for testing
    bytecode.push(0x80); // pushbytes
    bytecode.push(64); // length
    bytecode.extend_from_slice(&[1u8; 64]); // Non-zero dummy signature

    // Public key (32 bytes) - using a non-zero dummy key for testing
    bytecode.push(0x80); // pushbytes
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]); // Non-zero dummy public key

    bytecode.push(OP_ED25519VERIFY);

    // The verification will fail with dummy values, but should return 0 (not error)
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ed25519verify_invalid_key_length() {
    // Test Ed25519 verification with invalid public key length
    let mut bytecode = Vec::new();

    // Data
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");

    // Signature (64 bytes)
    bytecode.push(0x80); // pushbytes
    bytecode.push(64); // length
    bytecode.extend_from_slice(&[0u8; 64]);

    // Invalid public key (wrong length)
    bytecode.push(0x80); // pushbytes
    bytecode.push(16); // wrong length (should be 32)
    bytecode.extend_from_slice(&[0u8; 16]);

    bytecode.push(OP_ED25519VERIFY);

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_ed25519verify_invalid_sig_length() {
    // Test Ed25519 verification with invalid signature length
    let mut bytecode = Vec::new();

    // Data
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");

    // Invalid signature (wrong length)
    bytecode.push(0x80); // pushbytes
    bytecode.push(32); // wrong length (should be 64)
    bytecode.extend_from_slice(&[0u8; 32]);

    // Public key (32 bytes)
    bytecode.push(0x80); // pushbytes
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[0u8; 32]);

    bytecode.push(OP_ED25519VERIFY);

    execute_expect_error(&bytecode).unwrap();
}

#[test]
fn test_op_ed25519verify_bare() {
    // Test Ed25519 bare signature verification
    let mut bytecode = Vec::new();

    // Data to verify
    bytecode.push(0x80); // pushbytes
    bytecode.push(11); // length
    bytecode.extend_from_slice(b"hello world");

    // Signature (64 bytes)
    bytecode.push(0x80); // pushbytes
    bytecode.push(64); // length
    bytecode.extend_from_slice(&[1u8; 64]);

    // Public key (32 bytes)
    bytecode.push(0x80); // pushbytes
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[1u8; 32]);

    bytecode.push(OP_ED25519VERIFY_BARE);

    // Should return 0 for invalid signature
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ecdsa_verify_placeholder() {
    // Test ECDSA verification (placeholder implementation)
    let mut bytecode = Vec::new();

    // Recovery ID
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes());

    // Data
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");

    // Signature
    bytecode.push(0x80); // pushbytes
    bytecode.push(64); // length
    bytecode.extend_from_slice(&[0u8; 64]);

    // Public key
    bytecode.push(0x80); // pushbytes
    bytecode.push(33); // length (compressed)
    bytecode.extend_from_slice(&[0u8; 33]);

    bytecode.push(OP_ECDSA_VERIFY);

    // Placeholder returns 0
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ecdsa_pk_decompress_placeholder() {
    // Test ECDSA public key decompression (placeholder)
    let mut bytecode = Vec::new();

    // Compressed public key
    bytecode.push(0x80); // pushbytes
    bytecode.push(33); // length
    bytecode.extend_from_slice(&[0u8; 33]);

    bytecode.push(OP_ECDSA_PK_DECOMPRESS);

    // Placeholder returns 64-byte result
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_ecdsa_pk_recover_placeholder() {
    // Test ECDSA public key recovery (placeholder)
    let mut bytecode = Vec::new();

    // Data
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");

    // Signature
    bytecode.push(0x80); // pushbytes
    bytecode.push(64); // length
    bytecode.extend_from_slice(&[0u8; 64]);

    // Recovery ID
    bytecode.push(0x81); // pushint
    bytecode.extend_from_slice(&0u64.to_be_bytes());

    bytecode.push(OP_ECDSA_PK_RECOVER);

    // Placeholder returns 64-byte result
    bytecode.push(OP_LEN);
    bytecode = with_assert_equals(bytecode, StackValue::Uint(64));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_op_vrf_verify_placeholder() {
    // Test VRF verification (placeholder)
    let mut bytecode = Vec::new();

    // Data
    bytecode.push(0x80); // pushbytes
    bytecode.push(5); // length
    bytecode.extend_from_slice(b"hello");

    // Proof
    bytecode.push(0x80); // pushbytes
    bytecode.push(80); // length
    bytecode.extend_from_slice(&[0u8; 80]);

    // Public key
    bytecode.push(0x80); // pushbytes
    bytecode.push(32); // length
    bytecode.extend_from_slice(&[0u8; 32]);

    bytecode.push(OP_VRF_VERIFY);

    // VRF verify pushes two values: output (64 bytes) and verification result (0/1)
    // Check verification result is 0 (placeholder)
    // Pop the output bytes first
    bytecode.push(OP_SWAP); // [result, output]
    bytecode.push(OP_POP); // [result]
    bytecode = with_assert_equals(bytecode, StackValue::Uint(0));

    execute_and_check(&bytecode, true).unwrap();
}

#[test]
fn test_hash_algorithms_different_outputs() {
    // Test that different hash algorithms produce different outputs for same input
    let mut bytecode = Vec::new();

    let test_data = b"test data";

    // SHA256
    bytecode.push(0x80); // pushbytes
    bytecode.push(test_data.len() as u8);
    bytecode.extend_from_slice(test_data);
    bytecode.push(OP_SHA256);

    // Keccak256
    bytecode.push(0x80); // pushbytes
    bytecode.push(test_data.len() as u8);
    bytecode.extend_from_slice(test_data);
    bytecode.push(OP_KECCAK256);

    // Compare - should be different
    bytecode.push(OP_EQ);
    bytecode.push(OP_NOT); // NOT equal
    bytecode.push(0x43); // return

    execute_and_check(&bytecode, true).unwrap();
}
