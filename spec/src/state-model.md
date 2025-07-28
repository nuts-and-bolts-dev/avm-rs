# State Model

This chapter provides a formal specification of the AVM state model, including global and local state semantics, transaction group execution, and ledger interaction patterns. The state model defines how smart contracts interact with persistent blockchain state.

## State Architecture Overview

The AVM operates with multiple layers of state:

```
StateUniverse = GlobalState × LocalState × TransactionState × LedgerState
```

Where:
- `GlobalState`: Application-wide key-value storage
- `LocalState`: Per-account application state  
- `TransactionState`: Temporary state within transaction execution
- `LedgerState`: Read-only blockchain state (accounts, assets, etc.)

## Global State

### Global State Definition

Global state provides application-wide persistent storage:

```
GlobalState = AppID → (Key → Value)

where:
  AppID ∈ ℕ⁺ 
  Key ∈ bytes  (|Key| ≤ 64)
  Value ∈ TealValue = uint64 + bytes
```

### Global State Operations

#### Global State Read

```
get_global : AppID × Key → (Value × bool)

get_global(app_id, key) = 
  if key ∈ GlobalState[app_id] then
    (GlobalState[app_id][key], true)
  else
    (Uint(0), false)
```

#### Global State Write

```
put_global : AppID × Key × Value → GlobalState'

put_global(app_id, key, value) = 
  GlobalState[app_id ↦ GlobalState[app_id][key ↦ value]]
```

#### Global State Delete

```
del_global : AppID × Key → GlobalState'

del_global(app_id, key) = 
  GlobalState[app_id ↦ GlobalState[app_id] \ {key}]
```

### Global State Constraints

1. **Key Limit**: Each application can store up to `GlobalNumByteSlice + GlobalNumUint` keys
2. **Key Size**: `|key| ≤ 64` bytes
3. **Value Size**: `|value| ≤ 128` bytes for byte values
4. **Schema Enforcement**: Keys must conform to declared schema

## Local State

### Local State Definition

Local state provides per-account application storage:

```
LocalState = (Account × AppID) → (Key → Value)

where:
  Account ∈ Address = bytes₃₂
  AppID ∈ ℕ⁺
  Key ∈ bytes  (|Key| ≤ 64)
  Value ∈ TealValue
```

### Opt-In Requirement

Before accessing local state, accounts must opt into the application:

```
OptedIn ⊆ Account × AppID

opted_in(account, app_id) ⟺ (account, app_id) ∈ OptedIn
```

### Local State Operations

#### Local State Read

```
get_local : Account × AppID × Key → (Value × bool)

get_local(account, app_id, key) = 
  if ¬opted_in(account, app_id) then
    (Uint(0), false)
  else if key ∈ LocalState[(account, app_id)] then
    (LocalState[(account, app_id)][key], true)
  else
    (Uint(0), false)
```

#### Local State Write

```
put_local : Account × AppID × Key × Value → LocalState'

put_local(account, app_id, key, value) = 
  require opted_in(account, app_id)
  LocalState[(account, app_id) ↦ LocalState[(account, app_id)][key ↦ value]]
```

#### Local State Delete

```
del_local : Account × AppID × Key → LocalState'

del_local(account, app_id, key) = 
  require opted_in(account, app_id)  
  LocalState[(account, app_id) ↦ LocalState[(account, app_id)] \ {key}]
```

### Local State Constraints

1. **Opt-In Required**: Account must be opted into application
2. **Key Limit**: Each account can store up to `LocalNumByteSlice + LocalNumUint` keys per application
3. **Key Size**: `|key| ≤ 64` bytes  
4. **Value Size**: `|value| ≤ 128` bytes for byte values

## Box Storage (TEAL v8+)

### Box Storage Definition

Box storage provides large-scale key-value storage for applications:

```
BoxStorage = AppID → (BoxName → BoxContent)

where:
  BoxName ∈ bytes  (|BoxName| ≤ 64)
  BoxContent ∈ bytes  (|BoxContent| ≤ 32768)
```

### Box Operations

#### Box Creation

```
box_create : AppID × BoxName × Size → (BoxStorage' × bool)

box_create(app_id, name, size) = 
  if name ∈ BoxStorage[app_id] then
    (BoxStorage, false)  // Already exists
  else
    require size ≤ MAX_BOX_SIZE
    (BoxStorage[app_id ↦ BoxStorage[app_id][name ↦ zeros(size)]], true)
```

#### Box Access

```
box_get : AppID × BoxName → (BoxContent × bool)

box_get(app_id, name) = 
  if name ∈ BoxStorage[app_id] then
    (BoxStorage[app_id][name], true)
  else
    (∅, false)
```

#### Box Modification

```
box_put : AppID × BoxName × BoxContent → BoxStorage'

box_put(app_id, name, content) = 
  require name ∈ BoxStorage[app_id]
  require |content| = |BoxStorage[app_id][name]|  // Size must match
  BoxStorage[app_id ↦ BoxStorage[app_id][name ↦ content]]
```

### Box Storage Constraints

1. **Box Limit**: Applications have a maximum box storage budget
2. **Name Size**: `|name| ≤ 64` bytes
3. **Content Size**: `|content| ≤ 32768` bytes per box
4. **Storage Cost**: Box storage incurs minimum balance requirements

## Transaction State

### Transaction Context

Each program execution occurs within a transaction context:

```
TransactionContext = {
  sender: Address,
  fee: uint64,
  first_valid: uint64,
  last_valid: uint64,
  note: bytes,
  lease: bytes₃₂,
  rekey_to: Address,
  tx_type: TransactionType,
  ...
}
```

### Transaction Group State

Transactions can be grouped for atomic execution:

```
TransactionGroup = [Transaction₁, Transaction₂, ..., Transactionₙ]

where n ≤ MAX_GROUP_SIZE = 16
```

#### Group Properties

1. **Atomicity**: All transactions in group succeed or all fail
2. **Ordering**: Transactions execute in group order
3. **Isolation**: Groups execute atomically with respect to other groups

### Inner Transaction State

Applications can create inner transactions:

```
InnerTransaction = {
  type: TransactionType,
  sender: Address,
  fields: FieldMap,
  ...
}

InnerTransactionGroup = [InnerTransaction₁, ..., InnerTransactionₘ]
```

## Ledger State

### Ledger State Definition

The ledger provides read-only access to blockchain state:

```
LedgerState = {
  accounts: Account → AccountData,
  assets: AssetID → AssetData,  
  applications: AppID → AppData,
  blocks: Round → BlockData,
  global_params: GlobalParams,
}
```

### Account Data

```
AccountData = {
  balance: uint64,
  min_balance: uint64,
  auth_addr: Address,
  assets: AssetID → AssetHolding,
  apps_local_state: AppID → LocalState,
  apps_total_schema: Schema,
  created_assets: Set<AssetID>,
  created_apps: Set<AppID>,
}
```

### Asset Data

```
AssetData = {
  total: uint64,
  decimals: uint64,
  default_frozen: bool,
  unit_name: bytes,
  name: bytes,
  url: bytes,
  metadata_hash: bytes₃₂,
  manager: Address,
  reserve: Address,
  freeze: Address,
  clawback: Address,
}
```

### Application Data

```
AppData = {
  approval_program: bytes,
  clear_state_program: bytes,
  global_state: GlobalState,
  global_schema: Schema,
  local_schema: Schema,
  extra_program_pages: uint64,
}
```

## State Transition Semantics

### Transaction Execution Model

```
execute_transaction : LedgerState × Transaction → LedgerState' × ExecutionResult

execute_transaction(L, txn) = 
  match txn.type with
  | Payment → execute_payment(L, txn)
  | ApplicationCall → execute_app_call(L, txn)
  | AssetTransfer → execute_asset_transfer(L, txn)
  | ...
```

### Application Call Execution

```
execute_app_call : LedgerState × ApplicationCallTxn → LedgerState' × ExecutionResult

execute_app_call(L, txn) = 
  let program = L.applications[txn.app_id].approval_program
  let initial_state = create_execution_state(L, txn)
  let result = execute_program(program, initial_state)
  (apply_state_changes(L, result.state_delta), result.success)
```

### State Delta Application

State changes are accumulated during execution and applied atomically:

```
StateDelta = {
  global_deltas: Map<AppID, Map<Key, ValueDelta>>,
  local_deltas: Map<(Account, AppID), Map<Key, ValueDelta>>,
  box_deltas: Map<AppID, Map<BoxName, BoxDelta>>,
}

ValueDelta = SetValue(Value) | DeleteKey
BoxDelta = CreateBox(Size) | DeleteBox | SetContent(bytes)
```

## State Access Control

### Permission Model

State access follows a strict permission model:

```
CanRead : ExecutionContext × StateLocation → bool
CanWrite : ExecutionContext × StateLocation → bool

StateLocation = 
  | GlobalState(AppID, Key)
  | LocalState(Account, AppID, Key)  
  | BoxStorage(AppID, BoxName)
  | LedgerState(Path)
```

### Access Rules

1. **Global State**: 
   - Read: Any program
   - Write: Only the owning application

2. **Local State**:
   - Read: Any program (if account opted in)
   - Write: Only with account authorization

3. **Box Storage**:
   - Read/Write: Only the owning application

4. **Ledger State**:
   - Read: Any program (subject to availability)
   - Write: Never (read-only)

## State Consistency Properties

### ACID Properties

The state model ensures ACID properties:

1. **Atomicity**: All state changes within a transaction group succeed or fail together
2. **Consistency**: State changes preserve all invariants and constraints  
3. **Isolation**: Concurrent transaction groups don't interfere
4. **Durability**: Committed state changes are permanently stored

### State Invariants

1. **Balance Conservation**: Total balance is preserved across transfers
2. **Schema Compliance**: State keys/values conform to declared schemas
3. **Opt-In Requirement**: Local state access requires opt-in
4. **Authorization**: State modifications require proper authorization

## State Verification

### State Proof System

State changes can be verified using cryptographic proofs:

```
StateProof = {
  root_hash: bytes₃₂,
  proof_path: ProofPath,
  state_delta: StateDelta,
}

verify_state_proof : StateProof × bytes₃₂ → bool
```

### Merkle Tree Structure

State is organized in Merkle trees for efficient verification:

```
StateTree = Leaf(Value) | Branch(StateTree, StateTree, Hash)

compute_root : StateTree → bytes₃₂
generate_proof : StateTree × Key → ProofPath
```

## Implementation Considerations

### State Caching

Efficient state access requires caching strategies:

```rust
struct StateCache {
    global_cache: HashMap<(AppID, Key), Option<TealValue>>,
    local_cache: HashMap<(Account, AppID, Key), Option<TealValue>>,
    box_cache: HashMap<(AppID, BoxName), Option<Vec<u8>>>,
}
```

### State Batching

State changes are batched for efficiency:

```rust
struct StateBatch {
    changes: Vec<StateChange>,
    reads: Vec<StateRead>,
    creates: Vec<StateCreate>,
    deletes: Vec<StateDelete>,
}
```

### Conflict Detection

Concurrent access requires conflict detection:

```rust
fn detect_conflicts(batch1: &StateBatch, batch2: &StateBatch) -> bool {
    // Check for overlapping write sets
    !batch1.write_set().is_disjoint(&batch2.write_set())
}
```

## Advanced State Features

### State Schemas

Applications declare state schemas:

```
Schema = {
  num_uints: uint64,
  num_byte_slices: uint64,
}

validate_schema : StateChange × Schema → bool
```

### State Migrations

Applications can migrate state between versions:

```
migrate_state : OldState × MigrationSpec → NewState
```

### State Archival

Historical state can be archived:

```
ArchivalPolicy = {
  retention_period: uint64,
  compression: CompressionType,
  verification: bool,
}
```

## State Model Correctness

### Determinism

**Theorem (State Determinism)**: Given identical initial state and transaction sequence, all nodes reach the same final state.

### Consistency

**Theorem (State Consistency)**: All state invariants are preserved across valid state transitions.

### Availability

**Theorem (State Availability)**: State operations complete within bounded time assuming bounded transaction complexity.

This state model provides the foundation for secure, efficient, and verifiable state management in the Algorand Virtual Machine, enabling complex smart contract applications while maintaining blockchain security and performance requirements.