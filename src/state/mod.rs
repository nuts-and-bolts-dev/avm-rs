//! State management interfaces and implementations

use crate::error::AvmResult;
use crate::types::TealValue;
use std::collections::HashMap;

/// Account address type
pub type Address = Vec<u8>;

/// Application ID type
pub type AppId = u64;

/// Asset ID type
pub type AssetId = u64;

/// Micro Algos type
pub type MicroAlgos = u64;

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
}

/// Mock ledger implementation for testing
#[derive(Debug, Default)]
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
}
