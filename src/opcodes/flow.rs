//! Flow control opcodes

use crate::error::{AvmError, AvmResult};
use crate::vm::EvalContext;

/// Branch if not zero
/// TODO: Conditional branches in complex tests cause arithmetic underflow
/// Branch logic may execute wrong path leading to invalid operations (5-10=underflow)
/// Jump offset calculations need verification against TEAL specification
pub fn op_bnz(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read 2-byte offset
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    ctx.advance_pc(2)?;
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if condition {
        // Offset is relative to the PC after the instruction
        let target = (ctx.pc() as i32 + offset as i32) as usize;
        ctx.set_pc(target)?;
    }

    Ok(())
}

/// Branch if zero
/// TODO: Conditional branches in complex tests cause arithmetic underflow
/// Branch logic may execute wrong path leading to invalid operations (5-10=underflow)
/// Jump offset calculations need verification against TEAL specification
pub fn op_bz(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read 2-byte offset
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    ctx.advance_pc(2)?;
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if !condition {
        // Offset is relative to the PC after the instruction
        let target = (ctx.pc() as i32 + offset as i32) as usize;
        ctx.set_pc(target)?;
    }

    Ok(())
}

/// Unconditional branch
/// TODO: Complex flow tests fail with IntegerOverflow - branching logic incorrect
/// Jump offset calculations and conditional branches need debugging
/// Branch targets may be calculated incorrectly causing wrong execution paths
pub fn op_b(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read 2-byte offset
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    ctx.advance_pc(2)?;
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    // Offset is relative to the PC after the instruction
    let target = (ctx.pc() as i32 + offset as i32) as usize;
    ctx.set_pc(target)?;

    Ok(())
}

/// Return from program
pub fn op_return(ctx: &mut EvalContext) -> AvmResult<()> {
    // Set PC to end of program to signal completion
    let program_len = ctx.program_len();
    ctx.set_pc(program_len)?;
    // Don't advance PC further as we're at the end
    Ok(())
}

/// Assert that value is not zero
pub fn op_assert(ctx: &mut EvalContext) -> AvmResult<()> {
    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if !condition {
        return Err(AvmError::execution_halted("assert failed"));
    }

    ctx.advance_pc(1)?;
    Ok(())
}

/// Call subroutine
pub fn op_callsub(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read 2-byte offset
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    ctx.advance_pc(2)?;
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    // Offset is relative to the PC after the instruction
    let target = (ctx.pc() as i32 + offset as i32) as usize;
    ctx.call_subroutine(target)?;

    Ok(())
}

/// Return from subroutine
pub fn op_retsub(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.return_from_subroutine()?;
    Ok(())
}
