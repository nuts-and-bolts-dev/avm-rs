# Opcode Semantics

This chapter provides comprehensive formal semantics for all TEAL opcodes. Each opcode is specified using operational semantics with precise preconditions, state transformations, and error conditions.

## Notation and Conventions

### Operational Semantics Format

For each opcode, we provide:

1. **Syntax**: Bytecode representation and assembly syntax
2. **Typing**: Input/output type constraints
3. **Semantics**: State transformation rules
4. **Preconditions**: Requirements for valid execution
5. **Postconditions**: Guaranteed state properties
6. **Errors**: Conditions that cause execution failure

### Type Judgments

We use typing judgments of the form:
```
S ⊢ op : τ₁ × ... × τₙ → τ₁' × ... × τₘ'
```

This reads as "under stack S, operation op consumes n values of types τ₁...τₙ and produces m values of types τ₁'...τₘ'".

### State Transformation Notation

```
⟨S, P, pc, C, σ, L⟩ →[op] ⟨S', P, pc', C', σ', L'⟩
```

## Arithmetic Operations

### Addition (`+`, opcode 0x08)

**Syntax**: `+`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
a, b ∈ uint64
result = a + b (mod 2⁶⁴)
―――――――――――――――――――――――――――――――――――――――――  [ADD]
⟨S, P, pc, C, σ, L⟩ →[+] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

**Error Conditions**:
```
|S| < 2
―――――――――――――――――――――  [ADD-UNDERFLOW]
⟨S, P, pc, C, σ, L⟩ →[+] ⊥
```

### Subtraction (`-`, opcode 0x09)

**Syntax**: `-`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
a, b ∈ uint64
a ≥ b
result = a - b
―――――――――――――――――――――――――――――――――――――――――  [SUB]
⟨S, P, pc, C, σ, L⟩ →[-] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

**Error Conditions**:
```
S = b :: a :: S'  ∧  a < b
―――――――――――――――――――――――――――  [SUB-UNDERFLOW]
⟨S, P, pc, C, σ, L⟩ →[-] ⊥
```

### Division (`/`, opcode 0x0A)

**Syntax**: `/`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
a, b ∈ uint64
b ≠ 0
result = ⌊a / b⌋
―――――――――――――――――――――――――――――――――――――――――  [DIV]
⟨S, P, pc, C, σ, L⟩ →[/] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

**Error Conditions**:
```
S = 0 :: a :: S'
―――――――――――――――――――――――――  [DIV-BY-ZERO]
⟨S, P, pc, C, σ, L⟩ →[/] ⊥
```

### Multiplication (`*`, opcode 0x0B)

**Syntax**: `*`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
a, b ∈ uint64
result = (a × b) mod 2⁶⁴
―――――――――――――――――――――――――――――――――――――――――  [MUL]
⟨S, P, pc, C, σ, L⟩ →[*] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

### Modulo (`%`, opcode 0x0C)

**Syntax**: `%`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
a, b ∈ uint64
b ≠ 0
result = a mod b
―――――――――――――――――――――――――――――――――――――――――  [MOD]
⟨S, P, pc, C, σ, L⟩ →[%] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

## Logical Operations

### Logical AND (`&&`, opcode 0x10)

**Syntax**: `&&`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
result = if (a ≠ 0) ∧ (b ≠ 0) then 1 else 0
―――――――――――――――――――――――――――――――――――――――――――――――――  [AND]
⟨S, P, pc, C, σ, L⟩ →[&&] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

### Logical OR (`||`, opcode 0x11)

**Syntax**: `||`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
result = if (a ≠ 0) ∨ (b ≠ 0) then 1 else 0
―――――――――――――――――――――――――――――――――――――――――――――――――  [OR]
⟨S, P, pc, C, σ, L⟩ →[||] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

### Logical NOT (`!`, opcode 0x12)

**Syntax**: `!`

**Type**: `uint64 → uint64`

**Semantics**:
```
S = a :: S'
result = if a = 0 then 1 else 0
―――――――――――――――――――――――――――――――――――――  [NOT]
⟨S, P, pc, C, σ, L⟩ →[!] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

## Comparison Operations

### Equality (`==`, opcode 0x12)

**Syntax**: `==`

**Type**: `any × any → uint64`

**Semantics**:
```
S = b :: a :: S'
result = if a = b then 1 else 0
―――――――――――――――――――――――――――――――――――――――  [EQ]
⟨S, P, pc, C, σ, L⟩ →[==] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

**Note**: Equality is defined for both uint64 and bytes values. For bytes, it's lexicographic equality.

### Less Than (`<`, opcode 0x0C)

**Syntax**: `<`

**Type**: `uint64 × uint64 → uint64`

**Semantics**:
```
S = b :: a :: S'
a, b ∈ uint64
result = if a < b then 1 else 0
―――――――――――――――――――――――――――――――――――――――  [LT]
⟨S, P, pc, C, σ, L⟩ →[<] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

## Stack Operations

### Duplicate (`dup`, opcode 0x48)

**Syntax**: `dup`

**Type**: `any → any × any`

**Semantics**:
```
S = a :: S'
|S'| < MAX_STACK_SIZE - 1
―――――――――――――――――――――――――――――――――――――――――  [DUP]
⟨S, P, pc, C, σ, L⟩ →[dup] ⟨a :: a :: S', P, pc+1, C+1, σ, L⟩
```

### Pop (`pop`, opcode 0x48)

**Syntax**: `pop`

**Type**: `any → ε`

**Semantics**:
```
S = a :: S'
―――――――――――――――――――――――――――――――――――――――  [POP]
⟨S, P, pc, C, σ, L⟩ →[pop] ⟨S', P, pc+1, C+1, σ, L⟩
```

### Swap (`swap`, opcode 0x4C)

**Syntax**: `swap`

**Type**: `any × any → any × any`

**Semantics**:
```
S = b :: a :: S'
―――――――――――――――――――――――――――――――――――――――――  [SWAP]
⟨S, P, pc, C, σ, L⟩ →[swap] ⟨a :: b :: S', P, pc+1, C+1, σ, L⟩
```

## Flow Control Operations

### Branch if Not Zero (`bnz`, opcode 0x40)

**Syntax**: `bnz target`

**Type**: `uint64 → ε`

**Semantics**:
```
S = a :: S'
P[pc+1] = target_low
P[pc+2] = target_high  
target = (target_high << 8) | target_low
new_pc = if a ≠ 0 then pc + 3 + target else pc + 3
0 ≤ new_pc ≤ |P|
―――――――――――――――――――――――――――――――――――――――――――――――――――  [BNZ]
⟨S, P, pc, C, σ, L⟩ →[bnz] ⟨S', P, new_pc, C+1, σ, L⟩
```

### Branch if Zero (`bz`, opcode 0x41)

**Syntax**: `bz target`

**Type**: `uint64 → ε`

**Semantics**:
```
S = a :: S'
P[pc+1] = target_low
P[pc+2] = target_high
target = (target_high << 8) | target_low
new_pc = if a = 0 then pc + 3 + target else pc + 3
0 ≤ new_pc ≤ |P|
―――――――――――――――――――――――――――――――――――――――――――――――――――  [BZ]
⟨S, P, pc, C, σ, L⟩ →[bz] ⟨S', P, new_pc, C+1, σ, L⟩
```

### Unconditional Branch (`b`, opcode 0x42)

**Syntax**: `b target`

**Type**: `ε → ε`

**Semantics**:
```
P[pc+1] = target_low
P[pc+2] = target_high
target = (target_high << 8) | target_low
new_pc = pc + 3 + target
0 ≤ new_pc ≤ |P|
―――――――――――――――――――――――――――――――――――――――――――――――――――  [B]
⟨S, P, pc, C, σ, L⟩ →[b] ⟨S, P, new_pc, C+1, σ, L⟩
```

### Return (`return`, opcode 0x43)

**Syntax**: `return`

**Type**: `uint64 → halt`

**Semantics**:
```
S = a :: ∅
result = if a ≠ 0 then ✓ else ⊥
―――――――――――――――――――――――――――――――――――――――――  [RETURN]
⟨S, P, pc, C, σ, L⟩ →[return] result
```

## Constant Operations

### Push Integer (`pushint`, opcode 0x81)

**Syntax**: `pushint VALUE`

**Type**: `ε → uint64`

**Semantics**:
```
P[pc+1..pc+8] = value_bytes  (8-byte big-endian)
value = decode_uint64(value_bytes)
|S| < MAX_STACK_SIZE
―――――――――――――――――――――――――――――――――――――――――――――――――  [PUSHINT]
⟨S, P, pc, C, σ, L⟩ →[pushint] ⟨value :: S, P, pc+9, C+1, σ, L⟩
```

### Push Bytes (`pushbytes`, opcode 0x80)

**Syntax**: `pushbytes "VALUE"`

**Type**: `ε → bytes`

**Semantics**:
```
P[pc+1] = length
P[pc+2..pc+1+length] = value_bytes
|S| < MAX_STACK_SIZE
―――――――――――――――――――――――――――――――――――――――――――――――――――――――  [PUSHBYTES]
⟨S, P, pc, C, σ, L⟩ →[pushbytes] ⟨value_bytes :: S, P, pc+2+length, C+1, σ, L⟩
```

## Cryptographic Operations

### SHA256 (`sha256`, opcode 0x01)

**Syntax**: `sha256`

**Type**: `bytes → bytes`

**Semantics**:
```
S = data :: S'
result = SHA256(data)
|result| = 32
―――――――――――――――――――――――――――――――――――――――――――  [SHA256]
⟨S, P, pc, C, σ, L⟩ →[sha256] ⟨result :: S', P, pc+1, C+35, σ, L⟩
```

### Keccak256 (`keccak256`, opcode 0x02)

**Syntax**: `keccak256`

**Type**: `bytes → bytes`

**Semantics**:
```
S = data :: S'
result = KECCAK256(data)
|result| = 32
―――――――――――――――――――――――――――――――――――――――――――――  [KECCAK256]
⟨S, P, pc, C, σ, L⟩ →[keccak256] ⟨result :: S', P, pc+1, C+130, σ, L⟩
```

### Ed25519 Verify (`ed25519verify`, opcode 0x04)

**Syntax**: `ed25519verify`

**Type**: `bytes × bytes × bytes → uint64`

**Semantics**:
```
S = pubkey :: signature :: data :: S'
|pubkey| = 32
|signature| = 64
result = if Ed25519_Verify(data, signature, pubkey) then 1 else 0
―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――  [ED25519VERIFY]
⟨S, P, pc, C, σ, L⟩ →[ed25519verify] ⟨result :: S', P, pc+1, C+1900, σ, L⟩
```

## Byte Operations

### Length (`len`, opcode 0x21)

**Syntax**: `len`

**Type**: `bytes → uint64`

**Semantics**:
```
S = data :: S'
result = |data|
―――――――――――――――――――――――――――――――――――――――――  [LEN]
⟨S, P, pc, C, σ, L⟩ →[len] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

### Substring (`substring`, opcode 0x51)

**Syntax**: `substring START END`

**Type**: `bytes → bytes`

**Semantics**:
```
S = data :: S'
P[pc+1] = start
P[pc+2] = end
0 ≤ start ≤ end ≤ |data|
result = data[start:end]
―――――――――――――――――――――――――――――――――――――――――――――――  [SUBSTRING]
⟨S, P, pc, C, σ, L⟩ →[substring] ⟨result :: S', P, pc+3, C+1, σ, L⟩
```

### Concatenate (`concat`, opcode 0x50)

**Syntax**: `concat`

**Type**: `bytes × bytes → bytes`

**Semantics**:
```
S = b :: a :: S'
result = a || b  (concatenation)
|result| ≤ MAX_BYTE_ARRAY_LENGTH
―――――――――――――――――――――――――――――――――――――――――――――――  [CONCAT]
⟨S, P, pc, C, σ, L⟩ →[concat] ⟨result :: S', P, pc+1, C+1, σ, L⟩
```

## Scratch Space Operations

### Load (`load`, opcode 0x34)

**Syntax**: `load INDEX`

**Type**: `ε → any`

**Semantics**:
```
P[pc+1] = index
index ≤ 255
value = σ(index)
|S| < MAX_STACK_SIZE
―――――――――――――――――――――――――――――――――――――――――――――  [LOAD]
⟨S, P, pc, C, σ, L⟩ →[load] ⟨value :: S, P, pc+2, C+1, σ, L⟩
```

### Store (`store`, opcode 0x35)

**Syntax**: `store INDEX`

**Type**: `any → ε`

**Semantics**:
```
S = value :: S'
P[pc+1] = index
index ≤ 255
σ' = σ[index ↦ value]
―――――――――――――――――――――――――――――――――――――――――――――――  [STORE]
⟨S, P, pc, C, σ, L⟩ →[store] ⟨S', P, pc+2, C+1, σ', L⟩
```

## Subroutine Operations (TEAL v4+)

### Call Subroutine (`callsub`, opcode 0x88)

**Syntax**: `callsub TARGET`

**Type**: `ε → ε`

**Semantics**:
```
P[pc+1] = target_low
P[pc+2] = target_high
target = (target_high << 8) | target_low
|call_stack| < MAX_CALL_DEPTH
target < |P|
―――――――――――――――――――――――――――――――――――――――――――――――――――――――  [CALLSUB]
⟨S, P, pc, C, σ, L, cs⟩ →[callsub] ⟨S, P, target, C+1, σ, L, (pc+3) :: cs⟩
```

### Return from Subroutine (`retsub`, opcode 0x89)

**Syntax**: `retsub`

**Type**: `ε → ε`

**Semantics**:
```
call_stack = return_pc :: cs'
―――――――――――――――――――――――――――――――――――――――――――――――――――――  [RETSUB]
⟨S, P, pc, C, σ, L, call_stack⟩ →[retsub] ⟨S, P, return_pc, C+1, σ, L, cs'⟩
```

## Error Handling

### Error (`err`, opcode 0x00)

**Syntax**: `err`

**Type**: `ε → halt`

**Semantics**:
```
―――――――――――――――――――――――――――  [ERR]
⟨S, P, pc, C, σ, L⟩ →[err] ⊥
```

### Assert (`assert`, opcode 0x44)

**Syntax**: `assert`

**Type**: `uint64 → ε`

**Semantics**:
```
S = a :: S'
a ≠ 0
―――――――――――――――――――――――――――――――――――――――――  [ASSERT-SUCCESS]
⟨S, P, pc, C, σ, L⟩ →[assert] ⟨S', P, pc+1, C+1, σ, L⟩
```

```
S = 0 :: S'
―――――――――――――――――――――――――――  [ASSERT-FAIL]
⟨S, P, pc, C, σ, L⟩ →[assert] ⊥
```

## Application State Operations (Application Mode Only)

### Global State Get (`app_global_get`, opcode 0x60)

**Syntax**: `app_global_get`

**Type**: `bytes → any × uint64`

**Semantics**:
```
S = key :: S'
mode = Application
(value, exists) = L.get_global_state(current_app_id, key)
result_value = if exists then value else Uint(0)
result_flag = if exists then 1 else 0
|S'| + 1 < MAX_STACK_SIZE
―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――  [APP-GLOBAL-GET]
⟨S, P, pc, C, σ, L⟩ →[app_global_get] ⟨result_flag :: result_value :: S', P, pc+1, C+1, σ, L⟩
```

### Global State Put (`app_global_put`, opcode 0x61)

**Syntax**: `app_global_put`

**Type**: `bytes × any → ε`

**Semantics**:
```
S = value :: key :: S'
mode = Application
L' = L.set_global_state(current_app_id, key, value)
―――――――――――――――――――――――――――――――――――――――――――――――――――――――  [APP-GLOBAL-PUT]
⟨S, P, pc, C, σ, L⟩ →[app_global_put] ⟨S', P, pc+1, C+1, σ, L'⟩
```

## Opcode Classification

### By Execution Mode

| Mode | Opcodes |
|------|---------|
| Both | Arithmetic, Logic, Stack, Flow Control, Constants, Crypto, Bytes |
| Application Only | State operations, Inner transactions, Box storage |

### By TEAL Version

| Version | New Opcodes |
|---------|-------------|
| v1 | Basic arithmetic, logic, crypto, stack |
| v2 | Branch operations, more arithmetic |
| v3 | Asset operations |
| v4 | Subroutines (callsub, retsub) |
| v5 | Inner transactions, advanced state |
| v6-v11 | Extended operations and optimizations |

### By Cost Category

| Cost Range | Operations |
|------------|------------|
| 1 | Basic arithmetic, logic, stack |
| 1-50 | Crypto hashes, byte operations |
| 100-2000 | Signature verification, complex crypto |
| 3000+ | Advanced elliptic curve operations |

This completes the formal semantic specification for TEAL opcodes, providing the mathematical foundation necessary for implementation verification and correctness proofs.