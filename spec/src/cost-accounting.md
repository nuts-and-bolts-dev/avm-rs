# Cost Accounting

This chapter provides a comprehensive formal specification of the AVM cost accounting system, including cost models, resource management, and budget enforcement mechanisms that prevent denial-of-service attacks and ensure fair resource utilization.

## Cost Model Overview

The AVM employs a comprehensive cost accounting system to prevent resource exhaustion attacks and ensure fair computational resource allocation across all programs executed on the network.

### Fundamental Cost Principles

1. **Deterministic Costing**: All operations have fixed, deterministic costs
2. **Conservative Estimation**: Costs are designed to be conservative upper bounds
3. **Resource Proportionality**: Costs reflect actual computational and memory resources
4. **Attack Prevention**: Cost budgets prevent infinite loops and resource exhaustion

### Cost Universe Definition

```
Cost = ℕ⁺
CostBudget = ℕ⁺  
CostAccumulator = ℕ

CostFunction : Opcode → Cost
CostBudget_Mode : RunMode → CostBudget
```

## Cost Function Specification

### Base Cost Model

Each opcode has an associated execution cost:

```
cost : Opcode → Cost

cost(op) = base_cost(op) + dynamic_cost(op, inputs)
```

Where:
- `base_cost(op)`: Fixed cost component for opcode execution
- `dynamic_cost(op, inputs)`: Variable cost based on input size/complexity

### Cost Categories

#### Basic Operations (Cost: 1)

```
BasicOps = {
  // Stack operations
  push, pop, dup, swap, select,
  // Arithmetic 
  +, -, *, %, <, >, <=, >=, ==, !=, &&, ||, !,
  // Bitwise operations
  &, |, ^, ~,
  // Type conversion
  itob, btoi, len
}

∀op ∈ BasicOps. cost(op) = 1
```

#### Advanced Arithmetic (Cost: 1-20)

```
cost(mulw) = 10        // Wide multiplication
cost(addw) = 10        // Wide addition  
cost(divmodw) = 20     // Division with remainder
cost(divw) = 20        // Wide division
cost(expw) = 10        // Wide exponentiation
cost(sqrt) = 4         // Square root
cost(exp) = 1          // Basic exponentiation
cost(shl) = 1          // Shift left
cost(shr) = 1          // Shift right
cost(bitlen) = 1       // Bit length
```

#### Cryptographic Operations (Cost: 35-8000)

```
cost(sha256) = 35
cost(sha512_256) = 45
cost(sha3_256) = 45
cost(keccak256) = 130
cost(ed25519verify) = 1900
cost(ed25519verify_bare) = 1900
cost(ecdsa_verify) = 1700
cost(ecdsa_pk_decompress) = 650
cost(ecdsa_pk_recover) = 2000
cost(vrf_verify) = 5700
```

#### Elliptic Curve Operations (Cost: 100-8000)

```
cost(ec_add) = 100
cost(ec_scalar_mul) = 1000
cost(ec_multi_scalar_mul) = 3000
cost(ec_subgroup_check) = 500
cost(ec_map_to) = 200
cost(ec_pairing_check) = 8000
```

#### Storage Operations (Cost: 40-400)

```
cost(box_create) = 400
cost(box_extract) = 40
cost(box_replace) = 40
cost(box_del) = 40
cost(box_len) = 40
cost(box_get) = 40
cost(box_put) = 40
cost(box_splice) = 40
cost(box_resize) = 40
```

### Dynamic Cost Components

#### Input Size-Dependent Costs

For operations that process variable-size inputs:

```
dynamic_cost(op, inputs) = Σᵢ size_factor(op) × |inputᵢ|

Where:
  size_factor(concat) = 0.1    // Per byte concatenated
  size_factor(hash_ops) = 0.01 // Per byte hashed  
  size_factor(crypto_ops) = 0.5 // Per byte processed
```

#### Memory Allocation Costs

```
memory_cost(allocation_size) = ⌈allocation_size / 32⌉

Examples:
  cost(bzero(n)) = 1 + memory_cost(n)
  cost(substring(s, start, len)) = 1 + memory_cost(len)
```

## Budget Management

### Budget Assignment by Mode

```
DefaultBudget : RunMode → CostBudget

DefaultBudget(Signature) = 700
DefaultBudget(Application) = 20000
```

### Rationale for Budget Limits

#### Signature Mode (700 units)
- Designed for lightweight transaction validation
- Sufficient for complex cryptographic verification
- Prevents expensive operations that could slow transaction processing

#### Application Mode (20,000 units)  
- Allows complex smart contract logic
- Enables multiple cryptographic operations
- Supports significant state manipulation

### Budget Enforcement

```
execute_with_budget : State × Opcode × CostBudget → State' ∪ {BudgetExceeded}

execute_with_budget(⟨S, P, pc, C, σ, L⟩, op, budget) =
  let op_cost = cost(op)
  if C + op_cost > budget then
    BudgetExceeded
  else
    let new_state = execute_opcode(⟨S, P, pc, C, σ, L⟩, op)
    update_cost(new_state, C + op_cost)
```

## Cost Accumulation Semantics

### Cost State Transition Rules

```
                    C + cost(op) ≤ budget
―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
⟨S, P, pc, C, σ, L⟩ →cost(op) ⟨S', P, pc', C + cost(op), σ', L'⟩
```

```
                    C + cost(op) > budget
―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
⟨S, P, pc, C, σ, L⟩ →cost(op) BudgetExceeded
```

### Cost Monotonicity

**Invariant (Cost Monotonicity)**: For any execution sequence:
```
∀i,j. i < j ⇒ Cᵢ ≤ Cⱼ
```

Where `Cᵢ` is the cost accumulator at step `i`.

**Proof**: Each transition adds a non-negative cost: `cost(op) ≥ 0` for all opcodes.

## Resource-Based Cost Analysis

### Computational Complexity Mapping

```
ComplexityClass : Opcode → {O(1), O(n), O(n²), O(n³)}

// Constant time operations
∀op ∈ BasicOps. ComplexityClass(op) = O(1)

// Linear operations  
ComplexityClass(concat) = O(n)
ComplexityClass(hash_ops) = O(n)

// Quadratic operations
ComplexityClass(ed25519verify) = O(n²)

// Cubic operations
ComplexityClass(ec_pairing_check) = O(n³)
```

### Memory Usage Estimation

```
MemoryUsage : Opcode × Inputs → Bytes

MemoryUsage(bzero, [n]) = n
MemoryUsage(concat, [a, b]) = |a| + |b|
MemoryUsage(substring, [s, start, len]) = len
MemoryUsage(box_create, [name, size]) = |name| + size
```

### I/O Cost Modeling

```
IOCost : Opcode → Cost

// Ledger read operations
IOCost(app_global_get) = 1
IOCost(app_local_get) = 1
IOCost(balance) = 1

// Ledger write operations  
IOCost(app_global_put) = 1
IOCost(app_local_put) = 1
IOCost(log) = 1
```

## Advanced Cost Features

### Dynamic Budget Adjustment

For future extensibility, budgets may be adjusted based on:

```
AdjustedBudget(base_budget, factors) = 
  base_budget × Π factors

Where factors may include:
- Network congestion multiplier
- Account-specific limits
- Application complexity rating
```

### Cost Pooling (Multi-Transaction Groups)

```
GroupBudget = Σᵢ IndividualBudgetᵢ × pool_factor

pool_factor ∈ [0.8, 1.2]  // Allow some redistribution
```

### Precomputation Cost Caching

```
PrecomputeCache : (Opcode × Inputs) → Cost

// For expensive operations with common inputs
PrecomputeCache((ed25519verify, (data, sig, pubkey))) = 
  if (data, sig, pubkey) ∈ CommonInputs then 1900 × 0.1 else 1900
```

## Cost Verification and Analysis

### Static Cost Analysis

```
StaticCostAnalysis : Program → CostBound

StaticCostAnalysis(P) = Σ_{op ∈ P} MaxCost(op)

Where MaxCost(op) considers worst-case input sizes
```

### Loop Cost Bounds

```
LoopCostBound : Program × BranchTargets → CostBound

// For programs with loops, establish maximum iteration bounds
MaxIterations(loop) ≤ budget / MinCostPerIteration(loop)
```

### Cost Estimation for Complex Programs

```
EstimateCost : Program × InputDistribution → CostEstimate

EstimateCost(P, D) = Σᵢ Pr(pathᵢ|D) × Cost(pathᵢ)
```

## Implementation Considerations

### Cost Counter Implementation

```rust
#[derive(Debug, Clone)]
pub struct CostAccumulator {
    current: u64,
    budget: u64,
    checkpoints: Vec<u64>, // For rollback
}

impl CostAccumulator {
    pub fn add_cost(&mut self, cost: u64) -> Result<(), BudgetExceeded> {
        if self.current + cost > self.budget {
            return Err(BudgetExceeded {
                required: self.current + cost,
                available: self.budget,
            });
        }
        self.current += cost;
        Ok(())
    }
    
    pub fn remaining(&self) -> u64 {
        self.budget.saturating_sub(self.current)
    }
}
```

### Opcode Cost Table Generation

```rust
pub fn generate_cost_table(version: TealVersion) -> HashMap<u8, u64> {
    let mut costs = HashMap::new();
    
    // Basic operations
    for &op in &BASIC_OPS {
        costs.insert(op, 1);
    }
    
    // Cryptographic operations
    costs.insert(OP_SHA256, 35);
    costs.insert(OP_ED25519VERIFY, 1900);
    
    // Version-specific cost adjustments
    match version {
        TealVersion::V8..=TealVersion::V11 => {
            costs.insert(OP_BOX_CREATE, 400);
        }
        _ => {}
    }
    
    costs
}
```

## Security Analysis

### DoS Attack Prevention

**Theorem (DoS Prevention)**: The cost accounting system prevents computational DoS attacks.

**Proof**: 
1. All operations have finite, bounded costs
2. Total program cost is bounded by the budget
3. Budget limits are set below system resource thresholds
4. Therefore, no single program can exhaust system resources

### Resource Exhaustion Bounds

```
MaxSystemLoad = NetworkTPS × MaxBudget × ExecutionTime

Where:
  NetworkTPS ≤ 1000 // Transactions per second
  MaxBudget = 20000 // Application mode budget
  ExecutionTime ≤ 10ms // Maximum allowed execution time
```

### Economic Security

Cost accounting provides economic security through:

1. **Fair Resource Allocation**: Cost reflects actual resource usage
2. **Attack Cost**: Expensive operations require proportional fees
3. **Market Mechanisms**: Cost can be adjusted based on network demand

## Cost Model Validation

### Benchmarking Methodology

```
Benchmark : Opcode → (WallTime, MemoryUsage, IOOperations)

cost(op) should satisfy:
  cost(op) ≥ α × WallTime(op) + β × MemoryUsage(op) + γ × IOOperations(op)

Where α, β, γ are calibration constants
```

### Empirical Cost Validation

```
ValidateCosts : CostTable × BenchmarkResults → ValidationReport

ValidationReport = {
  overestimated: Set<Opcode>,    // Costs too high
  underestimated: Set<Opcode>,   // Costs too low  
  calibration_factor: Float,     // Overall adjustment needed
}
```

## Future Cost Model Extensions

### Machine Learning-Based Cost Prediction

```
MLCostPredictor : (Opcode, InputFeatures) → CostEstimate

Where InputFeatures may include:
- Input sizes
- Stack state
- Historical execution patterns
- Network conditions
```

### Adaptive Cost Adjustment

```
AdaptiveCost(op, network_state) = 
  base_cost(op) × congestion_factor(network_state)
```

### Gas-Style Metering (Future)

```
GasPrice : Time → Cost/Gas
GasLimit : Transaction → Gas
GasCost : Opcode → Gas

TotalCost = GasUsed × GasPrice
```

## Cost Accounting Correctness

### Determinism

**Theorem (Cost Determinism)**: For any program P and input I, the total execution cost is deterministic.

**Proof**: Each opcode has a fixed cost, and execution is deterministic, therefore total cost is deterministic.

### Termination

**Theorem (Bounded Execution)**: All programs terminate within finite time due to cost bounds.

**Proof**: 
1. Each step consumes at least 1 cost unit
2. Total cost is bounded by the budget
3. Therefore, execution must terminate within `budget` steps

### Fairness

**Theorem (Resource Fairness)**: The cost system ensures fair resource allocation among programs.

**Proof**: All programs are subject to the same cost model and budget constraints, ensuring no single program can monopolize resources.

This cost accounting system provides the mathematical foundation for secure, efficient, and fair resource management in the Algorand Virtual Machine, preventing attacks while enabling complex smart contract functionality.
