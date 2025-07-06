//! Transaction field access opcodes

use crate::error::{AvmError, AvmResult};
use crate::types::{GlobalField, StackValue, TxnField};
use crate::vm::EvalContext;

/// Access transaction field
pub fn op_txn(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
    // Read the field ID parameter
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let field = parse_txn_field(field_id)?;
    let value = get_txn_field(ctx, field)?;

    ctx.push(value)?;
    Ok(())
}

/// Access transaction field array element
pub fn op_txna(ctx: &mut EvalContext) -> AvmResult<()> {
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
    // Advance past the opcode first
    ctx.advance_pc(1)?;
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
        33 => Ok(TxnField::ConfigAsset),
        34 => Ok(TxnField::ConfigAssetTotal),
        35 => Ok(TxnField::ConfigAssetDecimals),
        36 => Ok(TxnField::ConfigAssetDefaultFrozen),
        37 => Ok(TxnField::ConfigAssetName), // ConfigAssetUnitName in assembler
        38 => Ok(TxnField::ConfigAssetName),
        39 => Ok(TxnField::ConfigAssetURL),
        40 => Ok(TxnField::ConfigAssetMetadataHash),
        41 => Ok(TxnField::ConfigAssetManager),
        42 => Ok(TxnField::ConfigAssetReserve),
        43 => Ok(TxnField::ConfigAssetFreeze),
        44 => Ok(TxnField::ConfigAssetClawback),
        45 => Ok(TxnField::FreezeAsset),
        46 => Ok(TxnField::FreezeAssetAccount),
        47 => Ok(TxnField::FreezeAssetFrozen),
        48 => Ok(TxnField::Assets),
        49 => Ok(TxnField::NumAssets),
        50 => Ok(TxnField::Applications),
        51 => Ok(TxnField::NumApplications),
        52 => Ok(TxnField::GlobalNumUint),
        53 => Ok(TxnField::GlobalNumByteSlice),
        54 => Ok(TxnField::LocalNumUint),
        55 => Ok(TxnField::LocalNumByteSlice),
        56 => Ok(TxnField::ExtraProgramPages),
        57 => Ok(TxnField::Nonparticipation),
        58 => Ok(TxnField::Logs),
        59 => Ok(TxnField::NumLogs),
        60 => Ok(TxnField::CreatedAssetID),
        61 => Ok(TxnField::CreatedApplicationID),
        62 => Ok(TxnField::LastLog),
        63 => Ok(TxnField::StateProofPK),
        64 => Ok(TxnField::ApprovalProgramPages),
        65 => Ok(TxnField::NumApprovalProgramPages),
        66 => Ok(TxnField::ClearStateProgramPages),
        67 => Ok(TxnField::NumClearStateProgramPages),
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
        15 => Ok(GlobalField::AssetCreateMinBalance),
        16 => Ok(GlobalField::AssetOptInMinBalance),
        17 => Ok(GlobalField::GenesisHash),
        _ => Err(AvmError::InvalidGlobalField {
            field: format!("Unknown field ID: {field_id}"),
        }),
    }
}

/// Get transaction field value from the current transaction
fn get_txn_field(ctx: &EvalContext, field: TxnField) -> AvmResult<StackValue> {
    let teal_value = ctx.ledger().get_txn_field(ctx.group_index(), field)?;
    Ok(teal_value.to_stack_value())
}

/// Get transaction field array value from the current transaction
fn get_txn_field_array(ctx: &EvalContext, field: TxnField, index: usize) -> AvmResult<StackValue> {
    // For array fields, we need special handling
    match field {
        TxnField::ApplicationArgs => {
            let current_tx = ctx.ledger().current_transaction()?;
            if index < current_tx.application_args.len() {
                Ok(StackValue::Bytes(
                    current_tx.application_args[index].clone(),
                ))
            } else {
                Ok(StackValue::Bytes(vec![]))
            }
        }
        TxnField::Accounts => {
            let current_tx = ctx.ledger().current_transaction()?;
            if index < current_tx.accounts.len() {
                Ok(StackValue::Bytes(current_tx.accounts[index].clone()))
            } else {
                Ok(StackValue::Bytes(vec![0u8; 32]))
            }
        }
        _ => Err(AvmError::InvalidTransactionField {
            field: format!("Field {field:?} is not an array"),
        }),
    }
}

/// Get group transaction field value from a specific transaction in the group
fn get_group_txn_field(
    ctx: &EvalContext,
    group_index: usize,
    field: TxnField,
) -> AvmResult<StackValue> {
    let teal_value = ctx.ledger().get_txn_field(group_index, field)?;
    Ok(teal_value.to_stack_value())
}

/// Get group transaction field array value from a specific transaction in the group
fn get_group_txn_field_array(
    ctx: &EvalContext,
    group_index: usize,
    field: TxnField,
    index: usize,
) -> AvmResult<StackValue> {
    // Get the specific transaction from the group
    let transactions = ctx.ledger().transaction_group()?;
    if group_index >= transactions.len() {
        return Err(AvmError::invalid_program(format!(
            "Group index {} out of bounds (group size: {})",
            group_index,
            transactions.len()
        )));
    }

    let tx = &transactions[group_index];
    match field {
        TxnField::ApplicationArgs => {
            if index < tx.application_args.len() {
                Ok(StackValue::Bytes(tx.application_args[index].clone()))
            } else {
                Ok(StackValue::Bytes(vec![]))
            }
        }
        TxnField::Accounts => {
            if index < tx.accounts.len() {
                Ok(StackValue::Bytes(tx.accounts[index].clone()))
            } else {
                Ok(StackValue::Bytes(vec![0u8; 32]))
            }
        }
        _ => Err(AvmError::InvalidTransactionField {
            field: format!("Field {field:?} is not an array"),
        }),
    }
}

/// Get global field value using the ledger interface
fn get_global_field(ctx: &EvalContext, field: GlobalField) -> AvmResult<StackValue> {
    let teal_value = ctx.ledger().get_global_field(field)?;
    Ok(teal_value.to_stack_value())
}
