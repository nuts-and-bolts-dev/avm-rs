//! Constant loading opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::varuint::read_varuint_from_context;
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

    // Read count as varuint
    let count = read_varuint_from_context(ctx)? as usize;

    // Read each integer constant as varuint
    let mut constants = Vec::with_capacity(count);
    for _ in 0..count {
        let value = read_varuint_from_context(ctx)?;
        constants.push(value);
    }

    // Store constants in context
    ctx.set_int_constants(constants);
    Ok(())
}

/// Load integer constant
pub fn op_intc(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    let index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // Load from constant block
    let value = ctx.get_int_constant(index)?;
    ctx.push(StackValue::Uint(value))?;
    Ok(())
}

/// Load integer constant 0
pub fn op_intc_0(ctx: &mut EvalContext) -> AvmResult<()> {
    let value = ctx.get_int_constant(0)?;
    ctx.push(StackValue::Uint(value))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Load integer constant 1
pub fn op_intc_1(ctx: &mut EvalContext) -> AvmResult<()> {
    let value = ctx.get_int_constant(1)?;
    ctx.push(StackValue::Uint(value))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Load integer constant 2
pub fn op_intc_2(ctx: &mut EvalContext) -> AvmResult<()> {
    let value = ctx.get_int_constant(2)?;
    ctx.push(StackValue::Uint(value))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Load integer constant 3
pub fn op_intc_3(ctx: &mut EvalContext) -> AvmResult<()> {
    let value = ctx.get_int_constant(3)?;
    ctx.push(StackValue::Uint(value))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Byte constant block
pub fn op_bytecblock(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;

    // Read count as varuint
    let count = read_varuint_from_context(ctx)? as usize;

    // Read each byte constant (length-prefixed)
    let mut constants = Vec::with_capacity(count);
    for _ in 0..count {
        let length = read_varuint_from_context(ctx)? as usize;
        let bytes = ctx.read_bytes(length)?.to_vec();
        ctx.advance_pc(length)?;
        constants.push(bytes);
    }

    // Store constants in context
    ctx.set_byte_constants(constants);
    Ok(())
}

/// Load byte constant
pub fn op_bytec(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    let index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    // Load from constant block
    let bytes = ctx.get_byte_constant(index)?;
    ctx.push(StackValue::Bytes(bytes.to_vec()))?;
    Ok(())
}

/// Load byte constant 0
pub fn op_bytec_0(ctx: &mut EvalContext) -> AvmResult<()> {
    let bytes = ctx.get_byte_constant(0)?;
    ctx.push(StackValue::Bytes(bytes.to_vec()))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Load byte constant 1
pub fn op_bytec_1(ctx: &mut EvalContext) -> AvmResult<()> {
    let bytes = ctx.get_byte_constant(1)?;
    ctx.push(StackValue::Bytes(bytes.to_vec()))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Load byte constant 2
pub fn op_bytec_2(ctx: &mut EvalContext) -> AvmResult<()> {
    let bytes = ctx.get_byte_constant(2)?;
    ctx.push(StackValue::Bytes(bytes.to_vec()))?;
    ctx.advance_pc(1)?;
    Ok(())
}

/// Load byte constant 3
pub fn op_bytec_3(ctx: &mut EvalContext) -> AvmResult<()> {
    let bytes = ctx.get_byte_constant(3)?;
    ctx.push(StackValue::Bytes(bytes.to_vec()))?;
    ctx.advance_pc(1)?;
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
    ctx.advance_pc(1)?;
    Ok(())
}
