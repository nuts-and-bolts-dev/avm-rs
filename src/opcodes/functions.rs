//! Function control opcodes

use crate::error::AvmResult;
use crate::vm::EvalContext;

/// Function prototype declaration
pub fn op_proto(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let args = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past immediate
    let returns = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past immediate

    // Proto just declares function signature, doesn't execute anything
    // Store the function prototype information in context
    ctx.set_function_prototype(args, returns)?;

    Ok(())
}

/// Frame dig - access value from function frame
pub fn op_frame_dig(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let depth = ctx.read_bytes(1)?[0] as i8;
    ctx.advance_pc(1)?; // advance past immediate

    let value = ctx.frame_dig(depth)?;
    ctx.push(value)?;

    Ok(())
}

/// Frame bury - store value in function frame
pub fn op_frame_bury(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode
    let depth = ctx.read_bytes(1)?[0] as i8;
    ctx.advance_pc(1)?; // advance past immediate

    let value = ctx.pop()?;
    ctx.frame_bury(depth, value)?;

    Ok(())
}

/// Switch statement - jump to one of many targets
pub fn op_switch(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode

    // Read number of targets
    let num_targets = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past count

    // Read all target offsets
    let mut targets = Vec::new();
    for _ in 0..num_targets {
        let target_bytes = ctx.read_bytes(2)?;
        let target = i16::from_be_bytes([target_bytes[0], target_bytes[1]]);
        targets.push(target);
        ctx.advance_pc(2)?;
    }

    // Pop the switch value
    let switch_value = ctx.pop()?;
    let index = switch_value.as_uint()? as usize;

    // Jump to target or fall through
    if index < targets.len() {
        let target = targets[index];
        ctx.branch(target)?;
    }
    // If index is out of bounds, just continue (fall through)

    Ok(())
}

/// Match statement - like switch but matches specific values
pub fn op_match(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?; // advance past opcode

    // Read number of match cases
    let num_cases = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?; // advance past count

    // Read all match values and targets
    let mut cases = Vec::new();
    for _ in 0..num_cases {
        // Read match value (8 bytes for u64)
        let value_bytes = ctx.read_bytes(8)?;
        let match_value = u64::from_be_bytes([
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
            value_bytes[4],
            value_bytes[5],
            value_bytes[6],
            value_bytes[7],
        ]);
        ctx.advance_pc(8)?;

        // Read target offset (2 bytes)
        let target_bytes = ctx.read_bytes(2)?;
        let target = i16::from_be_bytes([target_bytes[0], target_bytes[1]]);
        ctx.advance_pc(2)?;

        cases.push((match_value, target));
    }

    // Pop the value to match
    let match_value = ctx.pop()?;
    let value = match_value.as_uint()?;

    // Find matching case and jump
    for (case_value, target) in cases {
        if value == case_value {
            ctx.branch(target)?;
            return Ok(());
        }
    }

    // No match found, fall through
    Ok(())
}
