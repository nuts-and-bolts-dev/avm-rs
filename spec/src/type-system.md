# Type System

This chapter specifies the TEAL type system, including value types, type safety properties, and static analysis foundations. The type system ensures program correctness and prevents many classes of runtime errors.

## Type Universe

The TEAL type system is built around two fundamental value types with additional derived types for specific contexts.

### Base Types

```
τ ::= uint64 | bytes | ⊥
```

Where:
- `uint64`: 64-bit unsigned integers in range [0, 2⁶⁴-1]
- `bytes`: Variable-length byte arrays
- `⊥`: Error/undefined type (bottom type)

### Type Hierarchy

```
       Any
      /   \
   uint64  bytes
      \   /
        ⊥
```

The bottom type `⊥` is a subtype of all other types, representing error states or undefined values.

### Value Representation

```
Value ::= Uint(n) | Bytes(b) | Error
  where n ∈ ℕ ∩ [0, 2⁶⁴-1]
        b ∈ Byte*
```

## Typing Rules

### Typing Judgments

We use typing judgments of the form:
```
Γ ⊢ e : τ
```

This reads as "under type environment Γ, expression e has type τ".

For stack operations:
```
S ⊢ op : τ₁ × ... × τₙ → τ₁' × ... × τₘ'
```

### Type Environment

The type environment tracks the types of values in various contexts:

```
Γ ::= ∅ | Γ, x : τ
```

For the AVM, we primarily work with stack types:
```
StackType ::= [τ₁, τ₂, ..., τₙ]
```

Where τ₁ is the top of the stack.

## Type Checking Rules

### Arithmetic Operations

```
S ⊢ a : uint64    S ⊢ b : uint64
―――――――――――――――――――――――――――――――――  [T-ARITH]
S ⊢ op(a, b) : uint64

where op ∈ {+, -, *, /, %, &, |, ^}
```

### Logical Operations

```
S ⊢ a : uint64    S ⊢ b : uint64
―――――――――――――――――――――――――――――――――  [T-LOGICAL]
S ⊢ op(a, b) : uint64

where op ∈ {&&, ||, <, >, <=, >=, ==, !=}
```

### Byte Operations

```
S ⊢ a : bytes    S ⊢ b : bytes
――――――――――――――――――――――――――――――  [T-BYTE-OP]
S ⊢ concat(a, b) : bytes
```

```
S ⊢ data : bytes
――――――――――――――――――――  [T-LEN]
S ⊢ len(data) : uint64
```

### Stack Operations

```
S ⊢ a : τ
―――――――――――――――――――  [T-DUP]
S ⊢ dup(a) : τ × τ
```

```
S ⊢ a : τ
――――――――――――――  [T-POP]
S ⊢ pop(a) : ε
```

```
S ⊢ a : τ₁    S ⊢ b : τ₂
――――――――――――――――――――――――――  [T-SWAP]
S ⊢ swap(a, b) : τ₂ × τ₁
```

### Cryptographic Operations

```
S ⊢ data : bytes
―――――――――――――――――――――――  [T-HASH]
S ⊢ hash(data) : bytes

where hash ∈ {sha256, keccak256, sha512_256, sha3_256}
```

```
S ⊢ data : bytes    S ⊢ sig : bytes    S ⊢ pubkey : bytes
――――――――――――――――――――――――――――――――――――――――――――――――――――――――  [T-VERIFY]
S ⊢ ed25519verify(data, sig, pubkey) : uint64
```

## Type Coercion and Conversion

### Boolean Coercion

Any value can be coerced to boolean for conditional operations:

```
tobool : uint64 → bool
tobool(n) = n ≠ 0

tobool : bytes → bool  
tobool(b) = b ≠ ∅ ∧ ∃i. b[i] ≠ 0
```

### Type Conversion Operations

```
S ⊢ n : uint64
―――――――――――――――――――  [T-ITOB]
S ⊢ itob(n) : bytes
```

```
S ⊢ b : bytes    valid_uint(b)
――――――――――――――――――――――――――――――  [T-BTOI]
S ⊢ btoi(b) : uint64
```

Where `valid_uint(b)` ensures the byte array represents a valid unsigned integer.

## Type Safety Properties

### Type Preservation

**Theorem (Type Preservation)**: If `S ⊢ e : τ` and `e → e'`, then `S ⊢ e' : τ`.

**Proof Sketch**: By induction on the derivation of the typing judgment. Each operational rule preserves types according to the corresponding typing rule.

### Progress

**Theorem (Progress)**: If `∅ ⊢ e : τ`, then either:
1. `e` is a value, or  
2. There exists `e'` such that `e → e'`, or
3. `e → ⊥` (error state)

### Type Soundness

**Theorem (Type Soundness)**: If `∅ ⊢ e : τ` and `e →* v`, then `v` is a value of type `τ` or `v = ⊥`.

## Stack Type System

### Stack Type Checking

The stack type checker maintains a type stack that mirrors the value stack:

```
TypeStack = [τ₁, τ₂, ..., τₙ]
```

### Stack Type Transformations

Each opcode has a corresponding stack type transformation:

```
transform : Opcode → TypeStack → TypeStack ∪ {TypeError}
```

Examples:

```
transform(+, [uint64, uint64 | rest]) = [uint64 | rest]
transform(concat, [bytes, bytes | rest]) = [bytes | rest]  
transform(dup, [τ | rest]) = [τ, τ | rest]
```

### Type Checking Algorithm

```
typecheck : Program → Result<TypeStack, TypeError>

typecheck(program):
  stack = []
  for opcode in program:
    stack = transform(opcode, stack)
    if stack = TypeError:
      return TypeError
  return stack
```

## Static Analysis Foundations

### Reachability Analysis

Define reachable program points:

```
Reachable ⊆ ℕ
reach(P, 0) = {0} ∪ ⋃{reach(P, pc') | pc → pc' in P}
```

### Type Flow Analysis

Track type information flow through the program:

```
TypeFlow : ProgramPoint → TypeStack
```

### Dead Code Elimination

Identify unreachable code:

```
DeadCode = {pc ∈ [0, |P|) | pc ∉ Reachable}
```

## Version-Specific Type Rules

### TEAL v1-v3: Basic Types

- Only `uint64` and `bytes` supported
- Simple arithmetic and byte operations
- No complex control flow typing

### TEAL v4+: Subroutine Types

Subroutines introduce function types:

```
FunctionType = [τ₁, ..., τₙ] → [τ₁', ..., τₘ']
```

Type checking for subroutines:

```
S ⊢ sub : [τ₁, ..., τₙ] → [τ₁', ..., τₘ']    S ⊢ args : [τ₁, ..., τₙ]
――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――  [T-CALL]
S ⊢ callsub(sub, args) : [τ₁', ..., τₘ']
```

### TEAL v8+: Box Storage Types

Box operations introduce storage types:

```
BoxType = Name × Content
  where Name : bytes
        Content : bytes
```

Type rules for box operations:

```
S ⊢ name : bytes    S ⊢ size : uint64
―――――――――――――――――――――――――――――――――――――――  [T-BOX-CREATE]
S ⊢ box_create(name, size) : uint64
```

## Error Handling in Type System

### Type Errors

```
TypeError ::= 
  | StackUnderflow(expected: ℕ, actual: ℕ)
  | TypeMismatch(expected: Type, actual: Type)  
  | InvalidConversion(from: Type, to: Type)
  | UndefinedOperation(op: Opcode, types: Type*)
```

### Error Recovery

The type checker can employ error recovery strategies:

1. **Optimistic Typing**: Assume best-case types for ambiguous operations
2. **Conservative Typing**: Assume worst-case types to prevent false positives  
3. **Error Propagation**: Propagate type errors to enclosing contexts

## Implementation Considerations

### Type Representation

Types can be represented efficiently:

```rust
#[derive(Debug, Clone, PartialEq)]
enum TealType {
    Uint64,
    Bytes,
    Bottom, // Error type
}
```

### Type Stack Implementation

```rust
struct TypeStack {
    types: Vec<TealType>,
    max_depth: usize,
}

impl TypeStack {
    fn push(&mut self, ty: TealType) -> Result<(), TypeError> {
        if self.types.len() >= self.max_depth {
            return Err(TypeError::StackOverflow);
        }
        self.types.push(ty);
        Ok(())
    }
    
    fn pop(&mut self) -> Result<TealType, TypeError> {
        self.types.pop().ok_or(TypeError::StackUnderflow)
    }
}
```

### Type Checking Integration

The type checker integrates with the VM execution:

```rust
fn execute_with_typecheck(
    vm: &VirtualMachine,
    program: &[u8],
    type_check: bool,
) -> Result<bool, AvmError> {
    if type_check {
        typecheck_program(program)?;
    }
    vm.execute(program)
}
```

## Advanced Type Features

### Dependent Types (Future Extension)

For advanced verification, dependent types could be introduced:

```
DependentType ::= Π(x : τ₁). τ₂(x)
```

This would enable expressing properties like:
- Array bounds checking
- Resource usage bounds
- State transition constraints

### Linear Types (Future Extension)

Linear types could ensure resource safety:

```
LinearType ::= !τ  (exactly one use)
             | ?τ  (at most one use)  
             | &τ  (shared reference)
```

## Type System Correctness

### Decidability

**Theorem (Type Checking Decidability)**: The TEAL type checking problem is decidable.

**Proof**: The type system is finite (only two base types), and programs are finite. Type checking terminates in O(n) time where n is the program length.

### Completeness

**Theorem (Type System Completeness)**: If a program executes successfully without type errors, then it passes static type checking.

### Soundness

**Theorem (Type System Soundness)**: If a program passes static type checking, then it will not fail due to type errors during execution.

This type system provides the foundation for static analysis, optimization, and verification of TEAL programs while maintaining the simplicity and efficiency required for blockchain execution.