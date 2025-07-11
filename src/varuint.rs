//! Variable-length unsigned integer encoding (proto-buf style)

use crate::error::{AvmError, AvmResult};

/// Encode a u64 as varuint bytes
pub fn encode_varuint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();

    while value >= 0x80 {
        bytes.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    bytes.push(value as u8);

    bytes
}

/// Decode varuint from bytes, returning (value, bytes_consumed)
pub fn decode_varuint(bytes: &[u8]) -> AvmResult<(u64, usize)> {
    let mut value = 0u64;
    let mut shift = 0;
    let mut index = 0;

    for &byte in bytes {
        if index >= 10 {
            // Prevent overflow - varuint should not exceed 10 bytes for u64
            return Err(AvmError::InvalidProgram("Varuint too long".to_string()));
        }

        value |= ((byte & 0x7F) as u64) << shift;
        index += 1;

        if (byte & 0x80) == 0 {
            // MSB is 0, this is the last byte
            return Ok((value, index));
        }

        shift += 7;
        if shift >= 64 {
            return Err(AvmError::InvalidProgram("Varuint overflow".to_string()));
        }
    }

    Err(AvmError::InvalidProgram("Incomplete varuint".to_string()))
}

/// Read varuint from context and advance PC
pub fn read_varuint_from_context(ctx: &mut crate::vm::EvalContext) -> AvmResult<u64> {
    let remaining_bytes = ctx.program_len() - ctx.pc();
    if remaining_bytes == 0 {
        return Err(AvmError::InvalidProgram(
            "Unexpected end of program reading varuint".to_string(),
        ));
    }

    let bytes = &ctx.get_program()[ctx.pc()..];
    let (value, consumed) = decode_varuint(bytes)?;
    ctx.advance_pc(consumed)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varuint_encoding_small_values() {
        assert_eq!(encode_varuint(0), vec![0]);
        assert_eq!(encode_varuint(1), vec![1]);
        assert_eq!(encode_varuint(127), vec![127]);
    }

    #[test]
    fn test_varuint_encoding_large_values() {
        assert_eq!(encode_varuint(128), vec![0x80, 0x01]);
        assert_eq!(encode_varuint(255), vec![0xFF, 0x01]);
        assert_eq!(encode_varuint(300), vec![0xAC, 0x02]);
        assert_eq!(encode_varuint(16384), vec![0x80, 0x80, 0x01]);
    }

    #[test]
    fn test_varuint_round_trip() {
        let test_values = vec![
            0,
            1,
            127,
            128,
            255,
            256,
            16383,
            16384,
            2097151,
            2097152,
            u64::MAX,
        ];

        for value in test_values {
            let encoded = encode_varuint(value);
            let (decoded, consumed) = decode_varuint(&encoded).unwrap();
            assert_eq!(value, decoded);
            assert_eq!(consumed, encoded.len());
        }
    }

    #[test]
    fn test_varuint_decoding_errors() {
        // Empty bytes
        assert!(decode_varuint(&[]).is_err());

        // Incomplete varuint (all bytes have continuation bit set)
        assert!(decode_varuint(&[0x80, 0x80, 0x80]).is_err());

        // Too long varuint (would overflow u64)
        let too_long = vec![0xFF; 11];
        assert!(decode_varuint(&too_long).is_err());
    }
}
