//! Byte operations and manipulation opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;
use base64::Engine;
use num_traits::Zero;

const MAX_BYTE_MATH_SIZE: usize = 64;

/// Convert bytes to big-endian number (up to 64 bytes)
fn bytes_to_uint(bytes: &[u8]) -> AvmResult<num_bigint::BigUint> {
    if bytes.len() > MAX_BYTE_MATH_SIZE {
        return Err(AvmError::invalid_program(format!(
            "Byte array too large for math: {} > {}",
            bytes.len(),
            MAX_BYTE_MATH_SIZE
        )));
    }
    Ok(num_bigint::BigUint::from_bytes_be(bytes))
}

/// Convert big-endian number back to bytes
fn uint_to_bytes(val: &num_bigint::BigUint, min_len: usize) -> Vec<u8> {
    let bytes = val.to_bytes_be();
    if bytes.len() >= min_len {
        bytes
    } else {
        let mut result = vec![0u8; min_len - bytes.len()];
        result.extend_from_slice(&bytes);
        result
    }
}

/// Byte addition
pub fn op_b_plus(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    let result = a_num + b_num;
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let result_bytes = uint_to_bytes(&result, max_len);

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Byte subtraction
pub fn op_b_minus(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    if a_num < b_num {
        return Err(AvmError::IntegerUnderflow);
    }

    let result = a_num - b_num;
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let result_bytes = uint_to_bytes(&result, max_len);

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Byte division
pub fn op_b_div(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    if b_num.is_zero() {
        return Err(AvmError::DivisionByZero);
    }

    let result = a_num / b_num;
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let result_bytes = uint_to_bytes(&result, max_len);

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Byte multiplication
pub fn op_b_mul(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    let result = a_num * b_num;
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let result_bytes = uint_to_bytes(&result, max_len);

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Byte less than
pub fn op_b_lt(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    let result = if a_num < b_num { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Byte greater than
pub fn op_b_gt(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    let result = if a_num > b_num { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Byte less than or equal
pub fn op_b_le(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    let result = if a_num <= b_num { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Byte greater than or equal
pub fn op_b_ge(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    let result = if a_num >= b_num { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Byte equal
pub fn op_b_eq(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let result = if a_bytes == b_bytes { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Byte not equal
pub fn op_b_ne(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let result = if a_bytes != b_bytes { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Byte modulo
pub fn op_b_mod(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let a_num = bytes_to_uint(a_bytes)?;
    let b_num = bytes_to_uint(b_bytes)?;

    if b_num.is_zero() {
        return Err(AvmError::DivisionByZero);
    }

    let result = a_num % b_num;
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let result_bytes = uint_to_bytes(&result, max_len);

    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Byte bitwise OR
pub fn op_b_or(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    // Pad to same length
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let mut a_padded = vec![0u8; max_len];
    let mut b_padded = vec![0u8; max_len];

    a_padded[max_len - a_bytes.len()..].copy_from_slice(a_bytes);
    b_padded[max_len - b_bytes.len()..].copy_from_slice(b_bytes);

    let result: Vec<u8> = a_padded
        .iter()
        .zip(b_padded.iter())
        .map(|(a, b)| a | b)
        .collect();

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Byte bitwise AND
pub fn op_b_and(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    // Pad to same length
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let mut a_padded = vec![0u8; max_len];
    let mut b_padded = vec![0u8; max_len];

    a_padded[max_len - a_bytes.len()..].copy_from_slice(a_bytes);
    b_padded[max_len - b_bytes.len()..].copy_from_slice(b_bytes);

    let result: Vec<u8> = a_padded
        .iter()
        .zip(b_padded.iter())
        .map(|(a, b)| a & b)
        .collect();

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Byte bitwise XOR
pub fn op_b_xor(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    // Pad to same length
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());
    let mut a_padded = vec![0u8; max_len];
    let mut b_padded = vec![0u8; max_len];

    a_padded[max_len - a_bytes.len()..].copy_from_slice(a_bytes);
    b_padded[max_len - b_bytes.len()..].copy_from_slice(b_bytes);

    let result: Vec<u8> = a_padded
        .iter()
        .zip(b_padded.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Byte bitwise NOT
pub fn op_b_not(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let result: Vec<u8> = a_bytes.iter().map(|b| !b).collect();

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Byte manipulation operations
/// Get bit from bytes
pub fn op_getbit(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let bit_index = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let bit_idx = bit_index.as_uint()? as usize;

    let total_bits = bytes.len() * 8;
    if bit_idx >= total_bits {
        return Err(AvmError::invalid_program(format!(
            "Bit index {bit_idx} out of bounds for {total_bits} bits"
        )));
    }

    let byte_idx = bit_idx / 8;
    let bit_offset = 7 - (bit_idx % 8); // MSB is bit 0
    let bit_value = (bytes[byte_idx] >> bit_offset) & 1;

    ctx.push(StackValue::Uint(bit_value as u64))?;
    Ok(())
}

/// Set bit in bytes
pub fn op_setbit(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let new_bit = ctx.pop()?;
    let bit_index = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let bit_idx = bit_index.as_uint()? as usize;
    let bit_val = new_bit.as_uint()? & 1;

    let total_bits = bytes.len() * 8;
    if bit_idx >= total_bits {
        return Err(AvmError::invalid_program(format!(
            "Bit index {bit_idx} out of bounds for {total_bits} bits"
        )));
    }

    let mut result = bytes.to_vec();
    let byte_idx = bit_idx / 8;
    let bit_offset = 7 - (bit_idx % 8); // MSB is bit 0

    if bit_val == 1 {
        result[byte_idx] |= 1 << bit_offset;
    } else {
        result[byte_idx] &= !(1 << bit_offset);
    }

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Get byte from bytes
pub fn op_getbyte(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let byte_index = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let byte_idx = byte_index.as_uint()? as usize;

    if byte_idx >= bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Byte index {} out of bounds for {} bytes",
            byte_idx,
            bytes.len()
        )));
    }

    ctx.push(StackValue::Uint(bytes[byte_idx] as u64))?;
    Ok(())
}

/// Set byte in bytes
pub fn op_setbyte(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let new_byte = ctx.pop()?;
    let byte_index = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let byte_idx = byte_index.as_uint()? as usize;
    let byte_val = new_byte.as_uint()? as u8;

    if byte_idx >= bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Byte index {} out of bounds for {} bytes",
            byte_idx,
            bytes.len()
        )));
    }

    let mut result = bytes.to_vec();
    result[byte_idx] = byte_val;

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Extract bytes with immediate start and length
pub fn op_extract(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let start = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;
    let length = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    let value = ctx.pop()?;
    let bytes = value.as_bytes()?;

    if start >= bytes.len() || start + length > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Extract bounds [{}, {}) out of range for {} bytes",
            start,
            start + length,
            bytes.len()
        )));
    }

    let result = bytes[start..start + length].to_vec();
    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Extract bytes with stack arguments
pub fn op_extract3(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let length = ctx.pop()?;
    let start = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let start_idx = start.as_uint()? as usize;
    let len = length.as_uint()? as usize;

    if start_idx >= bytes.len() || start_idx + len > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Extract bounds [{}, {}) out of range for {} bytes",
            start_idx,
            start_idx + len,
            bytes.len()
        )));
    }

    let result = bytes[start_idx..start_idx + len].to_vec();
    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Extract uint16 from bytes
pub fn op_extract_uint16(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let start = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let start_idx = start.as_uint()? as usize;

    if start_idx + 2 > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Extract uint16 bounds [{}, {}) out of range for {} bytes",
            start_idx,
            start_idx + 2,
            bytes.len()
        )));
    }

    let result = u16::from_be_bytes([bytes[start_idx], bytes[start_idx + 1]]) as u64;
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Extract uint32 from bytes
pub fn op_extract_uint32(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let start = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let start_idx = start.as_uint()? as usize;

    if start_idx + 4 > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Extract uint32 bounds [{}, {}) out of range for {} bytes",
            start_idx,
            start_idx + 4,
            bytes.len()
        )));
    }

    let result = u32::from_be_bytes([
        bytes[start_idx],
        bytes[start_idx + 1],
        bytes[start_idx + 2],
        bytes[start_idx + 3],
    ]) as u64;
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Extract uint64 from bytes
pub fn op_extract_uint64(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let start = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let start_idx = start.as_uint()? as usize;

    if start_idx + 8 > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Extract uint64 bounds [{}, {}) out of range for {} bytes",
            start_idx,
            start_idx + 8,
            bytes.len()
        )));
    }

    let result = u64::from_be_bytes([
        bytes[start_idx],
        bytes[start_idx + 1],
        bytes[start_idx + 2],
        bytes[start_idx + 3],
        bytes[start_idx + 4],
        bytes[start_idx + 5],
        bytes[start_idx + 6],
        bytes[start_idx + 7],
    ]);
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Replace bytes with immediate start and replacement bytes
pub fn op_replace2(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let start = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    let replacement = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let repl_bytes = replacement.as_bytes()?;

    if start >= bytes.len() || start + repl_bytes.len() > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Replace bounds [{}, {}) out of range for {} bytes",
            start,
            start + repl_bytes.len(),
            bytes.len()
        )));
    }

    let mut result = bytes.to_vec();
    result[start..start + repl_bytes.len()].copy_from_slice(repl_bytes);

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Replace bytes with stack arguments
pub fn op_replace3(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let replacement = ctx.pop()?;
    let start = ctx.pop()?;
    let value = ctx.pop()?;

    let bytes = value.as_bytes()?;
    let repl_bytes = replacement.as_bytes()?;
    let start_idx = start.as_uint()? as usize;

    if start_idx >= bytes.len() || start_idx + repl_bytes.len() > bytes.len() {
        return Err(AvmError::invalid_program(format!(
            "Replace bounds [{}, {}) out of range for {} bytes",
            start_idx,
            start_idx + repl_bytes.len(),
            bytes.len()
        )));
    }

    let mut result = bytes.to_vec();
    result[start_idx..start_idx + repl_bytes.len()].copy_from_slice(repl_bytes);

    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Base64 decode
pub fn op_base64_decode(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let encoding = ctx.read_bytes(1)?[0]; // 0=URLEncoding, 1=StdEncoding
    ctx.advance_pc(1)?;

    let value = ctx.pop()?;
    let input = value.as_bytes()?;

    let input_str = std::str::from_utf8(input)
        .map_err(|_| AvmError::invalid_program("Invalid UTF-8 in base64 input"))?;

    let result = match encoding {
        0 => base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input_str),
        1 => base64::engine::general_purpose::STANDARD.decode(input_str),
        _ => return Err(AvmError::invalid_program("Invalid base64 encoding type")),
    };

    match result {
        Ok(decoded) => ctx.push(StackValue::Bytes(decoded))?,
        Err(_) => return Err(AvmError::invalid_program("Invalid base64 input")),
    }

    Ok(())
}

/// JSON reference - simplified implementation
pub fn op_json_ref(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let return_type = ctx.read_bytes(1)?[0]; // 0=JSONString, 1=JSONUint64, 2=JSONObject
    ctx.advance_pc(1)?;

    let key = ctx.pop()?;
    let json_data = ctx.pop()?;

    let json_bytes = json_data.as_bytes()?;
    let key_bytes = key.as_bytes()?;

    let _json_str = std::str::from_utf8(json_bytes)
        .map_err(|_| AvmError::invalid_program("Invalid UTF-8 in JSON data"))?;
    let key_str = std::str::from_utf8(key_bytes)
        .map_err(|_| AvmError::invalid_program("Invalid UTF-8 in JSON key"))?;

    // TODO: Implement proper JSON parsing and field extraction using serde_json
    match return_type {
        0 => {
            // JSONString
            let result = format!("\"{key_str}\""); // Placeholder
            ctx.push(StackValue::Bytes(result.into_bytes()))?;
        }
        1 => {
            // JSONUint64
            ctx.push(StackValue::Uint(0))?; // Placeholder
        }
        2 => {
            // JSONObject
            ctx.push(StackValue::Bytes(b"{}".to_vec()))?; // Placeholder
        }
        _ => return Err(AvmError::invalid_program("Invalid JSON return type")),
    }

    Ok(())
}
