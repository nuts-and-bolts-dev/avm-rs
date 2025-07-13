//! Argument access opcodes

use crate::error::AvmResult;
use crate::types::StackValue;
use crate::vm::EvalContext;

/// Access argument with immediate index
pub fn op_arg(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past immediate

    let args = ctx.ledger().program_args()?;
    if index < args.len() {
        ctx.push(StackValue::Bytes(args[index].clone()))?;
    } else {
        ctx.push(StackValue::Bytes(vec![]))?; // Empty bytes for out of bounds
    }
    Ok(())
}

/// Access argument 0
pub fn op_arg_0(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    let args = ctx.ledger().program_args()?;
    if !args.is_empty() {
        ctx.push(StackValue::Bytes(args[0].clone()))?;
    } else {
        ctx.push(StackValue::Bytes(vec![]))?;
    }
    Ok(())
}

/// Access argument 1
pub fn op_arg_1(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    let args = ctx.ledger().program_args()?;
    if args.len() > 1 {
        ctx.push(StackValue::Bytes(args[1].clone()))?;
    } else {
        ctx.push(StackValue::Bytes(vec![]))?;
    }
    Ok(())
}

/// Access argument 2
pub fn op_arg_2(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    let args = ctx.ledger().program_args()?;
    if args.len() > 2 {
        ctx.push(StackValue::Bytes(args[2].clone()))?;
    } else {
        ctx.push(StackValue::Bytes(vec![]))?;
    }
    Ok(())
}

/// Access argument 3
pub fn op_arg_3(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    let args = ctx.ledger().program_args()?;
    if args.len() > 3 {
        ctx.push(StackValue::Bytes(args[3].clone()))?;
    } else {
        ctx.push(StackValue::Bytes(vec![]))?;
    }
    Ok(())
}

/// Access arguments with stack index
pub fn op_args(ctx: &mut EvalContext) -> AvmResult<()> {
    let index = ctx.pop()?.as_uint()? as usize;
    ctx.advance_pc(1)?;

    let args = ctx.ledger().program_args()?;
    if index < args.len() {
        ctx.push(StackValue::Bytes(args[index].clone()))?;
    } else {
        ctx.push(StackValue::Bytes(vec![]))?; // Empty bytes for out of bounds
    }
    Ok(())
}
