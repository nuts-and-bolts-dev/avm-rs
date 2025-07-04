//! Transaction field access opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::{GlobalField, StackValue, TxnField};
use crate::vm::EvalContext;

/// Access transaction field
pub fn op_txn(ctx: &mut EvalContext) -> AvmResult<()> {
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let field = parse_txn_field(field_id)?;
    let value = get_txn_field(ctx, field)?;

    ctx.push(value)?;
    Ok(())
}

/// Access transaction field array element
pub fn op_txna(ctx: &mut EvalContext) -> AvmResult<()> {
    let field_id = ctx.read_bytes(1)?[0];
    let array_index = ctx.read_bytes(1)?[0];
    ctx.advance_pc(2)?;

    let field = parse_txn_field(field_id)?;
    let value = get_txn_field_array(ctx, field, array_index as usize)?;

    ctx.push(value)?;
    Ok(())
}

/// Access transaction field with stack index
pub fn op_txnas(ctx: &mut EvalContext) -> AvmResult<()> {
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let index = ctx.pop()?;
    let array_index = index.as_uint()? as usize;

    let field = parse_txn_field(field_id)?;
    let value = get_txn_field_array(ctx, field, array_index)?;

    ctx.push(value)?;
    Ok(())
}

/// Access group transaction field
pub fn op_gtxn(ctx: &mut EvalContext) -> AvmResult<()> {
    let group_index = ctx.read_bytes(1)?[0] as usize;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(2)?;

    if group_index >= ctx.group_size() {
        return Err(AvmError::invalid_program(format!(
            "Group index {} out of bounds (group size: {})",
            group_index,
            ctx.group_size()
        )));
    }

    let field = parse_txn_field(field_id)?;
    let value = get_group_txn_field(ctx, group_index, field)?;

    ctx.push(value)?;
    Ok(())
}

/// Access group transaction field array element
pub fn op_gtxna(ctx: &mut EvalContext) -> AvmResult<()> {
    let group_index = ctx.read_bytes(1)?[0] as usize;
    let field_id = ctx.read_bytes(1)?[0];
    let array_index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(3)?;

    if group_index >= ctx.group_size() {
        return Err(AvmError::invalid_program(format!(
            "Group index {} out of bounds (group size: {})",
            group_index,
            ctx.group_size()
        )));
    }

    let field = parse_txn_field(field_id)?;
    let value = get_group_txn_field_array(ctx, group_index, field, array_index)?;

    ctx.push(value)?;
    Ok(())
}

/// Access group transaction field with stack index
pub fn op_gtxns(ctx: &mut EvalContext) -> AvmResult<()> {
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let index = ctx.pop()?;
    let group_index = index.as_uint()? as usize;

    if group_index >= ctx.group_size() {
        return Err(AvmError::invalid_program(format!(
            "Group index {} out of bounds (group size: {})",
            group_index,
            ctx.group_size()
        )));
    }

    let field = parse_txn_field(field_id)?;
    let value = get_group_txn_field(ctx, group_index, field)?;

    ctx.push(value)?;
    Ok(())
}

/// Access group transaction field array with stack indices
pub fn op_gtxnsa(ctx: &mut EvalContext) -> AvmResult<()> {
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let array_index = ctx.pop()?;
    let group_index = ctx.pop()?;

    let group_idx = group_index.as_uint()? as usize;
    let array_idx = array_index.as_uint()? as usize;

    if group_idx >= ctx.group_size() {
        return Err(AvmError::invalid_program(format!(
            "Group index {} out of bounds (group size: {})",
            group_idx,
            ctx.group_size()
        )));
    }

    let field = parse_txn_field(field_id)?;
    let value = get_group_txn_field_array(ctx, group_idx, field, array_idx)?;

    ctx.push(value)?;
    Ok(())
}

/// Access global field
pub fn op_global(ctx: &mut EvalContext) -> AvmResult<()> {
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let field = parse_global_field(field_id)?;
    let value = get_global_field(ctx, field)?;

    ctx.push(value)?;
    Ok(())
}

/// Parse transaction field ID
fn parse_txn_field(field_id: u8) -> AvmResult<TxnField> {
    match field_id {
        0 => Ok(TxnField::Sender),
        1 => Ok(TxnField::Fee),
        2 => Ok(TxnField::FirstValid),
        3 => Ok(TxnField::FirstValidTime),
        4 => Ok(TxnField::LastValid),
        5 => Ok(TxnField::Note),
        6 => Ok(TxnField::Lease),
        7 => Ok(TxnField::Receiver),
        8 => Ok(TxnField::Amount),
        9 => Ok(TxnField::CloseRemainderTo),
        10 => Ok(TxnField::VotePK),
        11 => Ok(TxnField::SelectionPK),
        12 => Ok(TxnField::VoteFirst),
        13 => Ok(TxnField::VoteLast),
        14 => Ok(TxnField::VoteKeyDilution),
        15 => Ok(TxnField::Type),
        16 => Ok(TxnField::TypeEnum),
        17 => Ok(TxnField::XferAsset),
        18 => Ok(TxnField::AssetAmount),
        19 => Ok(TxnField::AssetSender),
        20 => Ok(TxnField::AssetReceiver),
        21 => Ok(TxnField::AssetCloseTo),
        22 => Ok(TxnField::GroupIndex),
        23 => Ok(TxnField::TxID),
        24 => Ok(TxnField::ApplicationID),
        25 => Ok(TxnField::OnCompletion),
        26 => Ok(TxnField::ApplicationArgs),
        27 => Ok(TxnField::NumAppArgs),
        28 => Ok(TxnField::Accounts),
        29 => Ok(TxnField::NumAccounts),
        30 => Ok(TxnField::ApprovalProgram),
        31 => Ok(TxnField::ClearStateProgram),
        32 => Ok(TxnField::RekeyTo),
        _ => Err(AvmError::InvalidTransactionField {
            field: format!("Unknown field ID: {field_id}"),
        }),
    }
}

/// Parse global field ID
fn parse_global_field(field_id: u8) -> AvmResult<GlobalField> {
    match field_id {
        0 => Ok(GlobalField::MinTxnFee),
        1 => Ok(GlobalField::MinBalance),
        2 => Ok(GlobalField::MaxTxnLife),
        3 => Ok(GlobalField::ZeroAddress),
        4 => Ok(GlobalField::GroupSize),
        5 => Ok(GlobalField::LogicSigVersion),
        6 => Ok(GlobalField::Round),
        7 => Ok(GlobalField::LatestTimestamp),
        8 => Ok(GlobalField::CurrentApplicationID),
        9 => Ok(GlobalField::CreatorAddress),
        10 => Ok(GlobalField::CurrentApplicationAddress),
        11 => Ok(GlobalField::GroupID),
        12 => Ok(GlobalField::OpcodeBudget),
        13 => Ok(GlobalField::CallerApplicationID),
        14 => Ok(GlobalField::CallerApplicationAddress),
        _ => Err(AvmError::InvalidGlobalField {
            field: format!("Unknown field ID: {field_id}"),
        }),
    }
}

/// Get transaction field value (placeholder implementation)
fn get_txn_field(_ctx: &EvalContext, field: TxnField) -> AvmResult<StackValue> {
    // Placeholder implementation - in a real implementation,
    // this would access the actual transaction data
    match field {
        TxnField::Sender => Ok(StackValue::Bytes(vec![0u8; 32])),
        TxnField::Fee => Ok(StackValue::Uint(1000)),
        TxnField::FirstValid => Ok(StackValue::Uint(1000)),
        TxnField::LastValid => Ok(StackValue::Uint(2000)),
        TxnField::Note => Ok(StackValue::Bytes(vec![])),
        TxnField::Lease => Ok(StackValue::Bytes(vec![0u8; 32])),
        TxnField::Receiver => Ok(StackValue::Bytes(vec![0u8; 32])),
        TxnField::Amount => Ok(StackValue::Uint(0)),
        TxnField::GroupIndex => Ok(StackValue::Uint(0)),
        TxnField::TxID => Ok(StackValue::Bytes(vec![0u8; 32])),
        TxnField::ApplicationID => Ok(StackValue::Uint(0)),
        TxnField::TypeEnum => Ok(StackValue::Uint(6)), // Application call
        _ => Ok(StackValue::Uint(0)),
    }
}

/// Get transaction field array value (placeholder)
fn get_txn_field_array(
    _ctx: &EvalContext,
    field: TxnField,
    _index: usize,
) -> AvmResult<StackValue> {
    match field {
        TxnField::ApplicationArgs => Ok(StackValue::Bytes(vec![])),
        TxnField::Accounts => Ok(StackValue::Bytes(vec![0u8; 32])),
        _ => Err(AvmError::InvalidTransactionField {
            field: format!("Field {field:?} is not an array"),
        }),
    }
}

/// Get group transaction field value (placeholder)
fn get_group_txn_field(
    ctx: &EvalContext,
    _group_index: usize,
    field: TxnField,
) -> AvmResult<StackValue> {
    // For now, just delegate to current transaction
    get_txn_field(ctx, field)
}

/// Get group transaction field array value (placeholder)
fn get_group_txn_field_array(
    ctx: &EvalContext,
    _group_index: usize,
    field: TxnField,
    index: usize,
) -> AvmResult<StackValue> {
    // For now, just delegate to current transaction
    get_txn_field_array(ctx, field, index)
}

/// Get global field value
fn get_global_field(ctx: &EvalContext, field: GlobalField) -> AvmResult<StackValue> {
    match field {
        GlobalField::MinTxnFee => Ok(StackValue::Uint(1000)),
        GlobalField::MinBalance => Ok(StackValue::Uint(100000)),
        GlobalField::MaxTxnLife => Ok(StackValue::Uint(1000)),
        GlobalField::ZeroAddress => Ok(StackValue::Bytes(vec![0u8; 32])),
        GlobalField::GroupSize => Ok(StackValue::Uint(ctx.group_size() as u64)),
        GlobalField::LogicSigVersion => Ok(StackValue::Uint(ctx.version() as u64)),
        GlobalField::Round => {
            let round = ctx.ledger().current_round()?;
            Ok(StackValue::Uint(round))
        }
        GlobalField::LatestTimestamp => {
            let timestamp = ctx.ledger().latest_timestamp()?;
            Ok(StackValue::Uint(timestamp))
        }
        GlobalField::CurrentApplicationID => {
            let app_id = ctx.ledger().current_application_id()?;
            Ok(StackValue::Uint(app_id))
        }
        GlobalField::CreatorAddress => {
            let addr = ctx.ledger().creator_address()?;
            Ok(StackValue::Bytes(addr))
        }
        GlobalField::CurrentApplicationAddress => {
            let addr = ctx.ledger().current_application_address()?;
            Ok(StackValue::Bytes(addr))
        }
        GlobalField::GroupID => {
            let group_id = ctx.ledger().group_id()?;
            Ok(StackValue::Bytes(group_id))
        }
        GlobalField::OpcodeBudget => {
            let budget = ctx.ledger().opcode_budget()?;
            Ok(StackValue::Uint(budget))
        }
        GlobalField::CallerApplicationID => {
            let caller_id = ctx.ledger().caller_application_id()?;
            Ok(StackValue::Uint(caller_id.unwrap_or(0)))
        }
        GlobalField::CallerApplicationAddress => {
            let caller_addr = ctx.ledger().caller_application_address()?;
            Ok(StackValue::Bytes(caller_addr.unwrap_or_default()))
        }
        _ => Ok(StackValue::Uint(0)),
    }
}
