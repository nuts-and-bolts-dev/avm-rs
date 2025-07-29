# Introduction

The Algorand Virtual Machine (AVM) is a stack-based virtual machine that executes TEAL (Transaction Execution Approval Language) bytecode for smart contract logic and transaction validation on the Algorand blockchain. This document provides comprehensive formal specifications for the AVM, designed to be sound, complete, and aligned with the official go-algorand reference implementation.

## Purpose and Scope

These formal specifications serve multiple critical purposes:

1. **Implementation Guide**: Provide precise behavioral specifications for building AVM implementations
2. **Verification Target**: Enable formal verification of AVM implementations against mathematical models
3. **Security Analysis**: Support rigorous security property analysis and proof
4. **Testing Framework**: Serve as the foundation for comprehensive test generation
5. **Documentation**: Offer precise, unambiguous behavioral specifications

## AVM Overview

The AVM is characterized by:

- **Stack-Based Architecture**: Operations manipulate a stack of values with maximum depth of 1000 elements
- **Dual Execution Modes**: 
  - *Signature Mode* (stateless): For Smart Signatures that validate transactions
  - *Application Mode* (stateful): For Smart Contracts with persistent state
- **Versioned Instruction Set**: TEAL versions 1-11 with backward compatibility
- **Resource Constraints**: Cost accounting and computational limits prevent abuse
- **Type Safety**: Strong typing with uint64 integers and byte arrays

## Mathematical Foundations

Our specifications employ rigorous mathematical foundations:

### Operational Semantics
We define the AVM using small-step operational semantics, representing execution as state transitions:

```
⟨S, P, pc, C, σ, L⟩ → ⟨S', P, pc', C', σ', L'⟩
```

Where:
- `S` is the evaluation stack
- `P` is the program bytecode  
- `pc` is the program counter
- `C` is the cost accumulator
- `σ` is the scratch space
- `L` is the ledger state

### Denotational Semantics
We provide mathematical meaning to programs through denotational semantics, mapping programs to mathematical functions over execution states.

### Type System
The AVM type system is formally specified with typing judgments and type safety theorems:

```
Γ ⊢ e : τ
```

This reads as "under type environment Γ, expression e has type τ".

## Specification Structure

The specifications are organized as follows:

1. **[Virtual Machine Model](./core-vm.md)**: Core execution model, state transitions, and abstract machine definition
2. **[Opcode Semantics](./opcodes.md)**: Formal operational semantics for each TEAL opcode
3. **[Type System](./type-system.md)**: Type rules, safety properties, and static analysis foundations
4. **[State Model](./state-model.md)**: Global/local state, transaction groups, and ledger interactions
5. **[Cost Accounting](./cost-accounting.md)**: Resource management, cost models, and budget enforcement
6. **[Soundness and Completeness](./soundness-completeness.md)**: Formal proofs of correctness properties
7. **[Security Properties](./security-properties.md)**: Security invariants and threat model analysis

## Notation Conventions

### Mathematical Notation

| Symbol | Meaning |
|--------|---------|
| `⟨...⟩` | Execution state tuple |
| `→` | Single-step transition |
| `→*` | Multi-step transition (reflexive transitive closure) |
| `⊢` | Typing judgment |
| `⊥` | Error/undefined state |
| `∅` | Empty set/stack |
| `∈` | Set membership |
| `⊆` | Subset relation |
| `∪` | Set union |
| `∩` | Set intersection |

### Type Notation

| Type | Description |
|------|-------------|
| `uint64` | 64-bit unsigned integer |
| `bytes` | Variable-length byte array |
| `bool` | Boolean (derived from uint64: 0 = false, ≠0 = true) |
| `τ₁ × τ₂` | Product type |
| `τ₁ + τ₂` | Sum type |

### Operational Rules Format

```
Premise₁  Premise₂  ...  Premiseₙ
―――――――――――――――――――――――――――――――  [Rule-Name]
        Conclusion
```

## Implementation Alignment

These specifications maintain careful alignment with the go-algorand reference implementation while remaining abstract enough to permit multiple correct implementations. Key alignment principles:

- **Behavioral Equivalence**: All specified behaviors match reference implementation
- **Error Conditions**: Complete coverage of error cases and edge conditions  
- **Resource Limits**: Exact specification of computational and memory limits
- **Version Compatibility**: Precise handling of TEAL version differences

## Verification Approach

The specifications support multiple verification methodologies:

1. **Model Checking**: State space exploration for finite-state properties
2. **Theorem Proving**: Interactive proofs of safety and liveness properties
3. **Property-Based Testing**: Automated test generation from formal properties
4. **Refinement Verification**: Proving implementation correctness against specifications

## Security Considerations

Security is a primary concern throughout these specifications:

- **Resource Exhaustion**: Formal cost models prevent DoS attacks
- **Type Safety**: Strong typing prevents many classes of vulnerabilities
- **State Isolation**: Clear boundaries between different execution contexts
- **Determinism**: Guaranteed deterministic execution across all nodes

These formal specifications provide the mathematical foundation necessary for building secure, correct, and verifiable AVM implementations.