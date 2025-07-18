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

/// TEAL version enum for type-safe version handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TealVersion {
    /// TEAL version 1 (initial version)
    V1 = 1,
    /// TEAL version 2 (added more opcodes)
    V2 = 2,
    /// TEAL version 3 (added asset opcodes)
    V3 = 3,
    /// TEAL version 4 (added more crypto opcodes)
    V4 = 4,
    /// TEAL version 5 (added application opcodes)
    V5 = 5,
    /// TEAL version 6 (added more opcodes)
    V6 = 6,
    /// TEAL version 7 (added inner transactions)
    V7 = 7,
    /// TEAL version 8 (added box storage)
    V8 = 8,
    /// TEAL version 9 (added more box operations)
    V9 = 9,
    /// TEAL version 10 (added elliptic curve operations)
    V10 = 10,
    /// TEAL version 11 (added MIMC hash and block opcode)
    V11 = 11,
}

impl TealVersion {
    /// Convert from u8 to TealVersion
    pub fn from_u8(version: u8) -> Result<Self, crate::error::AvmError> {
        match version {
            1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            3 => Ok(Self::V3),
            4 => Ok(Self::V4),
            5 => Ok(Self::V5),
            6 => Ok(Self::V6),
            7 => Ok(Self::V7),
            8 => Ok(Self::V8),
            9 => Ok(Self::V9),
            10 => Ok(Self::V10),
            11 => Ok(Self::V11),
            _ => Err(crate::error::AvmError::UnsupportedVersion(version)),
        }
    }

    /// Convert to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Get the latest supported version
    pub const fn latest() -> Self {
        Self::V11
    }

    /// Check if this version supports a specific feature
    pub fn supports_subroutines(self) -> bool {
        self >= Self::V4
    }

    /// Check if this version supports inner transactions
    pub fn supports_inner_transactions(self) -> bool {
        self >= Self::V5
    }

    /// Check if this version supports box operations
    pub fn supports_boxes(self) -> bool {
        self >= Self::V8
    }

    /// Check if this version supports advanced crypto operations
    pub fn supports_advanced_crypto(self) -> bool {
        self >= Self::V5
    }

    /// Check if this version supports extended box operations (splice, resize)
    pub fn supports_extended_box_ops(self) -> bool {
        self >= Self::V9
    }

    /// Check if this version supports elliptic curve operations
    pub fn supports_elliptic_curve_ops(self) -> bool {
        self >= Self::V10
    }

    /// Check if this version supports MIMC hash and block randomness
    pub fn supports_mimc_and_block(self) -> bool {
        self >= Self::V11
    }

    /// Get all available versions
    pub const fn all() -> &'static [Self] {
        &[
            Self::V1,
            Self::V2,
            Self::V3,
            Self::V4,
            Self::V5,
            Self::V6,
            Self::V7,
            Self::V8,
            Self::V9,
            Self::V10,
            Self::V11,
        ]
    }
}

impl fmt::Display for TealVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl Default for TealVersion {
    fn default() -> Self {
        Self::latest()
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
