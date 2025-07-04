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
