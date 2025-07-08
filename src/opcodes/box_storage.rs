//! Box storage opcodes for persistent key-value storage in smart contracts

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;

/// Create a new box with the given name and size
pub fn op_box_create(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let size = ctx.pop()?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;
    let size_value = size.as_uint()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // Box size is limited to 32,768 bytes
    if size_value > 32768 {
        return Err(AvmError::invalid_program("Box size exceeds 32,768 bytes"));
    }

    // In a real implementation, this would:
    // 1. Check if box already exists
    // 2. Create the box in persistent storage
    // 3. Return 1 if created, 0 if already exists

    // For now, always return success (1)
    ctx.push(StackValue::Uint(1))?;
    Ok(())
}

/// Extract bytes from a box
pub fn op_box_extract(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let length = ctx.pop()?;
    let start = ctx.pop()?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;
    let _start_idx = start.as_uint()? as usize;
    let len = length.as_uint()? as usize;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Extract the specified range of bytes
    // 3. Return the extracted bytes

    // For now, return empty bytes as placeholder
    ctx.push(StackValue::Bytes(vec![0u8; len]))?;
    Ok(())
}

/// Replace bytes in a box
pub fn op_box_replace(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let replacement = ctx.pop()?;
    let start = ctx.pop()?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;
    let _start_idx = start.as_uint()? as usize;
    let _repl_bytes = replacement.as_bytes()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Replace the bytes at the specified offset
    // 3. Validate that the replacement fits within the box

    Ok(())
}

/// Delete a box
pub fn op_box_del(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Delete the box from storage
    // 3. Return 1 if deleted, 0 if not found

    // For now, always return success (1)
    ctx.push(StackValue::Uint(1))?;
    Ok(())
}

/// Get the length of a box
pub fn op_box_len(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Return the box length and existence flag

    // For now, return length=0 and exists=1
    ctx.push(StackValue::Uint(0))?; // length
    ctx.push(StackValue::Uint(1))?; // exists
    Ok(())
}

/// Get the entire contents of a box
pub fn op_box_get(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Return the entire box contents and existence flag

    // For now, return empty contents and exists=1
    ctx.push(StackValue::Bytes(Vec::new()))?; // contents
    ctx.push(StackValue::Uint(1))?; // exists
    Ok(())
}

/// Put bytes into a box (overwrite entire contents)
pub fn op_box_put(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let value = ctx.pop()?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;
    let value_bytes = value.as_bytes()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // Box contents are limited to 32,768 bytes
    if value_bytes.len() > 32768 {
        return Err(AvmError::invalid_program("Box value exceeds 32,768 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name (must exist)
    // 2. Replace the entire contents with the new value
    // 3. Validate that the new value fits in the box size

    Ok(())
}

/// Splice bytes into a box (insert/replace with size change)
pub fn op_box_splice(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let replacement = ctx.pop()?;
    let length = ctx.pop()?;
    let start = ctx.pop()?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;
    let _start_idx = start.as_uint()? as usize;
    let _len = length.as_uint()? as usize;
    let _repl_bytes = replacement.as_bytes()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Remove 'length' bytes starting at 'start'
    // 3. Insert the replacement bytes at 'start'
    // 4. Validate the resulting box size doesn't exceed limits

    Ok(())
}

/// Resize a box
pub fn op_box_resize(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let new_size = ctx.pop()?;
    let name = ctx.pop()?;

    let name_bytes = name.as_bytes()?;
    let size_value = new_size.as_uint()?;

    // Box names are limited to 64 bytes
    if name_bytes.len() > 64 {
        return Err(AvmError::invalid_program("Box name exceeds 64 bytes"));
    }

    // Box size is limited to 32,768 bytes
    if size_value > 32768 {
        return Err(AvmError::invalid_program("Box size exceeds 32,768 bytes"));
    }

    // In a real implementation, this would:
    // 1. Look up the box by name
    // 2. Resize the box to the new size
    // 3. If shrinking, truncate contents
    // 4. If growing, pad with zeros

    Ok(())
}
