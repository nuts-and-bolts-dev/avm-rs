//! Flow control opcodes

use crate::error::{AvmError, AvmResult};
use crate::vm::EvalContext;

/// Branch if not zero
pub fn op_bnz(ctx: &mut EvalContext) -> AvmResult<()> {
    // Save the PC at start of instruction for offset calculation
    let instruction_pc = ctx.pc();

    // Advance past the opcode to read offset
    ctx.advance_pc(1)?;
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    // PC after full instruction (opcode + 2-byte offset)
    let pc_after_instruction = instruction_pc + 3;

    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if condition {
        // Calculate target with proper bounds checking
        let target = pc_after_instruction as i32 + offset as i32;
        if target < 0 || target as usize > ctx.program_len() {
            return Err(AvmError::ProgramCounterOutOfBounds {
                pc: target as usize,
                program_len: ctx.program_len(),
            });
        }
        ctx.set_pc(target as usize)?;
    } else {
        // Not branching - set PC to after instruction
        ctx.set_pc(pc_after_instruction)?;
    }

    Ok(())
}

/// Branch if zero
pub fn op_bz(ctx: &mut EvalContext) -> AvmResult<()> {
    // Save the PC at start of instruction for offset calculation
    let instruction_pc = ctx.pc();

    // Advance past the opcode to read offset
    ctx.advance_pc(1)?;
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    // PC after full instruction (opcode + 2-byte offset)
    let pc_after_instruction = instruction_pc + 3;

    let val = ctx.pop()?;
    let condition = val.as_bool()?;

    if !condition {
        // Calculate target with proper bounds checking
        let target = pc_after_instruction as i32 + offset as i32;
        if target < 0 || target as usize > ctx.program_len() {
            return Err(AvmError::ProgramCounterOutOfBounds {
                pc: target as usize,
                program_len: ctx.program_len(),
            });
        }
        ctx.set_pc(target as usize)?;
    } else {
        // Not branching - set PC to after instruction
        ctx.set_pc(pc_after_instruction)?;
    }

    Ok(())
}

/// Unconditional branch
pub fn op_b(ctx: &mut EvalContext) -> AvmResult<()> {
    // Save the PC at start of instruction for offset calculation
    let instruction_pc = ctx.pc();

    // Advance past the opcode to read offset
    ctx.advance_pc(1)?;
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    // PC after full instruction (opcode + 2-byte offset)
    let pc_after_instruction = instruction_pc + 3;

    // Calculate target with proper bounds checking
    let target = pc_after_instruction as i32 + offset as i32;
    if target < 0 || target as usize > ctx.program_len() {
        return Err(AvmError::ProgramCounterOutOfBounds {
            pc: target as usize,
            program_len: ctx.program_len(),
        });
    }
    ctx.set_pc(target as usize)?;

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
    // Save the PC at start of instruction for offset calculation
    let instruction_pc = ctx.pc();

    // Advance past the opcode to read offset
    ctx.advance_pc(1)?;
    let offset_bytes = ctx.read_bytes(2)?.to_vec();
    let offset = i16::from_be_bytes([offset_bytes[0], offset_bytes[1]]);

    // PC after full instruction (opcode + 2-byte offset) - this is the return address
    let return_address = instruction_pc + 3;

    // Calculate target with proper bounds checking
    let target = return_address as i32 + offset as i32;
    if target < 0 || target as usize > ctx.program_len() {
        return Err(AvmError::ProgramCounterOutOfBounds {
            pc: target as usize,
            program_len: ctx.program_len(),
        });
    }

    // Set PC to return address before calling subroutine
    ctx.set_pc(return_address)?;
    ctx.call_subroutine(target as usize)?;

    Ok(())
}

/// Return from subroutine
pub fn op_retsub(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.return_from_subroutine()?;
    Ok(())
}
