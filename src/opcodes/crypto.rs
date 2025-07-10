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
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
    Ok(())
}

/// Ed25519 signature verification without prefix
pub fn op_ed25519verify_bare(ctx: &mut EvalContext) -> AvmResult<()> {
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

    // Verify signature directly on the data without prefix (bare verification)
    let result = match verifying_key.verify(data_bytes, &signature) {
        Ok(()) => 1,
        Err(_) => 0,
    };

    ctx.push(StackValue::Uint(result))?;
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
    Ok(())
}

/// ECDSA public key decompression
pub fn op_ecdsa_pk_decompress(ctx: &mut EvalContext) -> AvmResult<()> {
    let compressed_key = ctx.pop()?;
    let _key_bytes = compressed_key.as_bytes()?;

    // Placeholder implementation
    // Full implementation would decompress the public key
    ctx.push(StackValue::Bytes(vec![0u8; 64]))?;
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
    Ok(())
}

/// Advanced cryptographic hash function (MiMC)
pub fn op_mimc(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let round_count = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    let message = ctx.pop()?;
    let key = ctx.pop()?;

    let _message_bytes = message.as_bytes()?;
    let _key_bytes = key.as_bytes()?;

    // Validate round count
    if round_count == 0 || round_count > 255 {
        return Err(AvmError::invalid_program("Invalid MiMC round count"));
    }

    // In a real implementation, this would:
    // 1. Implement the MiMC hash function
    // 2. Use the specified number of rounds
    // 3. Apply the key and message according to MiMC specification

    // For now, return placeholder hash
    ctx.push(StackValue::Bytes(vec![0u8; 32]))?;
    Ok(())
}

/// Get random bytes from blockchain randomness beacon
pub fn op_block(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let block_num = ctx.pop()?;
    let _block_number = block_num.as_uint()?;

    // Block fields:
    // 0 - BlkSeed: randomness seed for the block
    // 1 - BlkTimestamp: timestamp of the block
    match field_id {
        0 => {
            // BlkSeed - return 32 bytes of randomness
            ctx.push(StackValue::Bytes(vec![0u8; 32]))?;
        }
        1 => {
            // BlkTimestamp - return timestamp as uint64
            ctx.push(StackValue::Uint(0))?;
        }
        _ => {
            return Err(AvmError::invalid_program(format!(
                "Invalid block field: {field_id}"
            )));
        }
    }

    Ok(())
}
