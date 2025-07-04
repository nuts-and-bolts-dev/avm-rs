//! Cryptographic utilities and implementations

pub use ed25519_dalek::{Signature, Verifier, VerifyingKey};
pub use sha2::{Digest, Sha256, Sha512};
pub use sha3::{Keccak256, Sha3_256};

/// Re-export common cryptographic functions
pub mod hash {
    use super::*;

    /// Compute SHA256 hash
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Compute Keccak256 hash
    pub fn keccak256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Compute SHA512/256 hash
    pub fn sha512_256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(data);
        let result = hasher.finalize();
        result[..32].to_vec()
    }

    /// Compute SHA3-256 hash
    pub fn sha3_256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

/// Ed25519 signature verification utilities
pub mod ed25519 {
    use super::*;
    use crate::error::{AvmError, AvmResult};

    /// Verify Ed25519 signature
    pub fn verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> AvmResult<bool> {
        if public_key.len() != 32 {
            return Err(AvmError::crypto_error("Invalid public key length"));
        }

        if signature.len() != 64 {
            return Err(AvmError::crypto_error("Invalid signature length"));
        }

        let verifying_key = VerifyingKey::from_bytes(
            public_key
                .try_into()
                .map_err(|_| AvmError::crypto_error("Invalid public key format"))?,
        )
        .map_err(|e| AvmError::crypto_error(format!("Invalid public key: {e}")))?;

        let signature = Signature::from_bytes(
            signature
                .try_into()
                .map_err(|_| AvmError::crypto_error("Invalid signature format"))?,
        );

        match verifying_key.verify(message, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
