//! Constant loading opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;

/// Push immediate integer value
pub fn op_pushint(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read 8 bytes for the integer value
    let bytes = ctx.read_bytes(8)?.to_vec();
    ctx.advance_pc(8)?;

    let value = u64::from_be_bytes(bytes.try_into().unwrap());
    ctx.push(StackValue::Uint(value))?;
    Ok(())
}

/// Push immediate byte array
pub fn op_pushbytes(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read the length byte
    let length = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // Read the bytes
    let bytes = ctx.read_bytes(length)?.to_vec();
    ctx.advance_pc(length)?;

    ctx.push(StackValue::Bytes(bytes))?;
    Ok(())
}

/// Push multiple integers
pub fn op_pushints(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read count
    let count = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // Read each integer
    for _ in 0..count {
        let bytes = ctx.read_bytes(8)?.to_vec();
        ctx.advance_pc(8)?;

        let value = u64::from_be_bytes(bytes.try_into().unwrap());
        ctx.push(StackValue::Uint(value))?;
    }

    Ok(())
}

/// Push multiple byte arrays
pub fn op_pushbytess(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read count
    let count = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // Read each byte array
    for _ in 0..count {
        let length = ctx.read_bytes(1)?[0] as usize;
        ctx.advance_pc(1)?;

        let bytes = ctx.read_bytes(length)?.to_vec();
        ctx.advance_pc(length)?;

        ctx.push(StackValue::Bytes(bytes))?;
    }

    Ok(())
}

/// Integer constant block
pub fn op_intcblock(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read count
    let count = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // For now, just skip the constant block
    // Full implementation would store constants for later use
    for _ in 0..count {
        ctx.advance_pc(8)?; // Skip 8 bytes per integer
    }

    Ok(())
}

/// Load integer constant
pub fn op_intc(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    let index = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    // Placeholder - would load from constant block
    ctx.push(StackValue::Uint(index as u64))?;
    Ok(())
}

/// Load integer constant 0
pub fn op_intc_0(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Uint(0))?;
    Ok(())
}

/// Load integer constant 1
pub fn op_intc_1(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Uint(1))?;
    Ok(())
}

/// Load integer constant 2
pub fn op_intc_2(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Uint(2))?;
    Ok(())
}

/// Load integer constant 3
pub fn op_intc_3(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Uint(3))?;
    Ok(())
}

/// Byte constant block
pub fn op_bytecblock(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read count
    let count = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // For now, just skip the constant block
    // Full implementation would store constants for later use
    for _ in 0..count {
        let length = ctx.read_bytes(1)?[0] as usize;
        ctx.advance_pc(1 + length)?; // Skip length byte + data
    }

    Ok(())
}

/// Load byte constant
pub fn op_bytec(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    let index = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    // Placeholder - would load from constant block
    ctx.push(StackValue::Bytes(vec![index]))?;
    Ok(())
}

/// Load byte constant 0
pub fn op_bytec_0(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Bytes(vec![0]))?;
    Ok(())
}

/// Load byte constant 1
pub fn op_bytec_1(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Bytes(vec![1]))?;
    Ok(())
}

/// Load byte constant 2
pub fn op_bytec_2(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Bytes(vec![2]))?;
    Ok(())
}

/// Load byte constant 3
pub fn op_bytec_3(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.push(StackValue::Bytes(vec![3]))?;
    Ok(())
}

/// Create zero-filled byte array
pub fn op_bzero(ctx: &mut EvalContext) -> AvmResult<()> {
    let length = ctx.pop()?;
    let len = length.as_uint()? as usize;

    if len > 4096 {
        return Err(AvmError::InvalidByteArrayLength {
            expected: 4096,
            actual: len,
        });
    }

    let result = vec![0u8; len];
    ctx.push(StackValue::Bytes(result))?;
    Ok(())
}
