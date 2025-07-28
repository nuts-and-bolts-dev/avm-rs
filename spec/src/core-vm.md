# Virtual Machine Model

This chapter provides the formal specification of the Algorand Virtual Machine's core execution model, including the abstract machine definition, state transitions, and fundamental execution semantics.

## Abstract Machine Definition

The AVM is formally defined as a 6-tuple abstract machine:

```
AVM = ⟨S, P, pc, C, σ, L⟩
```

Where:
- `S`: Evaluation stack (bounded by MAX_STACK_SIZE = 1000)
- `P`: Program bytecode (immutable sequence of bytes)  
- `pc`: Program counter (natural number indexing into P)
- `C`: Cost accumulator (tracks computational resources)
- `σ`: Scratch space (256 slots of StackValue)
- `L`: Ledger access interface (read-only view of blockchain state)

### Value Domain

The AVM operates on a domain of stack values:

```
StackValue ::= Uint(n) | Bytes(b)
  where n ∈ ℕ ∩ [0, 2⁶⁴-1]
        b ∈ Byte*
```

### Machine State

The complete machine state is represented as:

```
State = ⟨S, P, pc, C, σ, L, mode, version, budget⟩

where:
  S ∈ StackValue*           (|S| ≤ 1000)
  P ∈ Byte*                 (immutable)
  pc ∈ ℕ                    (0 ≤ pc ≤ |P|)
  C ∈ ℕ                     (accumulated cost)
  σ: [0,255] → StackValue   (scratch space)
  L: LedgerState            (blockchain state)
  mode ∈ {Signature, Application}
  version ∈ [1,11]          (TEAL version)
  budget ∈ ℕ⁺               (cost budget)
```

## Execution Semantics

### Small-Step Operational Semantics

The AVM execution is defined by a transition relation `→`:

```
State → State ∪ {⊥, ✓}
```

Where:
- `⊥` represents an error state (execution failure)
- `✓` represents successful termination
- Regular states continue execution

### Transition Rules

#### Program Counter Bounds Check
```
pc ≥ |P|
―――――――――――――――――――――――  [PC-BOUNDS]
⟨S, P, pc, C, σ, L⟩ → ⊥
```

#### Cost Budget Enforcement
```
C + cost(P[pc]) > budget
――――――――――――――――――――――――――  [COST-EXCEEDED]
⟨S, P, pc, C, σ, L⟩ → ⊥
```

#### Opcode Execution
```
P[pc] = opcode
opcode ∈ ValidOpcodes(version, mode)
exec(opcode, ⟨S, P, pc, C, σ, L⟩) = ⟨S', P, pc', C', σ', L'⟩
―――――――――――――――――――――――――――――――――――――――――――――――――――――――  [EXEC-OP]
⟨S, P, pc, C, σ, L⟩ → ⟨S', P, pc', C', σ', L'⟩
```

#### Program Termination
```
pc = |P|  ∧  |S| = 1  ∧  head(S) ≠ Uint(0)
――――――――――――――――――――――――――――――――――――――――  [TERMINATE-SUCCESS]
⟨S, P, pc, C, σ, L⟩ → ✓
```

```
pc = |P|  ∧  |S| = 1  ∧  head(S) = Uint(0)
――――――――――――――――――――――――――――――――――――――――  [TERMINATE-FAILURE]
⟨S, P, pc, C, σ, L⟩ → ⊥
```

```
pc = |P|  ∧  |S| ≠ 1
―――――――――――――――――――――――  [TERMINATE-ERROR]
⟨S, P, pc, C, σ, L⟩ → ⊥
```

## Stack Operations

### Stack Constraints

The evaluation stack must satisfy:

1. **Size Bounds**: `|S| ≤ MAX_STACK_SIZE`
2. **Non-negative Size**: `|S| ≥ 0`
3. **Termination Condition**: `|S| = 1` at program end

### Stack Manipulation Primitives

#### Push Operation
```
push: StackValue × Stack → Stack ∪ {⊥}

push(v, S) = {
  v :: S     if |S| < MAX_STACK_SIZE
  ⊥          otherwise
}
```

#### Pop Operation
```
pop: Stack → (StackValue × Stack) ∪ {⊥}

pop(S) = {
  (head(S), tail(S))  if |S| > 0
  ⊥                   otherwise
}
```

#### Peek Operation
```
peek: Stack × ℕ → StackValue ∪ {⊥}

peek(S, depth) = {
  S[depth]  if depth < |S|
  ⊥         otherwise
}
```

## Program Counter Management

### Advancement Rules

#### Sequential Advancement
```
advance_pc: State × ℕ → State ∪ {⊥}

advance_pc(⟨S, P, pc, C, σ, L⟩, offset) = {
  ⟨S, P, pc + offset, C, σ, L⟩  if pc + offset ≤ |P|
  ⊥                              otherwise
}
```

#### Branch Operations
```
branch: State × ℤ → State ∪ {⊥}

branch(⟨S, P, pc, C, σ, L⟩, target) = {
  ⟨S, P, pc + target, C, σ, L⟩  if 0 ≤ pc + target ≤ |P|
  ⊥                              otherwise
}
```

## Cost Accounting

### Cost Model

Each opcode has an associated cost:

```
cost: Opcode → ℕ⁺

Examples:
cost(push) = 1
cost(sha256) = 35
cost(ed25519verify) = 1900
```

### Cost Accumulation

```
add_cost: State × ℕ → State ∪ {⊥}

add_cost(⟨S, P, pc, C, σ, L⟩, δ) = {
  ⟨S, P, pc, C + δ, σ, L⟩  if C + δ ≤ budget
  ⊥                        otherwise
}
```

## Scratch Space

The scratch space provides 256 slots of temporary storage:

```
σ: [0, 255] → StackValue

Initial state: ∀i ∈ [0, 255]. σ(i) = Uint(0)
```

### Scratch Operations

#### Load from Scratch
```
load_scratch: State × Byte → (StackValue × State) ∪ {⊥}

load_scratch(⟨S, P, pc, C, σ, L⟩, index) = {
  (σ(index), ⟨S, P, pc, C, σ, L⟩)  if index ≤ 255
  ⊥                                  otherwise
}
```

#### Store to Scratch
```
store_scratch: State × Byte × StackValue → State ∪ {⊥}

store_scratch(⟨S, P, pc, C, σ, L⟩, index, value) = {
  ⟨S, P, pc, C, σ[index ↦ value], L⟩  if index ≤ 255
  ⊥                                     otherwise
}
```

## Call Stack (Subroutines)

For TEAL versions ≥ 4, the AVM supports subroutines with a call stack:

```
CallStack = pc*  where |CallStack| ≤ MAX_CALL_DEPTH = 8
```

### Call Stack Operations

#### Subroutine Call
```
call_sub: State × ℕ → State ∪ {⊥}

call_sub(⟨S, P, pc, C, σ, L, cs⟩, target) = {
  ⟨S, P, target, C, σ, L, pc :: cs⟩  if |cs| < MAX_CALL_DEPTH ∧ target < |P|
  ⊥                                   otherwise
}
```

#### Subroutine Return
```
return_sub: State → State ∪ {⊥}

return_sub(⟨S, P, pc, C, σ, L, cs⟩) = {
  ⟨S, P, head(cs), C, σ, L, tail(cs)⟩  if |cs| > 0
  ⊥                                      otherwise
}
```

## Execution Modes

### Signature Mode (Stateless)

- **Purpose**: Transaction signature verification
- **State Access**: Read-only access to current transaction
- **Cost Budget**: 700 (default)
- **Restrictions**: No state modification operations

### Application Mode (Stateful)

- **Purpose**: Smart contract execution  
- **State Access**: Read/write access to global and local state
- **Cost Budget**: 20,000 (default)
- **Capabilities**: Full opcode set including state operations

## Version Compatibility

The AVM maintains backward compatibility across TEAL versions:

```
ValidOpcodes: Version × Mode → P(Opcode)

ValidOpcodes(v, m) = {op ∈ Opcodes | min_version(op) ≤ v ∧ m ∈ modes(op)}
```

### Version-Specific Features

| Version | New Features |
|---------|-------------|
| V1 | Basic arithmetic, stack operations, crypto |
| V2 | Branch operations, more arithmetic |
| V3 | Asset operations |
| V4 | Subroutines (callsub/retsub) |
| V5 | Inner transactions, application calls |
| V6 | Extended inner transaction support |
| V7 | VRF verification |
| V8 | Box storage operations |
| V9 | Extended box operations |
| V10 | Elliptic curve operations |
| V11 | MIMC hash, block randomness |

## Invariants

The following invariants must hold throughout execution:

1. **Stack Bound**: `|S| ≤ MAX_STACK_SIZE`
2. **Program Counter Bound**: `0 ≤ pc ≤ |P|`
3. **Cost Bound**: `C ≤ budget`
4. **Scratch Bound**: `∀i. 0 ≤ i ≤ 255 ⇒ σ(i) is defined`
5. **Call Stack Bound**: `|CallStack| ≤ MAX_CALL_DEPTH`

## Determinism

The AVM guarantees deterministic execution:

**Theorem (Determinism)**: For any valid program P and initial state S₀, the execution sequence is unique up to termination or error.

**Proof Sketch**: Each transition rule is deterministic - given a state and opcode, there is at most one valid successor state. Non-deterministic operations (like randomness) are handled through deterministic ledger state access.

This determinism is crucial for blockchain consensus, ensuring all nodes reach the same execution result for any given program and input state.