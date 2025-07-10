//! State management interfaces and implementations

use crate::error::AvmResult;
use crate::types::{GlobalField, TealValue, TxnField};
use std::collections::HashMap;

/// Account address type
pub type Address = Vec<u8>;

/// Application ID type
pub type AppId = u64;

/// Asset ID type
pub type AssetId = u64;

/// Micro Algos type
pub type MicroAlgos = u64;

/// Transaction type
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Payment,
    KeyRegistration,
    AssetConfig,
    AssetTransfer,
    AssetFreeze,
    ApplicationCall,
    StateProof,
}

/// Transaction data
#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: Address,
    pub fee: MicroAlgos,
    pub first_valid: u64,
    pub first_valid_time: u64,
    pub last_valid: u64,
    pub note: Vec<u8>,
    pub lease: Vec<u8>,
    pub receiver: Option<Address>,
    pub amount: Option<MicroAlgos>,
    pub close_remainder_to: Option<Address>,
    pub vote_pk: Option<Vec<u8>>,
    pub selection_pk: Option<Vec<u8>>,
    pub vote_first: Option<u64>,
    pub vote_last: Option<u64>,
    pub vote_key_dilution: Option<u64>,
    pub tx_type: TransactionType,
    pub type_enum: u64,
    pub xfer_asset: Option<AssetId>,
    pub asset_amount: Option<u64>,
    pub asset_sender: Option<Address>,
    pub asset_receiver: Option<Address>,
    pub asset_close_to: Option<Address>,
    pub group_index: u64,
    pub tx_id: Vec<u8>,
    pub application_id: Option<AppId>,
    pub on_completion: Option<u64>,
    pub application_args: Vec<Vec<u8>>,
    pub accounts: Vec<Address>,
    pub approval_program: Option<Vec<u8>>,
    pub clear_state_program: Option<Vec<u8>>,
    pub rekey_to: Option<Address>,
    pub config_asset: Option<AssetId>,
    pub config_asset_total: Option<u64>,
    pub config_asset_decimals: Option<u8>,
    pub config_asset_default_frozen: Option<bool>,
    pub config_asset_unit_name: Option<String>,
    pub config_asset_name: Option<String>,
    pub config_asset_url: Option<String>,
    pub config_asset_metadata_hash: Option<Vec<u8>>,
    pub config_asset_manager: Option<Address>,
    pub config_asset_reserve: Option<Address>,
    pub config_asset_freeze: Option<Address>,
    pub config_asset_clawback: Option<Address>,
    pub freeze_asset: Option<AssetId>,
    pub freeze_asset_account: Option<Address>,
    pub freeze_asset_frozen: Option<bool>,
    pub assets: Vec<AssetId>,
    pub applications: Vec<AppId>,
    pub global_num_uint: Option<u64>,
    pub global_num_byte_slice: Option<u64>,
    pub local_num_uint: Option<u64>,
    pub local_num_byte_slice: Option<u64>,
    pub extra_program_pages: Option<u32>,
    pub nonparticipation: Option<bool>,
    pub logs: Vec<Vec<u8>>,
    pub created_asset_id: Option<AssetId>,
    pub created_application_id: Option<AppId>,
    pub last_log: Option<Vec<u8>>,
    pub state_proof_pk: Option<Vec<u8>>,
    pub approval_program_pages: Vec<Vec<u8>>,
    pub clear_state_program_pages: Vec<Vec<u8>>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

impl Transaction {
    /// Create a new default transaction
    pub fn new() -> Self {
        Self {
            sender: vec![0; 32],
            fee: 1000,
            first_valid: 1,
            first_valid_time: 0,
            last_valid: 1000,
            note: Vec::new(),
            lease: vec![0; 32],
            receiver: None,
            amount: None,
            close_remainder_to: None,
            vote_pk: None,
            selection_pk: None,
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            tx_type: TransactionType::Payment,
            type_enum: 1,
            xfer_asset: None,
            asset_amount: None,
            asset_sender: None,
            asset_receiver: None,
            asset_close_to: None,
            group_index: 0,
            tx_id: vec![0; 32],
            application_id: None,
            on_completion: None,
            application_args: Vec::new(),
            accounts: Vec::new(),
            approval_program: None,
            clear_state_program: None,
            rekey_to: None,
            config_asset: None,
            config_asset_total: None,
            config_asset_decimals: None,
            config_asset_default_frozen: None,
            config_asset_unit_name: None,
            config_asset_name: None,
            config_asset_url: None,
            config_asset_metadata_hash: None,
            config_asset_manager: None,
            config_asset_reserve: None,
            config_asset_freeze: None,
            config_asset_clawback: None,
            freeze_asset: None,
            freeze_asset_account: None,
            freeze_asset_frozen: None,
            assets: Vec::new(),
            applications: Vec::new(),
            global_num_uint: None,
            global_num_byte_slice: None,
            local_num_uint: None,
            local_num_byte_slice: None,
            extra_program_pages: None,
            nonparticipation: None,
            logs: Vec::new(),
            created_asset_id: None,
            created_application_id: None,
            last_log: None,
            state_proof_pk: None,
            approval_program_pages: Vec::new(),
            clear_state_program_pages: Vec::new(),
        }
    }

    /// Create a payment transaction
    pub fn payment(sender: Address, receiver: Address, amount: MicroAlgos) -> Self {
        let mut tx = Self::new();
        tx.sender = sender;
        tx.receiver = Some(receiver);
        tx.amount = Some(amount);
        tx.tx_type = TransactionType::Payment;
        tx.type_enum = 1;
        tx
    }

    /// Create an asset transfer transaction
    pub fn asset_transfer(
        sender: Address,
        receiver: Address,
        asset_id: AssetId,
        amount: u64,
    ) -> Self {
        let mut tx = Self::new();
        tx.sender = sender;
        tx.asset_receiver = Some(receiver);
        tx.xfer_asset = Some(asset_id);
        tx.asset_amount = Some(amount);
        tx.tx_type = TransactionType::AssetTransfer;
        tx.type_enum = 4;
        tx
    }
}

/// Asset holding information
#[derive(Debug, Clone)]
pub struct AssetHolding {
    pub amount: u64,
    pub frozen: bool,
}

/// Asset parameters
#[derive(Debug, Clone)]
pub struct AssetParams {
    pub total: u64,
    pub decimals: u8,
    pub default_frozen: bool,
    pub name: String,
    pub unit_name: String,
    pub url: String,
    pub metadata_hash: Vec<u8>,
    pub manager: Address,
    pub reserve: Address,
    pub freeze: Address,
    pub clawback: Address,
}

/// Application parameters
#[derive(Debug, Clone)]
pub struct AppParams {
    pub approval_program: Vec<u8>,
    pub clear_state_program: Vec<u8>,
    pub global_state_schema: StateSchema,
    pub local_state_schema: StateSchema,
    pub extra_program_pages: u32,
    pub creator: Address,
}

/// State schema defining storage allocation
#[derive(Debug, Clone)]
pub struct StateSchema {
    pub num_uint: u64,
    pub num_byte_slice: u64,
}

/// Account parameters
#[derive(Debug, Clone)]
pub struct AccountParams {
    pub micro_algos: MicroAlgos,
    pub rewards_base: u64,
    pub reward_algos: MicroAlgos,
    pub status: String,
    pub auth_addr: Option<Address>,
    pub total_apps_schema: StateSchema,
    pub total_apps_extra_pages: u32,
    pub total_assets: u64,
    pub total_created_assets: u64,
    pub total_created_apps: u64,
    pub total_boxes: u64,
    pub total_box_bytes: u64,
}

/// Trait for accessing ledger state
pub trait LedgerAccess: std::fmt::Debug {
    /// Get account balance
    fn balance(&self, addr: &Address) -> AvmResult<MicroAlgos>;

    /// Get minimum balance for account
    fn min_balance(&self, addr: &Address) -> AvmResult<MicroAlgos>;

    /// Get global state value
    fn app_global_get(&self, app_id: AppId, key: &str) -> AvmResult<Option<TealValue>>;

    /// Set global state value
    fn app_global_put(&mut self, app_id: AppId, key: &str, value: TealValue) -> AvmResult<()>;

    /// Delete global state value
    fn app_global_del(&mut self, app_id: AppId, key: &str) -> AvmResult<()>;

    /// Get local state value
    fn app_local_get(
        &self,
        addr: &Address,
        app_id: AppId,
        key: &str,
    ) -> AvmResult<Option<TealValue>>;

    /// Set local state value
    fn app_local_put(
        &mut self,
        addr: &Address,
        app_id: AppId,
        key: &str,
        value: TealValue,
    ) -> AvmResult<()>;

    /// Delete local state value
    fn app_local_del(&mut self, addr: &Address, app_id: AppId, key: &str) -> AvmResult<()>;

    /// Check if account has opted into application
    fn app_opted_in(&self, addr: &Address, app_id: AppId) -> AvmResult<bool>;

    /// Get asset holding information
    fn asset_holding(&self, addr: &Address, asset_id: AssetId) -> AvmResult<Option<AssetHolding>>;

    /// Get asset parameters
    fn asset_params(&self, asset_id: AssetId) -> AvmResult<Option<AssetParams>>;

    /// Get application parameters
    fn app_params(&self, app_id: AppId) -> AvmResult<Option<AppParams>>;

    /// Get account parameters
    fn account_params(&self, addr: &Address) -> AvmResult<Option<AccountParams>>;

    /// Get current round number
    fn current_round(&self) -> AvmResult<u64>;

    /// Get latest timestamp
    fn latest_timestamp(&self) -> AvmResult<u64>;

    /// Get genesis hash
    fn genesis_hash(&self) -> AvmResult<Vec<u8>>;

    /// Get current application ID (for application mode)
    fn current_application_id(&self) -> AvmResult<AppId>;

    /// Get creator address for current application
    fn creator_address(&self) -> AvmResult<Address>;

    /// Get current application address
    fn current_application_address(&self) -> AvmResult<Address>;

    /// Get group ID for current transaction group
    fn group_id(&self) -> AvmResult<Vec<u8>>;

    /// Get opcode budget for current execution
    fn opcode_budget(&self) -> AvmResult<u64>;

    /// Get caller application ID (for inner transactions)
    fn caller_application_id(&self) -> AvmResult<Option<AppId>>;

    /// Get caller application address (for inner transactions)
    fn caller_application_address(&self) -> AvmResult<Option<Address>>;

    /// Get transaction field value
    fn get_txn_field(&self, txn_index: usize, field: TxnField) -> AvmResult<TealValue>;

    /// Get global field value
    fn get_global_field(&self, field: GlobalField) -> AvmResult<TealValue>;

    /// Get current transaction (cloned)
    fn current_transaction(&self) -> AvmResult<Transaction>;

    /// Get transaction group (cloned)
    fn transaction_group(&self) -> AvmResult<Vec<Transaction>>;

    /// Get program arguments for current transaction (cloned)
    fn program_args(&self) -> AvmResult<Vec<Vec<u8>>>;
}

/// Mock ledger implementation for testing
#[derive(Debug)]
pub struct MockLedger {
    balances: HashMap<Address, MicroAlgos>,
    min_balances: HashMap<Address, MicroAlgos>,
    global_state: HashMap<(AppId, String), TealValue>,
    local_state: HashMap<(Address, AppId, String), TealValue>,
    opted_in: HashMap<(Address, AppId), bool>,
    asset_holdings: HashMap<(Address, AssetId), AssetHolding>,
    asset_params: HashMap<AssetId, AssetParams>,
    app_params: HashMap<AppId, AppParams>,
    account_params: HashMap<Address, AccountParams>,
    current_round: u64,
    latest_timestamp: u64,
    genesis_hash: Vec<u8>,
    current_app_id: AppId,
    creator_addr: Address,
    current_app_addr: Address,
    group_id: Vec<u8>,
    opcode_budget: u64,
    caller_app_id: Option<AppId>,
    caller_app_addr: Option<Address>,
    transactions: Vec<Transaction>,
    current_txn_index: usize,
    program_args: Vec<Vec<u8>>,
}

impl MockLedger {
    /// Create a new mock ledger
    pub fn new() -> Self {
        Self::default()
    }

    /// Set account balance
    pub fn set_balance(&mut self, addr: Address, balance: MicroAlgos) {
        self.balances.insert(addr, balance);
    }

    /// Set minimum balance
    pub fn set_min_balance(&mut self, addr: Address, min_balance: MicroAlgos) {
        self.min_balances.insert(addr, min_balance);
    }

    /// Set global state
    pub fn set_global_state(&mut self, app_id: AppId, key: String, value: TealValue) {
        self.global_state.insert((app_id, key), value);
    }

    /// Set local state
    pub fn set_local_state(&mut self, addr: Address, app_id: AppId, key: String, value: TealValue) {
        self.local_state.insert((addr, app_id, key), value);
    }

    /// Set application opt-in status
    pub fn set_opted_in(&mut self, addr: Address, app_id: AppId, opted_in: bool) {
        self.opted_in.insert((addr, app_id), opted_in);
    }

    /// Set asset holding
    pub fn set_asset_holding(&mut self, addr: Address, asset_id: AssetId, holding: AssetHolding) {
        self.asset_holdings.insert((addr, asset_id), holding);
    }

    /// Set asset parameters
    pub fn set_asset_params(&mut self, asset_id: AssetId, params: AssetParams) {
        self.asset_params.insert(asset_id, params);
    }

    /// Set application parameters
    pub fn set_app_params(&mut self, app_id: AppId, params: AppParams) {
        self.app_params.insert(app_id, params);
    }

    /// Set account parameters
    pub fn set_account_params(&mut self, addr: Address, params: AccountParams) {
        self.account_params.insert(addr, params);
    }

    /// Set current round
    pub fn set_current_round(&mut self, round: u64) {
        self.current_round = round;
    }

    /// Set latest timestamp
    pub fn set_latest_timestamp(&mut self, timestamp: u64) {
        self.latest_timestamp = timestamp;
    }

    /// Set genesis hash
    pub fn set_genesis_hash(&mut self, hash: Vec<u8>) {
        self.genesis_hash = hash;
    }

    /// Set current application ID
    pub fn set_current_application_id(&mut self, app_id: AppId) {
        self.current_app_id = app_id;
    }

    /// Set creator address
    pub fn set_creator_address(&mut self, addr: Address) {
        self.creator_addr = addr;
    }

    /// Set current application address
    pub fn set_current_application_address(&mut self, addr: Address) {
        self.current_app_addr = addr;
    }

    /// Set group ID
    pub fn set_group_id(&mut self, group_id: Vec<u8>) {
        self.group_id = group_id;
    }

    /// Set opcode budget
    pub fn set_opcode_budget(&mut self, budget: u64) {
        self.opcode_budget = budget;
    }

    /// Set caller application ID
    pub fn set_caller_application_id(&mut self, app_id: Option<AppId>) {
        self.caller_app_id = app_id;
    }

    /// Set caller application address
    pub fn set_caller_application_address(&mut self, addr: Option<Address>) {
        self.caller_app_addr = addr;
    }

    /// Add a transaction to the group
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Set the current transaction index
    pub fn set_current_transaction_index(&mut self, index: usize) {
        self.current_txn_index = index;
    }

    /// Set program arguments
    pub fn set_program_args(&mut self, args: Vec<Vec<u8>>) {
        self.program_args = args;
    }

    /// Set up a simple payment transaction group
    pub fn setup_payment_transaction(
        &mut self,
        sender: Address,
        receiver: Address,
        amount: MicroAlgos,
    ) {
        let tx = Transaction::payment(sender, receiver, amount);
        self.transactions.clear();
        self.transactions.push(tx);
        self.current_txn_index = 0;
    }

    /// Set up an asset transfer transaction group
    pub fn setup_asset_transfer(
        &mut self,
        sender: Address,
        receiver: Address,
        asset_id: AssetId,
        amount: u64,
    ) {
        let tx = Transaction::asset_transfer(sender, receiver, asset_id, amount);
        self.transactions.clear();
        self.transactions.push(tx);
        self.current_txn_index = 0;
    }

    /// Clear all transactions
    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
        self.current_txn_index = 0;
    }

    /// Create a mock ledger with realistic defaults
    pub fn with_defaults() -> Self {
        let mut ledger = Self {
            balances: HashMap::new(),
            min_balances: HashMap::new(),
            global_state: HashMap::new(),
            local_state: HashMap::new(),
            opted_in: HashMap::new(),
            asset_holdings: HashMap::new(),
            asset_params: HashMap::new(),
            app_params: HashMap::new(),
            account_params: HashMap::new(),
            current_round: 1000,
            latest_timestamp: 1640995200, // 2022-01-01
            genesis_hash: vec![0; 32],
            current_app_id: 0,
            creator_addr: Vec::new(),
            current_app_addr: Vec::new(),
            group_id: Vec::new(),
            opcode_budget: 700,
            caller_app_id: None,
            caller_app_addr: None,
            transactions: Vec::new(),
            current_txn_index: 0,
            program_args: Vec::new(),
        };

        // Add a default payment transaction
        let sender = vec![1; 32];
        let receiver = vec![2; 32];
        ledger.setup_payment_transaction(sender, receiver, 1_000_000);

        ledger
    }
}

impl Default for MockLedger {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl LedgerAccess for MockLedger {
    fn balance(&self, addr: &Address) -> AvmResult<MicroAlgos> {
        Ok(self.balances.get(addr).copied().unwrap_or(0))
    }

    fn min_balance(&self, addr: &Address) -> AvmResult<MicroAlgos> {
        Ok(self.min_balances.get(addr).copied().unwrap_or(100000))
    }

    fn app_global_get(&self, app_id: AppId, key: &str) -> AvmResult<Option<TealValue>> {
        Ok(self.global_state.get(&(app_id, key.to_string())).cloned())
    }

    fn app_global_put(&mut self, app_id: AppId, key: &str, value: TealValue) -> AvmResult<()> {
        self.global_state.insert((app_id, key.to_string()), value);
        Ok(())
    }

    fn app_global_del(&mut self, app_id: AppId, key: &str) -> AvmResult<()> {
        self.global_state.remove(&(app_id, key.to_string()));
        Ok(())
    }

    fn app_local_get(
        &self,
        addr: &Address,
        app_id: AppId,
        key: &str,
    ) -> AvmResult<Option<TealValue>> {
        Ok(self
            .local_state
            .get(&(addr.clone(), app_id, key.to_string()))
            .cloned())
    }

    fn app_local_put(
        &mut self,
        addr: &Address,
        app_id: AppId,
        key: &str,
        value: TealValue,
    ) -> AvmResult<()> {
        self.local_state
            .insert((addr.clone(), app_id, key.to_string()), value);
        Ok(())
    }

    fn app_local_del(&mut self, addr: &Address, app_id: AppId, key: &str) -> AvmResult<()> {
        self.local_state
            .remove(&(addr.clone(), app_id, key.to_string()));
        Ok(())
    }

    fn app_opted_in(&self, addr: &Address, app_id: AppId) -> AvmResult<bool> {
        Ok(self
            .opted_in
            .get(&(addr.clone(), app_id))
            .copied()
            .unwrap_or(false))
    }

    fn asset_holding(&self, addr: &Address, asset_id: AssetId) -> AvmResult<Option<AssetHolding>> {
        Ok(self.asset_holdings.get(&(addr.clone(), asset_id)).cloned())
    }

    fn asset_params(&self, asset_id: AssetId) -> AvmResult<Option<AssetParams>> {
        Ok(self.asset_params.get(&asset_id).cloned())
    }

    fn app_params(&self, app_id: AppId) -> AvmResult<Option<AppParams>> {
        Ok(self.app_params.get(&app_id).cloned())
    }

    fn account_params(&self, addr: &Address) -> AvmResult<Option<AccountParams>> {
        Ok(self.account_params.get(addr).cloned())
    }

    fn current_round(&self) -> AvmResult<u64> {
        Ok(self.current_round)
    }

    fn latest_timestamp(&self) -> AvmResult<u64> {
        Ok(self.latest_timestamp)
    }

    fn genesis_hash(&self) -> AvmResult<Vec<u8>> {
        Ok(self.genesis_hash.clone())
    }

    fn current_application_id(&self) -> AvmResult<AppId> {
        Ok(self.current_app_id)
    }

    fn creator_address(&self) -> AvmResult<Address> {
        Ok(self.creator_addr.clone())
    }

    fn current_application_address(&self) -> AvmResult<Address> {
        Ok(self.current_app_addr.clone())
    }

    fn group_id(&self) -> AvmResult<Vec<u8>> {
        Ok(self.group_id.clone())
    }

    fn opcode_budget(&self) -> AvmResult<u64> {
        Ok(self.opcode_budget)
    }

    fn caller_application_id(&self) -> AvmResult<Option<AppId>> {
        Ok(self.caller_app_id)
    }

    fn caller_application_address(&self) -> AvmResult<Option<Address>> {
        Ok(self.caller_app_addr.clone())
    }

    fn get_txn_field(&self, txn_index: usize, field: TxnField) -> AvmResult<TealValue> {
        let tx = self.transactions.get(txn_index).ok_or_else(|| {
            crate::error::AvmError::InvalidTransactionField {
                field: format!("transaction index {txn_index}"),
            }
        })?;

        use TxnField::*;
        let value = match field {
            Sender => TealValue::Bytes(tx.sender.clone()),
            Fee => TealValue::Uint(tx.fee),
            FirstValid => TealValue::Uint(tx.first_valid),
            FirstValidTime => TealValue::Uint(tx.first_valid_time),
            LastValid => TealValue::Uint(tx.last_valid),
            Note => TealValue::Bytes(tx.note.clone()),
            Lease => TealValue::Bytes(tx.lease.clone()),
            Receiver => TealValue::Bytes(tx.receiver.clone().unwrap_or_default()),
            Amount => TealValue::Uint(tx.amount.unwrap_or(0)),
            CloseRemainderTo => TealValue::Bytes(tx.close_remainder_to.clone().unwrap_or_default()),
            VotePK => TealValue::Bytes(tx.vote_pk.clone().unwrap_or_default()),
            SelectionPK => TealValue::Bytes(tx.selection_pk.clone().unwrap_or_default()),
            VoteFirst => TealValue::Uint(tx.vote_first.unwrap_or(0)),
            VoteLast => TealValue::Uint(tx.vote_last.unwrap_or(0)),
            VoteKeyDilution => TealValue::Uint(tx.vote_key_dilution.unwrap_or(0)),
            Type => TealValue::Bytes(match tx.tx_type {
                TransactionType::Payment => b"pay".to_vec(),
                TransactionType::KeyRegistration => b"keyreg".to_vec(),
                TransactionType::AssetConfig => b"acfg".to_vec(),
                TransactionType::AssetTransfer => b"axfer".to_vec(),
                TransactionType::AssetFreeze => b"afrz".to_vec(),
                TransactionType::ApplicationCall => b"appl".to_vec(),
                TransactionType::StateProof => b"stpf".to_vec(),
            }),
            TypeEnum => TealValue::Uint(tx.type_enum),
            XferAsset => TealValue::Uint(tx.xfer_asset.unwrap_or(0)),
            AssetAmount => TealValue::Uint(tx.asset_amount.unwrap_or(0)),
            AssetSender => TealValue::Bytes(tx.asset_sender.clone().unwrap_or_default()),
            AssetReceiver => TealValue::Bytes(tx.asset_receiver.clone().unwrap_or_default()),
            AssetCloseTo => TealValue::Bytes(tx.asset_close_to.clone().unwrap_or_default()),
            GroupIndex => TealValue::Uint(tx.group_index),
            TxID => TealValue::Bytes(tx.tx_id.clone()),
            ApplicationID => TealValue::Uint(tx.application_id.unwrap_or(0)),
            OnCompletion => TealValue::Uint(tx.on_completion.unwrap_or(0)),
            ApplicationArgs => {
                // For now, return the first application arg if it exists
                TealValue::Bytes(tx.application_args.first().cloned().unwrap_or_default())
            }
            NumAppArgs => TealValue::Uint(tx.application_args.len() as u64),
            Accounts => {
                // For now, return the first account if it exists
                TealValue::Bytes(tx.accounts.first().cloned().unwrap_or_default())
            }
            NumAccounts => TealValue::Uint(tx.accounts.len() as u64),
            ApprovalProgram => TealValue::Bytes(tx.approval_program.clone().unwrap_or_default()),
            ClearStateProgram => {
                TealValue::Bytes(tx.clear_state_program.clone().unwrap_or_default())
            }
            RekeyTo => TealValue::Bytes(tx.rekey_to.clone().unwrap_or_default()),
            _ => TealValue::Uint(0), // Default for other fields
        };

        Ok(value)
    }

    fn get_global_field(&self, field: GlobalField) -> AvmResult<TealValue> {
        use GlobalField::*;
        let value = match field {
            MinTxnFee => TealValue::Uint(1000),
            MinBalance => TealValue::Uint(100000),
            MaxTxnLife => TealValue::Uint(1000),
            ZeroAddress => TealValue::Bytes(vec![0; 32]),
            GroupSize => TealValue::Uint(self.transactions.len() as u64),
            LogicSigVersion => TealValue::Uint(8),
            Round => TealValue::Uint(self.current_round),
            LatestTimestamp => TealValue::Uint(self.latest_timestamp),
            CurrentApplicationID => TealValue::Uint(self.current_app_id),
            CreatorAddress => TealValue::Bytes(self.creator_addr.clone()),
            CurrentApplicationAddress => TealValue::Bytes(self.current_app_addr.clone()),
            GroupID => TealValue::Bytes(self.group_id.clone()),
            OpcodeBudget => TealValue::Uint(self.opcode_budget),
            CallerApplicationID => TealValue::Uint(self.caller_app_id.unwrap_or(0)),
            CallerApplicationAddress => {
                TealValue::Bytes(self.caller_app_addr.clone().unwrap_or_default())
            }
            GenesisHash => TealValue::Bytes(self.genesis_hash.clone()),
            _ => TealValue::Uint(0), // Default for other fields
        };

        Ok(value)
    }

    fn current_transaction(&self) -> AvmResult<Transaction> {
        self.transactions
            .get(self.current_txn_index)
            .cloned()
            .ok_or_else(|| crate::error::AvmError::InvalidTransactionField {
                field: format!("current transaction index {}", self.current_txn_index),
            })
    }

    fn transaction_group(&self) -> AvmResult<Vec<Transaction>> {
        Ok(self.transactions.clone())
    }

    fn program_args(&self) -> AvmResult<Vec<Vec<u8>>> {
        Ok(self.program_args.clone())
    }
}
