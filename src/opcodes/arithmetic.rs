//! Arithmetic opcodes implementation

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;

/// Error opcode - immediately fails execution
pub fn op_err(_ctx: &mut EvalContext) -> AvmResult<()> {
    Err(AvmError::execution_halted("err opcode executed"))
}

/// Addition opcode
pub fn op_plus(ctx: &mut EvalContext) -> AvmResult<()> {
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            a_val.checked_add(b_val).ok_or(AvmError::IntegerOverflow)?
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Subtraction opcode
pub fn op_minus(ctx: &mut EvalContext) -> AvmResult<()> {
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            a_val.checked_sub(b_val).ok_or(AvmError::IntegerOverflow)?
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Division opcode
pub fn op_div(ctx: &mut EvalContext) -> AvmResult<()> {
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if b_val == 0 {
                return Err(AvmError::DivisionByZero);
            }
            a_val / b_val
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Multiplication opcode
pub fn op_mul(ctx: &mut EvalContext) -> AvmResult<()> {
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            a_val.checked_mul(b_val).ok_or(AvmError::IntegerOverflow)?
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Modulo opcode
pub fn op_mod(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if b_val == 0 {
                return Err(AvmError::DivisionByZero);
            }
            a_val % b_val
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Less than comparison
pub fn op_lt(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if a_val < b_val {
                1
            } else {
                0
            }
        }
        (StackValue::Bytes(a_bytes), StackValue::Bytes(b_bytes)) => {
            if a_bytes < b_bytes {
                1
            } else {
                0
            }
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "same type".to_string(),
                actual: "different types".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Greater than comparison
pub fn op_gt(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if a_val > b_val {
                1
            } else {
                0
            }
        }
        (StackValue::Bytes(a_bytes), StackValue::Bytes(b_bytes)) => {
            if a_bytes > b_bytes {
                1
            } else {
                0
            }
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "same type".to_string(),
                actual: "different types".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Less than or equal comparison
pub fn op_le(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if a_val <= b_val {
                1
            } else {
                0
            }
        }
        (StackValue::Bytes(a_bytes), StackValue::Bytes(b_bytes)) => {
            if a_bytes <= b_bytes {
                1
            } else {
                0
            }
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "same type".to_string(),
                actual: "different types".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Greater than or equal comparison
pub fn op_ge(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if a_val >= b_val {
                1
            } else {
                0
            }
        }
        (StackValue::Bytes(a_bytes), StackValue::Bytes(b_bytes)) => {
            if a_bytes >= b_bytes {
                1
            } else {
                0
            }
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "same type".to_string(),
                actual: "different types".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Equality comparison
pub fn op_eq(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if a_val == b_val {
                1
            } else {
                0
            }
        }
        (StackValue::Bytes(a_bytes), StackValue::Bytes(b_bytes)) => {
            if a_bytes == b_bytes {
                1
            } else {
                0
            }
        }
        _ => 0, // Different types are not equal
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Inequality comparison
pub fn op_ne(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => {
            if a_val != b_val {
                1
            } else {
                0
            }
        }
        (StackValue::Bytes(a_bytes), StackValue::Bytes(b_bytes)) => {
            if a_bytes != b_bytes {
                1
            } else {
                0
            }
        }
        _ => 1, // Different types are not equal
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Logical AND
pub fn op_and(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bool = a.as_bool()?;
    let b_bool = b.as_bool()?;

    let result = if a_bool && b_bool { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Logical OR
pub fn op_or(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bool = a.as_bool()?;
    let b_bool = b.as_bool()?;

    let result = if a_bool || b_bool { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Logical NOT
pub fn op_not(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let a = ctx.pop()?;
    let a_bool = a.as_bool()?;

    let result = if !a_bool { 1 } else { 0 };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Bitwise OR
pub fn op_bitwise_or(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => a_val | b_val,
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Bitwise AND
pub fn op_bitwise_and(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => a_val & b_val,
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Bitwise XOR
pub fn op_bitwise_xor(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let result = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => a_val ^ b_val,
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Bitwise NOT
pub fn op_bitwise_not(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let a = ctx.pop()?;

    let result = match a {
        StackValue::Uint(val) => !val,
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Multiply with overflow - returns low and high words
pub fn op_mulw(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let (a_val, b_val) = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => (a_val, b_val),
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    // Perform 128-bit multiplication
    let result = (a_val as u128) * (b_val as u128);
    let low = result as u64;
    let high = (result >> 64) as u64;

    ctx.push(StackValue::Uint(low))?;
    ctx.push(StackValue::Uint(high))?;
    Ok(())
}

/// Add with overflow - returns low and high words
pub fn op_addw(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let (a_val, b_val) = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => (a_val, b_val),
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    // Perform 128-bit addition
    let result = (a_val as u128) + (b_val as u128);
    let low = result as u64;
    let high = (result >> 64) as u64;

    ctx.push(StackValue::Uint(low))?;
    ctx.push(StackValue::Uint(high))?;
    Ok(())
}

/// Division with remainder - divmod with wide operands
pub fn op_divmodw(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let d = ctx.pop()?; // divisor
    let c = ctx.pop()?; // dividend high
    let b = ctx.pop()?; // dividend low
    let _a = ctx.pop()?; // dividend high (unused for simplicity)

    let (b_val, c_val, d_val) = match (b, c, d) {
        (StackValue::Uint(b_val), StackValue::Uint(c_val), StackValue::Uint(d_val)) => {
            (b_val, c_val, d_val)
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    if d_val == 0 {
        return Err(AvmError::DivisionByZero);
    }

    // Construct 128-bit dividend from high and low parts
    let dividend = ((c_val as u128) << 64) | (b_val as u128);
    let divisor = d_val as u128;

    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    ctx.push(StackValue::Uint((quotient >> 64) as u64))?; // quotient high
    ctx.push(StackValue::Uint(quotient as u64))?; // quotient low
    ctx.push(StackValue::Uint(remainder as u64))?; // remainder
    Ok(())
}

/// Shift left
pub fn op_shl(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?; // shift amount
    let a = ctx.pop()?; // value to shift

    let (a_val, b_val) = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => (a_val, b_val),
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    if b_val >= 64 {
        ctx.push(StackValue::Uint(0))?;
    } else {
        let result = a_val << b_val;
        ctx.push(StackValue::Uint(result))?;
    }
    Ok(())
}

/// Shift right
pub fn op_shr(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?; // shift amount
    let a = ctx.pop()?; // value to shift

    let (a_val, b_val) = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => (a_val, b_val),
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    if b_val >= 64 {
        ctx.push(StackValue::Uint(0))?;
    } else {
        let result = a_val >> b_val;
        ctx.push(StackValue::Uint(result))?;
    }
    Ok(())
}

/// Square root
pub fn op_sqrt(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let a = ctx.pop()?;

    let a_val = match a {
        StackValue::Uint(val) => val,
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    let result = (a_val as f64).sqrt() as u64;
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Bit length - highest set bit position
pub fn op_bitlen(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let a = ctx.pop()?;

    let a_val = match a {
        StackValue::Uint(val) => val,
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    let result = if a_val == 0 {
        0
    } else {
        64 - a_val.leading_zeros() as u64
    };
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Exponentiation
pub fn op_exp(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?; // exponent
    let a = ctx.pop()?; // base

    let (a_val, b_val) = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => (a_val, b_val),
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    // Check for overflow conditions
    if b_val > 64 {
        return Err(AvmError::IntegerOverflow);
    }

    let result = a_val
        .checked_pow(b_val as u32)
        .ok_or(AvmError::IntegerOverflow)?;
    ctx.push(StackValue::Uint(result))?;
    Ok(())
}

/// Exponentiation with overflow - returns high and low words
pub fn op_expw(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let b = ctx.pop()?; // exponent
    let a = ctx.pop()?; // base

    let (a_val, b_val) = match (a, b) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val)) => (a_val, b_val),
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    if b_val > 64 {
        return Err(AvmError::IntegerOverflow);
    }

    // Use 128-bit arithmetic for exponentiation
    let result = (a_val as u128)
        .checked_pow(b_val as u32)
        .ok_or(AvmError::IntegerOverflow)?;
    let low = result as u64;
    let high = (result >> 64) as u64;

    ctx.push(StackValue::Uint(low))?;
    ctx.push(StackValue::Uint(high))?;
    Ok(())
}

/// Byte square root - square root of bytes interpreted as big-endian number
pub fn op_bsqrt(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let a = ctx.pop()?;

    let a_bytes = match a {
        StackValue::Bytes(bytes) => bytes,
        _ => {
            return Err(AvmError::TypeError {
                expected: "bytes".to_string(),
                actual: "uint".to_string(),
            });
        }
    };

    if a_bytes.len() > 8 {
        return Err(AvmError::invalid_program("Bytes too large for square root"));
    }

    // Convert bytes to u64 (big-endian)
    let mut val = 0u64;
    for &byte in &a_bytes {
        val = (val << 8) | (byte as u64);
    }

    let sqrt_val = (val as f64).sqrt() as u64;

    // Convert back to bytes (big-endian)
    let result_bytes = sqrt_val.to_be_bytes().to_vec();
    ctx.push(StackValue::Bytes(result_bytes))?;
    Ok(())
}

/// Division with overflow - wide division
pub fn op_divw(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let c = ctx.pop()?; // divisor
    let b = ctx.pop()?; // dividend high
    let a = ctx.pop()?; // dividend low

    let (a_val, b_val, c_val) = match (a, b, c) {
        (StackValue::Uint(a_val), StackValue::Uint(b_val), StackValue::Uint(c_val)) => {
            (a_val, b_val, c_val)
        }
        _ => {
            return Err(AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            });
        }
    };

    if c_val == 0 {
        return Err(AvmError::DivisionByZero);
    }

    // Construct 128-bit dividend
    let dividend = ((b_val as u128) << 64) | (a_val as u128);
    let divisor = c_val as u128;

    let quotient = dividend / divisor;

    if quotient > u64::MAX as u128 {
        return Err(AvmError::IntegerOverflow);
    }

    ctx.push(StackValue::Uint(quotient as u64))?;
    Ok(())
}
