//! Cryptographic opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256, Sha512};
use sha3::Keccak256;

/// SHA256 hash
pub fn op_sha256(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let bytes = val.as_bytes()?;

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();

    ctx.push(StackValue::Bytes(result.to_vec()))?;
    Ok(())
}

/// Keccak256 hash
pub fn op_keccak256(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let bytes = val.as_bytes()?;

    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    let result = hasher.finalize();

    ctx.push(StackValue::Bytes(result.to_vec()))?;
    Ok(())
}

/// SHA512/256 hash
pub fn op_sha512_256(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let bytes = val.as_bytes()?;

    let mut hasher = Sha512::new();
    hasher.update(bytes);
    let result = hasher.finalize();

    // Take first 32 bytes for SHA512/256
    ctx.push(StackValue::Bytes(result[..32].to_vec()))?;
    Ok(())
}

/// SHA3-256 hash
pub fn op_sha3_256(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let bytes = val.as_bytes()?;

    let mut hasher = sha3::Sha3_256::new();
    hasher.update(bytes);
    let result = hasher.finalize();

    ctx.push(StackValue::Bytes(result.to_vec()))?;
    Ok(())
}

/// Ed25519 signature verification
pub fn op_ed25519verify(ctx: &mut EvalContext) -> AvmResult<()> {
    let public_key = ctx.pop()?;
    let signature = ctx.pop()?;
    let data = ctx.pop()?;

    let pub_key_bytes = public_key.as_bytes()?;
    let sig_bytes = signature.as_bytes()?;
    let data_bytes = data.as_bytes()?;

    // Validate input lengths
    if pub_key_bytes.len() != 32 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 32,
            actual: pub_key_bytes.len(),
        });
    }

    if sig_bytes.len() != 64 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 64,
            actual: sig_bytes.len(),
        });
    }

    // Create verifying key and signature
    let verifying_key = VerifyingKey::from_bytes(
        pub_key_bytes
            .try_into()
            .map_err(|_| AvmError::crypto_error("Invalid public key format"))?,
    )
    .map_err(|e| AvmError::crypto_error(format!("Invalid public key: {e}")))?;

    let signature = Signature::from_bytes(
        sig_bytes
            .try_into()
            .map_err(|_| AvmError::crypto_error("Invalid signature format"))?,
    );

    // Verify signature
    let result = match verifying_key.verify(data_bytes, &signature) {
        Ok(()) => 1,
        Err(_) => 0,
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// ECDSA signature verification (secp256k1)
pub fn op_ecdsa_verify(ctx: &mut EvalContext) -> AvmResult<()> {
    let public_key = ctx.pop()?;
    let signature = ctx.pop()?;
    let data = ctx.pop()?;
    let recovery_id = ctx.pop()?;

    let _pub_key_bytes = public_key.as_bytes()?;
    let _sig_bytes = signature.as_bytes()?;
    let _data_bytes = data.as_bytes()?;
    let _recovery_id = recovery_id.as_uint()?;

    // For now, return a placeholder implementation
    // Full ECDSA verification would require secp256k1 library
    ctx.push(StackValue::Uint(0))?;
    Ok(())
}

/// ECDSA public key decompression
pub fn op_ecdsa_pk_decompress(ctx: &mut EvalContext) -> AvmResult<()> {
    let compressed_key = ctx.pop()?;
    let _key_bytes = compressed_key.as_bytes()?;

    // Placeholder implementation
    // Full implementation would decompress the public key
    ctx.push(StackValue::Bytes(vec![0u8; 64]))?;
    Ok(())
}

/// ECDSA public key recovery
pub fn op_ecdsa_pk_recover(ctx: &mut EvalContext) -> AvmResult<()> {
    let recovery_id = ctx.pop()?;
    let signature = ctx.pop()?;
    let data = ctx.pop()?;

    let _recovery_id = recovery_id.as_uint()?;
    let _sig_bytes = signature.as_bytes()?;
    let _data_bytes = data.as_bytes()?;

    // Placeholder implementation
    // Full implementation would recover the public key
    ctx.push(StackValue::Bytes(vec![0u8; 64]))?;
    Ok(())
}

/// VRF verification
pub fn op_vrf_verify(ctx: &mut EvalContext) -> AvmResult<()> {
    let public_key = ctx.pop()?;
    let proof = ctx.pop()?;
    let data = ctx.pop()?;

    let _pub_key_bytes = public_key.as_bytes()?;
    let _proof_bytes = proof.as_bytes()?;
    let _data_bytes = data.as_bytes()?;

    // Placeholder implementation
    // Full VRF verification would be implemented here
    ctx.push(StackValue::Bytes(vec![0u8; 64]))?; // VRF output
    ctx.push(StackValue::Uint(0))?; // Verification result
    Ok(())
}
