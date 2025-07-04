//! Flow control opcodes

use crate::error::{AvmError, AvmResult};
use crate::vm::EvalContext;

/// Branch if not zero
pub fn op_bnz(ctx: &mut EvalContext) -> AvmResult<()> {
    let offset = i16::from_be_bytes([ctx.read_bytes(1)?[0], ctx.read_bytes(1)?[0]]);
    ctx.advance_pc(2)?;

    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if condition {
        let target = (ctx.pc() as i32 + offset as i32) as usize;
        ctx.set_pc(target)?;
    }

    Ok(())
}

/// Branch if zero
pub fn op_bz(ctx: &mut EvalContext) -> AvmResult<()> {
    let offset = i16::from_be_bytes([ctx.read_bytes(1)?[0], ctx.read_bytes(1)?[0]]);
    ctx.advance_pc(2)?;

    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if !condition {
        let target = (ctx.pc() as i32 + offset as i32) as usize;
        ctx.set_pc(target)?;
    }

    Ok(())
}

/// Unconditional branch
pub fn op_b(ctx: &mut EvalContext) -> AvmResult<()> {
    let offset = i16::from_be_bytes([ctx.read_bytes(1)?[0], ctx.read_bytes(1)?[0]]);
    ctx.advance_pc(2)?;

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

    Ok(())
}

/// Call subroutine
pub fn op_callsub(ctx: &mut EvalContext) -> AvmResult<()> {
    let offset = i16::from_be_bytes([ctx.read_bytes(1)?[0], ctx.read_bytes(1)?[0]]);
    ctx.advance_pc(2)?;

    let target = (ctx.pc() as i32 + offset as i32) as usize;
    ctx.call_subroutine(target)?;

    Ok(())
}

/// Return from subroutine
pub fn op_retsub(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.return_from_subroutine()?;
    Ok(())
}
