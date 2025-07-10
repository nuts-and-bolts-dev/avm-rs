//! Stack manipulation opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;

/// Pop the top stack value
pub fn op_pop(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.pop()?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Duplicate the top stack value
pub fn op_dup(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.peek()?.clone();
    ctx.push(val)?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Duplicate the top two stack values
/// TODO: Implementation incorrect - currently duplicates top value twice instead of duplicating top TWO values
/// Should copy values at positions 0,1 and push copies: [a,b] -> [a,b,a,b]
/// Current implementation: [a,b] -> [a,b,b,b] (wrong!)
pub fn op_dup2(ctx: &mut EvalContext) -> AvmResult<()> {
    if ctx.stack_size() < 2 {
        return Err(AvmError::StackUnderflow);
    }

    // Duplicate the top two values according to TEAL spec: [A, B] -> [A, B, A, B]
    // This matches go-algorand implementation: cx.Stack = append(cx.Stack, cx.Stack[prev:]...)
    let b = ctx.pop()?;  // Pop top value
    let a = ctx.pop()?;  // Pop second value
    
    // Push back original values
    ctx.push(a.clone())?;
    ctx.push(b.clone())?;
    
    // Push duplicates
    ctx.push(a)?;
    ctx.push(b)?;

    ctx.advance_pc(1)?;
    Ok(())
}

/// Swap the top two stack values
pub fn op_swap(ctx: &mut EvalContext) -> AvmResult<()> {
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    ctx.push(b)?;
    ctx.push(a)?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Select between two values based on condition
pub fn op_select(ctx: &mut EvalContext) -> AvmResult<()> {
    let c = ctx.pop()?;
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let condition = c.as_bool()?;
    let result = if condition { a } else { b };

    ctx.push(result)?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Dig value from stack depth N
pub fn op_dig(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    // Get the depth from the immediate value
    let depth = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    if ctx.stack_size() <= depth {
        return Err(AvmError::StackUnderflow);
    }

    // Get the value at depth N from the top and push a copy to the top
    let value = ctx.peek_at_depth(depth)?.clone();
    ctx.push(value)?;
    Ok(())
}

/// Bury value at stack depth N
pub fn op_bury(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    // Get the depth from the immediate value
    let depth = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    if ctx.stack_size() <= depth {
        return Err(AvmError::StackUnderflow);
    }

    // Pop the top value and replace the value at depth N
    let value = ctx.pop()?;
    let _ = ctx.remove_at_depth(depth)?; // Remove the old value
    ctx.insert_at_depth(depth, value)?; // Insert the new value
    Ok(())
}

/// Cover N values with top value
pub fn op_cover(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let n = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    if ctx.stack_size() <= n {
        return Err(AvmError::StackUnderflow);
    }

    // Pop the top value and insert it at depth N+1
    let value = ctx.pop()?;
    ctx.insert_at_depth(n, value)?;
    Ok(())
}

/// Uncover N values to top
pub fn op_uncover(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let n = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    if ctx.stack_size() <= n {
        return Err(AvmError::StackUnderflow);
    }

    // Remove the value at depth N and push it to the top
    let value = ctx.remove_at_depth(n)?;
    ctx.push(value)?;
    Ok(())
}

/// Load value from scratch space
pub fn op_load(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let index = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?; // advance past immediate

    let value = ctx.get_scratch(index)?.clone();
    ctx.push(value)?;
    Ok(())
}

/// Store value to scratch space
pub fn op_store(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let index = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?; // advance past immediate

    let value = ctx.pop()?;
    ctx.set_scratch(index, value)?;
    Ok(())
}

/// Get byte array length
pub fn op_len(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let len = match val {
        StackValue::Bytes(bytes) => bytes.len() as u64,
        StackValue::Uint(_) => 8, // Uint is 8 bytes
    };

    ctx.push(StackValue::Uint(len))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Convert integer to bytes (big-endian)
pub fn op_itob(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let int_val = val.as_uint()?;

    let bytes = int_val.to_be_bytes().to_vec();
    ctx.push(StackValue::Bytes(bytes))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Convert bytes to integer (big-endian)
pub fn op_btoi(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let bytes = val.as_bytes()?;

    if bytes.is_empty() {
        ctx.push(StackValue::Uint(0))?;
        ctx.advance_pc(1)?;
        return Ok(());
    }

    if bytes.len() > 8 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 8,
            actual: bytes.len(),
        });
    }

    // Pad with zeros if necessary
    let mut padded = vec![0u8; 8];
    let start_idx = 8 - bytes.len();
    padded[start_idx..].copy_from_slice(bytes);

    let int_val = u64::from_be_bytes(padded.try_into().unwrap());
    ctx.push(StackValue::Uint(int_val))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Concatenate two byte arrays
pub fn op_concat(ctx: &mut EvalContext) -> AvmResult<()> {
    let b = ctx.pop()?;
    let a = ctx.pop()?;

    let a_bytes = a.as_bytes()?;
    let b_bytes = b.as_bytes()?;

    let mut result = Vec::with_capacity(a_bytes.len() + b_bytes.len());
    result.extend_from_slice(a_bytes);
    result.extend_from_slice(b_bytes);

    ctx.push(StackValue::Bytes(result))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Extract substring with immediate start and length
pub fn op_substring(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let start = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past start parameter
    let length = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past length parameter

    let val = ctx.pop()?;
    let bytes = val.as_bytes()?;

    if start >= bytes.len() || start + length > bytes.len() {
        return Err(AvmError::InvalidByteArrayLength {
            expected: start + length,
            actual: bytes.len(),
        });
    }

    let result = bytes[start..start + length].to_vec();
    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}

/// Extract substring with stack arguments
pub fn op_substring3(ctx: &mut EvalContext) -> AvmResult<()> {
    let end = ctx.pop()?;
    let start = ctx.pop()?;
    let val = ctx.pop()?;

    let start_idx = start.as_uint()? as usize;
    let end_idx = end.as_uint()? as usize;
    let bytes = val.as_bytes()?;

    if start_idx >= bytes.len() || end_idx > bytes.len() || start_idx > end_idx {
        return Err(AvmError::InvalidByteArrayLength {
            expected: end_idx,
            actual: bytes.len(),
        });
    }

    let result = bytes[start_idx..end_idx].to_vec();
    ctx.push(StackValue::Bytes(result))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Duplicate N values from top of stack
pub fn op_dupn(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let n = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past parameter

    if n == 0 {
        return Ok(());
    }

    if ctx.stack_size() < n {
        return Err(AvmError::StackUnderflow);
    }

    // This is a simplified implementation - would need direct stack access for efficiency
    let mut values = Vec::new();
    for _ in 0..n {
        values.push(ctx.pop()?);
    }

    // Restore original values
    for val in values.iter().rev() {
        ctx.push(val.clone())?;
    }

    // Push duplicated values
    for val in values.iter().rev() {
        ctx.push(val.clone())?;
    }

    Ok(())
}

/// Pop N values from top of stack
pub fn op_popn(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let n = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past parameter

    if ctx.stack_size() < n {
        return Err(AvmError::StackUnderflow);
    }

    for _ in 0..n {
        ctx.pop()?;
    }

    Ok(())
}
