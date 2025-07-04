//! Core data types for the Algorand Virtual Machine

use serde::{Deserialize, Serialize};
use std::fmt;

/// Stack value type that can hold different types of data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StackValue {
    /// Unsigned 64-bit integer
    Uint(u64),
    /// Byte array (used for addresses, hashes, strings, etc.)
    Bytes(Vec<u8>),
}

impl StackValue {
    /// Create a new uint value
    pub fn uint(value: u64) -> Self {
        Self::Uint(value)
    }

    /// Create a new bytes value
    pub fn bytes(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }

    /// Create a new bytes value from a string
    pub fn from_string(s: &str) -> Self {
        Self::Bytes(s.as_bytes().to_vec())
    }

    /// Get the uint value, returning an error if not a uint
    pub fn as_uint(&self) -> Result<u64, crate::error::AvmError> {
        match self {
            Self::Uint(val) => Ok(*val),
            Self::Bytes(_) => Err(crate::error::AvmError::TypeError {
                expected: "uint".to_string(),
                actual: "bytes".to_string(),
            }),
        }
    }

    /// Get the bytes value, returning an error if not bytes
    pub fn as_bytes(&self) -> Result<&[u8], crate::error::AvmError> {
        match self {
            Self::Bytes(bytes) => Ok(bytes),
            Self::Uint(_) => Err(crate::error::AvmError::TypeError {
                expected: "bytes".to_string(),
                actual: "uint".to_string(),
            }),
        }
    }

    /// Get the bytes value as a mutable reference
    pub fn as_bytes_mut(&mut self) -> Result<&mut Vec<u8>, crate::error::AvmError> {
        match self {
            Self::Bytes(bytes) => Ok(bytes),
            Self::Uint(_) => Err(crate::error::AvmError::TypeError {
                expected: "bytes".to_string(),
                actual: "uint".to_string(),
            }),
        }
    }

    /// Convert to bool (0 is false, non-zero is true)
    pub fn as_bool(&self) -> Result<bool, crate::error::AvmError> {
        match self {
            Self::Uint(val) => Ok(*val != 0),
            Self::Bytes(bytes) => {
                // Empty bytes or all zeros is false
                Ok(!bytes.is_empty() && bytes.iter().any(|&b| b != 0))
            }
        }
    }

    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Uint(_) => "uint",
            Self::Bytes(_) => "bytes",
        }
    }

    /// Check if this is a uint value
    pub fn is_uint(&self) -> bool {
        matches!(self, Self::Uint(_))
    }

    /// Check if this is a bytes value
    pub fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(_))
    }

    /// Convert to uint if possible, otherwise return 0
    pub fn uint_or_zero(&self) -> u64 {
        match self {
            Self::Uint(val) => *val,
            Self::Bytes(_) => 0,
        }
    }
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint(val) => write!(f, "{val}"),
            Self::Bytes(bytes) => {
                if bytes.iter().all(|&b| b.is_ascii() && !b.is_ascii_control()) {
                    write!(f, "\"{}\"", String::from_utf8_lossy(bytes))
                } else {
                    write!(f, "0x{}", hex::encode(bytes))
                }
            }
        }
    }
}

impl From<u64> for StackValue {
    fn from(value: u64) -> Self {
        Self::Uint(value)
    }
}

impl From<Vec<u8>> for StackValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&[u8]> for StackValue {
    fn from(value: &[u8]) -> Self {
        Self::Bytes(value.to_vec())
    }
}

impl From<String> for StackValue {
    fn from(value: String) -> Self {
        Self::Bytes(value.into_bytes())
    }
}

impl From<&str> for StackValue {
    fn from(value: &str) -> Self {
        Self::Bytes(value.as_bytes().to_vec())
    }
}

/// TEAL value type used in global and local state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TealValue {
    /// Unsigned 64-bit integer
    Uint(u64),
    /// Byte array
    Bytes(Vec<u8>),
}

impl TealValue {
    /// Create a new uint value
    pub fn uint(value: u64) -> Self {
        Self::Uint(value)
    }

    /// Create a new bytes value
    pub fn bytes(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }

    /// Convert to StackValue
    pub fn to_stack_value(&self) -> StackValue {
        match self {
            Self::Uint(val) => StackValue::Uint(*val),
            Self::Bytes(bytes) => StackValue::Bytes(bytes.clone()),
        }
    }

    /// Create from StackValue
    pub fn from_stack_value(value: &StackValue) -> Self {
        match value {
            StackValue::Uint(val) => Self::Uint(*val),
            StackValue::Bytes(bytes) => Self::Bytes(bytes.clone()),
        }
    }
}

impl fmt::Display for TealValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint(val) => write!(f, "{val}"),
            Self::Bytes(bytes) => {
                if bytes.iter().all(|&b| b.is_ascii() && !b.is_ascii_control()) {
                    write!(f, "\"{}\"", String::from_utf8_lossy(bytes))
                } else {
                    write!(f, "0x{}", hex::encode(bytes))
                }
            }
        }
    }
}

/// Run mode for the AVM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunMode {
    /// Signature verification mode (stateless)
    Signature,
    /// Application mode (stateful)
    Application,
}

/// Transaction field identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxnField {
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
    Type,
    TypeEnum,
    XferAsset,
    AssetAmount,
    AssetSender,
    AssetReceiver,
    AssetCloseTo,
    GroupIndex,
    TxID,
    ApplicationID,
    OnCompletion,
    ApplicationArgs,
    NumAppArgs,
    Accounts,
    NumAccounts,
    ApprovalProgram,
    ClearStateProgram,
    RekeyTo,
    ConfigAsset,
    ConfigAssetTotal,
    ConfigAssetDecimals,
    ConfigAssetDefaultFrozen,
    ConfigAssetName,
    ConfigAssetURL,
    ConfigAssetMetadataHash,
    ConfigAssetManager,
    ConfigAssetReserve,
    ConfigAssetFreeze,
    ConfigAssetClawback,
    FreezeAsset,
    FreezeAssetAccount,
    FreezeAssetFrozen,
    Assets,
    NumAssets,
    Applications,
    NumApplications,
    GlobalNumUint,
    GlobalNumByteSlice,
    LocalNumUint,
    LocalNumByteSlice,
    ExtraProgramPages,
    Nonparticipation,
    Logs,
    NumLogs,
    CreatedAssetID,
    CreatedApplicationID,
    LastLog,
    StateProofPK,
    ApprovalProgramPages,
    NumApprovalProgramPages,
    ClearStateProgramPages,
    NumClearStateProgramPages,
}

/// Global field identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalField {
    MinTxnFee,
    MinBalance,
    MaxTxnLife,
    ZeroAddress,
    GroupSize,
    LogicSigVersion,
    Round,
    LatestTimestamp,
    CurrentApplicationID,
    CreatorAddress,
    CurrentApplicationAddress,
    GroupID,
    OpcodeBudget,
    CallerApplicationID,
    CallerApplicationAddress,
    AssetCreateMinBalance,
    AssetOptInMinBalance,
    GenesisHash,
}
