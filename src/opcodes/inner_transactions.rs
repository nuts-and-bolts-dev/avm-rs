//! Inner transaction opcodes for creating and submitting transactions from smart contracts

use crate::error::{AvmError, AvmResult};
use crate::types::StackValue;
use crate::vm::EvalContext;

/// Inner transaction field types
#[derive(Debug, Clone)]
pub enum InnerTransactionField {
    Type,
    TypeEnum,
    Sender,
    Fee,
    FirstValid,
    FirstValidTime,
    LastValid,
    Note,
    Lease,
    Receiver,
    Amount,
    CloseRemainderTo,
    VotePK,
    SelectionPK,
    VoteFirst,
    VoteLast,
    VoteKeyDilution,
    StateProofPK,
    AssetAmount,
    AssetSender,
    AssetReceiver,
    AssetCloseTo,
    XferAsset,
    AssetFrozen,
    FreezeAsset,
    FreezeAssetAccount,
    ConfigAsset,
    ConfigAssetTotal,
    ConfigAssetDecimals,
    ConfigAssetDefaultFrozen,
    ConfigAssetName,
    ConfigAssetUnitName,
    ConfigAssetURL,
    ConfigAssetMetadataHash,
    ConfigAssetManager,
    ConfigAssetReserve,
    ConfigAssetFreeze,
    ConfigAssetClawback,
    ApplicationID,
    OnCompletion,
    ApplicationArgs,
    Accounts,
    Assets,
    Applications,
    GlobalNumUint,
    GlobalNumByteSlice,
    LocalNumUint,
    LocalNumByteSlice,
    ExtraProgramPages,
    ApprovalProgram,
    ClearStateProgram,
    CreatedAssetID,
    CreatedApplicationID,
    LastLog,
    NumLogs,
}

impl InnerTransactionField {
    /// Convert field ID to field type
    pub fn from_id(id: u8) -> AvmResult<Self> {
        match id {
            0 => Ok(Self::Type),
            1 => Ok(Self::TypeEnum),
            2 => Ok(Self::Sender),
            3 => Ok(Self::Fee),
            4 => Ok(Self::FirstValid),
            5 => Ok(Self::FirstValidTime),
            6 => Ok(Self::LastValid),
            7 => Ok(Self::Note),
            8 => Ok(Self::Lease),
            9 => Ok(Self::Receiver),
            10 => Ok(Self::Amount),
            11 => Ok(Self::CloseRemainderTo),
            12 => Ok(Self::VotePK),
            13 => Ok(Self::SelectionPK),
            14 => Ok(Self::VoteFirst),
            15 => Ok(Self::VoteLast),
            16 => Ok(Self::VoteKeyDilution),
            17 => Ok(Self::StateProofPK),
            18 => Ok(Self::AssetAmount),
            19 => Ok(Self::AssetSender),
            20 => Ok(Self::AssetReceiver),
            21 => Ok(Self::AssetCloseTo),
            22 => Ok(Self::XferAsset),
            23 => Ok(Self::AssetFrozen),
            24 => Ok(Self::FreezeAsset),
            25 => Ok(Self::FreezeAssetAccount),
            26 => Ok(Self::ConfigAsset),
            27 => Ok(Self::ConfigAssetTotal),
            28 => Ok(Self::ConfigAssetDecimals),
            29 => Ok(Self::ConfigAssetDefaultFrozen),
            30 => Ok(Self::ConfigAssetName),
            31 => Ok(Self::ConfigAssetUnitName),
            32 => Ok(Self::ConfigAssetURL),
            33 => Ok(Self::ConfigAssetMetadataHash),
            34 => Ok(Self::ConfigAssetManager),
            35 => Ok(Self::ConfigAssetReserve),
            36 => Ok(Self::ConfigAssetFreeze),
            37 => Ok(Self::ConfigAssetClawback),
            38 => Ok(Self::ApplicationID),
            39 => Ok(Self::OnCompletion),
            40 => Ok(Self::ApplicationArgs),
            41 => Ok(Self::Accounts),
            42 => Ok(Self::Assets),
            43 => Ok(Self::Applications),
            44 => Ok(Self::GlobalNumUint),
            45 => Ok(Self::GlobalNumByteSlice),
            46 => Ok(Self::LocalNumUint),
            47 => Ok(Self::LocalNumByteSlice),
            48 => Ok(Self::ExtraProgramPages),
            49 => Ok(Self::ApprovalProgram),
            50 => Ok(Self::ClearStateProgram),
            51 => Ok(Self::CreatedAssetID),
            52 => Ok(Self::CreatedApplicationID),
            53 => Ok(Self::LastLog),
            54 => Ok(Self::NumLogs),
            _ => Err(AvmError::invalid_program(format!(
                "Invalid inner transaction field: {}",
                id
            ))),
        }
    }
}

/// Log an event (limited to application mode)
pub fn op_log(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let data = ctx.pop()?;
    let log_data = data.as_bytes()?;

    // Log limit is 1024 bytes
    if log_data.len() > 1024 {
        return Err(AvmError::invalid_program("Log data exceeds 1024 bytes"));
    }

    // In a real implementation, this would add to the transaction's log array
    // For now, we'll just validate the operation

    Ok(())
}

/// Begin construction of an inner transaction
pub fn op_itxn_begin(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    // Initialize inner transaction construction
    // In a real implementation, this would set up a new transaction context

    Ok(())
}

/// Set field for current inner transaction
pub fn op_itxn_field(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let value = ctx.pop()?;
    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would set the field value in the current inner transaction
    match value {
        StackValue::Uint(_) => {
            // Handle uint fields
        }
        StackValue::Bytes(_) => {
            // Handle byte fields
        }
    }

    Ok(())
}

/// Submit current inner transaction
pub fn op_itxn_submit(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    // In a real implementation, this would validate and submit the inner transaction
    // and add it to the transaction group

    Ok(())
}

/// Access field from last submitted inner transaction
pub fn op_itxn(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would get the field from the last inner transaction
    // For now, return placeholder values
    match field_id {
        // Uint fields
        1 | 3 | 4 | 5 | 6 | 10 | 14 | 15 | 16 | 18 | 23 | 27 | 28 | 29 | 38 | 39 | 44 | 45 | 46
        | 47 | 48 | 51 | 52 | 54 => {
            ctx.push(StackValue::Uint(0))?;
        }
        // Bytes fields
        _ => {
            ctx.push(StackValue::Bytes(Vec::new()))?;
        }
    }

    Ok(())
}

/// Access array field from last submitted inner transaction
pub fn op_itxna(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;
    let _index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would get the indexed field from the last inner transaction
    // For now, return placeholder values
    match field_id {
        40 | 41 | 42 | 43 => {
            // Array fields
            ctx.push(StackValue::Bytes(Vec::new()))?;
        }
        _ => {
            return Err(AvmError::invalid_program("Field is not an array"));
        }
    }

    Ok(())
}

/// Begin construction of next inner transaction in group
pub fn op_itxn_next(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;

    // In a real implementation, this would start construction of the next inner transaction

    Ok(())
}

/// Access field from specific inner transaction in group
pub fn op_gitxn(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let _group_index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would get the field from the specified inner transaction
    // For now, return placeholder values
    match field_id {
        // Uint fields
        1 | 3 | 4 | 5 | 6 | 10 | 14 | 15 | 16 | 18 | 23 | 27 | 28 | 29 | 38 | 39 | 44 | 45 | 46
        | 47 | 48 | 51 | 52 | 54 => {
            ctx.push(StackValue::Uint(0))?;
        }
        // Bytes fields
        _ => {
            ctx.push(StackValue::Bytes(Vec::new()))?;
        }
    }

    Ok(())
}

/// Access array field from specific inner transaction in group
pub fn op_gitxna(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let _group_index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;
    let _index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;

    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would get the indexed field from the specified inner transaction
    // For now, return placeholder values
    match field_id {
        40 | 41 | 42 | 43 => {
            // Array fields
            ctx.push(StackValue::Bytes(Vec::new()))?;
        }
        _ => {
            return Err(AvmError::invalid_program("Field is not an array"));
        }
    }

    Ok(())
}

/// Access array field from last submitted inner transaction (stack index)
pub fn op_itxnas(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let index = ctx.pop()?;
    let _idx = index.as_uint()? as usize;

    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would get the indexed field from the last inner transaction
    // For now, return placeholder values
    match field_id {
        40 | 41 | 42 | 43 => {
            // Array fields
            ctx.push(StackValue::Bytes(Vec::new()))?;
        }
        _ => {
            return Err(AvmError::invalid_program("Field is not an array"));
        }
    }

    Ok(())
}

/// Access array field from specific inner transaction in group (stack index)
pub fn op_gitxnas(ctx: &mut EvalContext) -> AvmResult<()> {
    ctx.advance_pc(1)?;
    let _group_index = ctx.read_bytes(1)?[0] as usize;
    ctx.advance_pc(1)?;
    let field_id = ctx.read_bytes(1)?[0];
    ctx.advance_pc(1)?;

    let index = ctx.pop()?;
    let _idx = index.as_uint()? as usize;

    let _field = InnerTransactionField::from_id(field_id)?;

    // In a real implementation, this would get the indexed field from the specified inner transaction
    // For now, return placeholder values
    match field_id {
        40 | 41 | 42 | 43 => {
            // Array fields
            ctx.push(StackValue::Bytes(Vec::new()))?;
        }
        _ => {
            return Err(AvmError::invalid_program("Field is not an array"));
        }
    }

    Ok(())
}
