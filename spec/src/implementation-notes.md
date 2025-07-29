# Implementation Notes

This chapter provides practical guidance for implementing the Algorand Virtual Machine, including optimization strategies, edge case handling, performance considerations, and architectural patterns. These notes bridge the gap between formal specifications and production-ready implementations.

## Architecture Guidelines

### Modular Design Principles

**Component Separation**:
```rust
pub struct AvmImplementation {
    pub vm_core: VirtualMachine,           // Core execution engine
    pub opcode_registry: OpcodeRegistry,   // Opcode specifications
    pub type_checker: TypeChecker,         // Static analysis
    pub cost_tracker: CostAccumulator,     // Resource management
    pub state_manager: StateManager,       // Persistent state
    pub crypto_provider: CryptoProvider,   // Cryptographic operations
}
```

**Interface Design**:
```rust
pub trait AvmExecution {
    type Error: std::error::Error;
    
    fn execute(&mut self, program: &[u8], config: ExecutionConfig) 
        -> Result<bool, Self::Error>;
    
    fn create_context<'a>(&'a self, program: &'a [u8], config: ExecutionConfig) 
        -> Result<EvalContext<'a>, Self::Error>;
    
    fn step(&mut self, context: &mut EvalContext) 
        -> Result<StepResult, Self::Error>;
}
```

### Memory Management Strategy

**Stack Implementation**:
```rust
pub struct AvmStack {
    values: Vec<StackValue>,
    max_size: usize,
}

impl AvmStack {
    pub fn push(&mut self, value: StackValue) -> Result<(), StackOverflow> {
        if self.values.len() >= self.max_size {
            return Err(StackOverflow { limit: self.max_size });
        }
        self.values.push(value);
        Ok(())
    }
    
    pub fn pop(&mut self) -> Result<StackValue, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }
    
    // Optimized peek operations
    pub fn peek(&self, depth: usize) -> Result<&StackValue, StackUnderflow> {
        if depth >= self.values.len() {
            return Err(StackUnderflow);
        }
        let index = self.values.len() - 1 - depth;
        Ok(&self.values[index])
    }
}
```

**Memory Pooling**:
```rust
pub struct MemoryPool {
    small_blocks: Vec<Vec<u8>>,    // Reusable small allocations
    large_blocks: Vec<Vec<u8>>,    // Reusable large allocations
    threshold: usize,               // Size threshold for pooling
}

impl MemoryPool {
    pub fn allocate(&mut self, size: usize) -> Vec<u8> {
        if size <= self.threshold {
            self.small_blocks.pop()
                .map(|mut buf| { buf.clear(); buf.reserve(size); buf })
                .unwrap_or_else(|| Vec::with_capacity(size))
        } else {
            self.large_blocks.pop()
                .map(|mut buf| { buf.clear(); buf.reserve(size); buf })
                .unwrap_or_else(|| Vec::with_capacity(size))
        }
    }
    
    pub fn deallocate(&mut self, buffer: Vec<u8>) {
        if buffer.capacity() <= self.threshold {
            self.small_blocks.push(buffer);
        } else {
            self.large_blocks.push(buffer);
        }
    }
}
```

## Performance Optimization

### Opcode Dispatch Optimization

**Jump Table Implementation**:
```rust
type OpcodeHandler = fn(&mut EvalContext) -> AvmResult<()>;

pub struct OptimizedDispatcher {
    handlers: [OpcodeHandler; 256],  // Direct array lookup
    handler_costs: [u64; 256],       // Pre-computed costs
}

impl OptimizedDispatcher {
    pub fn execute_opcode(&self, opcode: u8, ctx: &mut EvalContext) 
        -> AvmResult<()> {
        // Fast bounds check
        let handler = unsafe { *self.handlers.get_unchecked(opcode as usize) };
        let cost = unsafe { *self.handler_costs.get_unchecked(opcode as usize) };
        
        // Check cost before execution
        ctx.add_cost(cost)?;
        
        // Execute handler
        handler(ctx)
    }
}
```

**Branch Prediction Friendly Code**:
```rust
// Optimize for common cases
impl EvalContext {
    #[inline(always)]
    pub fn execute_step(&mut self) -> AvmResult<bool> {
        // Most common case: PC in bounds
        if likely(self.pc < self.program.len()) {
            let opcode = unsafe { *self.program.get_unchecked(self.pc) };
            self.dispatch_opcode(opcode)?;
            Ok(false) // Continue execution
        } else {
            // Less common: end of program
            self.handle_program_end()
        }
    }
}

#[inline(always)]
fn likely(condition: bool) -> bool {
    std::intrinsics::likely(condition)
}
```

### Value Type Optimization

**Efficient Value Representation**:
```rust
#[derive(Clone, Debug)]
pub enum StackValue {
    Uint(u64),
    Bytes(SmallVec<[u8; 32]>),  // Inline small values
}

impl StackValue {
    // Optimized type checking
    #[inline(always)]
    pub fn is_uint(&self) -> bool {
        matches!(self, StackValue::Uint(_))
    }
    
    // Zero-copy uint extraction
    #[inline(always)]
    pub fn as_uint(&self) -> Result<u64, TypeError> {
        match self {
            StackValue::Uint(n) => Ok(*n),
            StackValue::Bytes(_) => Err(TypeError::ExpectedUint),
        }
    }
    
    // Efficient byte operations
    pub fn concat(&self, other: &StackValue) -> Result<StackValue, TypeError> {
        match (self, other) {
            (StackValue::Bytes(a), StackValue::Bytes(b)) => {
                let mut result = SmallVec::with_capacity(a.len() + b.len());
                result.extend_from_slice(a);
                result.extend_from_slice(b);
                Ok(StackValue::Bytes(result))
            },
            _ => Err(TypeError::ExpectedBytes),
        }
    }
}
```

### Cryptographic Operation Optimization

**Hardware Acceleration**:
```rust
pub struct OptimizedCrypto {
    sha256_accelerated: bool,
    ed25519_batch_verification: bool,
}

impl OptimizedCrypto {
    pub fn sha256(&self, data: &[u8]) -> [u8; 32] {
        if self.sha256_accelerated && data.len() > 1024 {
            // Use hardware SHA extensions if available
            self.hardware_sha256(data)
        } else {
            // Use optimized software implementation
            self.software_sha256(data)
        }
    }
    
    pub fn ed25519verify_batch(&self, operations: &[(Vec<u8>, Vec<u8>, Vec<u8>)]) 
        -> Vec<bool> {
        if self.ed25519_batch_verification && operations.len() > 1 {
            // Use batch verification for multiple signatures
            self.batch_verify_ed25519(operations)
        } else {
            // Fallback to individual verification
            operations.iter().map(|(data, sig, pubkey)| {
                self.verify_ed25519_single(data, sig, pubkey)
            }).collect()
        }
    }
}
```

## Error Handling Patterns

### Comprehensive Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AvmError {
    #[error("Stack overflow: attempted to exceed limit of {limit} elements")]
    StackOverflow { limit: usize },
    
    #[error("Stack underflow: attempted to pop from empty stack")]
    StackUnderflow,
    
    #[error("Program counter out of bounds: {pc} >= {program_len}")]
    ProgramCounterOutOfBounds { pc: usize, program_len: usize },
    
    #[error("Cost budget exceeded: {actual} > {limit}")]
    CostBudgetExceeded { actual: u64, limit: u64 },
    
    #[error("Invalid opcode: 0x{opcode:02x} at PC {pc}")]
    InvalidOpcode { opcode: u8, pc: usize },
    
    #[error("Type error: {message}")]
    TypeError { message: String },
    
    #[error("Cryptographic verification failed")]
    CryptoVerificationFailed,
    
    #[error("State access denied: {operation} on {key}")]
    StateAccessDenied { operation: String, key: String },
    
    #[error("Resource limit exceeded: {resource} = {actual} > {limit}")]
    ResourceLimitExceeded { 
        resource: String, 
        actual: usize, 
        limit: usize 
    },
}
```

### Error Recovery Strategies

```rust
pub enum ErrorRecovery {
    Halt,           // Stop execution immediately
    Skip,           // Skip current instruction and continue
    DefaultValue,   // Return default value and continue
    Retry(u32),     // Retry operation up to N times
}

impl EvalContext {
    pub fn handle_error(&mut self, error: AvmError, recovery: ErrorRecovery) 
        -> AvmResult<()> {
        match recovery {
            ErrorRecovery::Halt => Err(error),
            ErrorRecovery::Skip => {
                self.advance_pc(1)?;
                Ok(())
            },
            ErrorRecovery::DefaultValue => {
                self.push(StackValue::Uint(0))?;
                Ok(())
            },
            ErrorRecovery::Retry(n) if n > 0 => {
                // Implementation-specific retry logic
                self.retry_operation(n)
            },
            _ => Err(error),
        }
    }
}
```

## Edge Case Handling

### Arithmetic Edge Cases

```rust
impl ArithmeticOps {
    pub fn safe_add(a: u64, b: u64) -> Result<u64, ArithmeticError> {
        a.checked_add(b).ok_or(ArithmeticError::Overflow)
    }
    
    pub fn safe_sub(a: u64, b: u64) -> Result<u64, ArithmeticError> {
        a.checked_sub(b).ok_or(ArithmeticError::Underflow)
    }
    
    pub fn safe_mul(a: u64, b: u64) -> Result<u64, ArithmeticError> {
        a.checked_mul(b).ok_or(ArithmeticError::Overflow)
    }
    
    pub fn safe_div(a: u64, b: u64) -> Result<u64, ArithmeticError> {
        if b == 0 {
            Err(ArithmeticError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    // Handle edge cases in wide operations
    pub fn mulw(a: u64, b: u64) -> (u64, u64) {
        let result = (a as u128) * (b as u128);
        ((result >> 64) as u64, result as u64)
    }
}
```

### String and Byte Operations

```rust
impl ByteOps {
    pub fn safe_substring(data: &[u8], start: usize, length: usize) 
        -> Result<Vec<u8>, ByteOpError> {
        // Handle all edge cases
        if start > data.len() {
            return Err(ByteOpError::StartIndexOutOfBounds);
        }
        
        let end = start.saturating_add(length);
        if end > data.len() {
            return Err(ByteOpError::LengthExceedsData);
        }
        
        Ok(data[start..end].to_vec())
    }
    
    pub fn safe_extract_uint64(data: &[u8], offset: usize) 
        -> Result<u64, ByteOpError> {
        if offset + 8 > data.len() {
            return Err(ByteOpError::InsufficientData);
        }
        
        let bytes: [u8; 8] = data[offset..offset + 8]
            .try_into()
            .map_err(|_| ByteOpError::ConversionError)?;
        
        Ok(u64::from_be_bytes(bytes))
    }
}
```

### Stack Manipulation Edge Cases

```rust
impl StackOps {
    pub fn safe_dup_n(stack: &mut AvmStack, depth: usize) -> AvmResult<()> {
        // Check depth bounds
        if depth >= stack.size() {
            return Err(AvmError::StackUnderflow);
        }
        
        // Check if push would overflow
        if stack.size() >= stack.max_size() {
            return Err(AvmError::StackOverflow { 
                limit: stack.max_size() 
            });
        }
        
        let value = stack.peek(depth)?.clone();
        stack.push(value)
    }
    
    pub fn safe_swap(stack: &mut AvmStack) -> AvmResult<()> {
        if stack.size() < 2 {
            return Err(AvmError::StackUnderflow);
        }
        
        let a = stack.pop()?;
        let b = stack.pop()?;
        stack.push(a)?;
        stack.push(b)?;
        Ok(())
    }
}
```

## State Management Implementation

### Efficient State Caching

```rust
pub struct StateCache {
    global_cache: LruCache<(AppId, String), TealValue>,
    local_cache: LruCache<(AccountId, AppId, String), TealValue>,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

impl StateCache {
    pub fn get_global(&mut self, app_id: AppId, key: &str) 
        -> Option<&TealValue> {
        let cache_key = (app_id, key.to_string());
        if let Some(value) = self.global_cache.get(&cache_key) {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
            Some(value)
        } else {
            self.cache_misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }
    
    pub fn put_global(&mut self, app_id: AppId, key: String, value: TealValue) {
        self.global_cache.put((app_id, key), value);
    }
    
    pub fn cache_stats(&self) -> (u64, u64) {
        (
            self.cache_hits.load(Ordering::Relaxed),
            self.cache_misses.load(Ordering::Relaxed)
        )
    }
}
```

### Transaction Batching

```rust
pub struct StateBatch {
    global_operations: Vec<GlobalStateOp>,
    local_operations: Vec<LocalStateOp>,
    box_operations: Vec<BoxStateOp>,
}

impl StateBatch {
    pub fn apply_batch(&self, ledger: &mut dyn LedgerAccess) 
        -> Result<(), StateError> {
        // Apply all operations atomically
        ledger.begin_transaction()?;
        
        match self.apply_operations(ledger) {
            Ok(()) => {
                ledger.commit_transaction()?;
                Ok(())
            },
            Err(e) => {
                ledger.rollback_transaction()?;
                Err(e)
            }
        }
    }
    
    fn apply_operations(&self, ledger: &mut dyn LedgerAccess) 
        -> Result<(), StateError> {
        // Apply in dependency order
        for op in &self.global_operations {
            op.apply(ledger)?;
        }
        
        for op in &self.local_operations {
            op.apply(ledger)?;
        }
        
        for op in &self.box_operations {
            op.apply(ledger)?;
        }
        
        Ok(())
    }
}
```

## Testing Infrastructure

### Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use quickcheck::{quickcheck, TestResult};
    use super::*;
    
    #[quickcheck]
    fn stack_operations_preserve_invariants(ops: Vec<StackOp>) -> TestResult {
        let mut stack = AvmStack::new(1000);
        let mut total_size = 0;
        
        for op in ops {
            match op {
                StackOp::Push(value) => {
                    if total_size < 1000 {
                        assert!(stack.push(value).is_ok());
                        total_size += 1;
                    }
                },
                StackOp::Pop => {
                    if total_size > 0 {
                        assert!(stack.pop().is_ok());
                        total_size -= 1;
                    }
                },
            }
            
            // Invariant: stack size matches our tracking
            assert_eq!(stack.size(), total_size);
        }
        
        TestResult::passed()
    }
    
    #[quickcheck]
    fn arithmetic_operations_dont_panic(a: u64, b: u64) -> bool {
        // All arithmetic operations should handle overflow gracefully
        let _ = ArithmeticOps::safe_add(a, b);
        let _ = ArithmeticOps::safe_sub(a, b);
        let _ = ArithmeticOps::safe_mul(a, b);
        let _ = ArithmeticOps::safe_div(a, b);
        true
    }
}
```

### Fuzzing Integration

```rust
#[cfg(fuzzing)]
pub fn fuzz_execute_program(data: &[u8]) -> bool {
    if data.len() < 4 {
        return true; // Skip short inputs
    }
    
    let version = TealVersion::from_u8(data[0] % 12).unwrap_or(TealVersion::V1);
    let program = &data[1..];
    
    let mut vm = VirtualMachine::with_version(version);
    let mut ledger = MockLedger::new();
    let config = ExecutionConfig::new(version);
    
    // Execution should never panic, only return errors
    let _ = vm.execute(program, config, &mut ledger);
    true
}

// Libfuzzer entry point
#[no_mangle]
pub extern "C" fn LLVMFuzzerTestOneInput(data: *const u8, size: usize) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(data, size) };
    if fuzz_execute_program(slice) { 0 } else { 1 }
}
```

## Debugging and Observability

### Execution Tracing

```rust
pub struct ExecutionTracer {
    events: Vec<TraceEvent>,
    enabled: bool,
    max_events: usize,
}

#[derive(Debug, Clone)]
pub enum TraceEvent {
    OpcodExecuted { 
        pc: usize, 
        opcode: u8, 
        name: String,
        cost: u64,
        stack_before: Vec<StackValue>,
        stack_after: Vec<StackValue>,
    },
    BranchTaken { 
        from: usize, 
        to: usize, 
        condition: bool 
    },
    StateAccess { 
        operation: StateOperation,
        key: String,
        value: Option<TealValue>,
    },
    CostIncurred { 
        amount: u64, 
        total: u64, 
        budget: u64 
    },
}

impl ExecutionTracer {
    pub fn trace_opcode(&mut self, ctx: &EvalContext, spec: &OpSpec) {
        if !self.enabled || self.events.len() >= self.max_events {
            return;
        }
        
        self.events.push(TraceEvent::OpcodExecuted {
            pc: ctx.pc(),
            opcode: spec.opcode,
            name: spec.name.clone(),
            cost: spec.cost,
            stack_before: ctx.stack().to_vec(),
            stack_after: ctx.stack().to_vec(), // Will be updated after execution
        });
    }
    
    pub fn export_trace(&self) -> String {
        serde_json::to_string_pretty(&self.events)
            .unwrap_or_else(|_| "Failed to serialize trace".to_string())
    }
}
```

### Performance Monitoring

```rust
pub struct PerformanceMonitor {
    opcode_counts: HashMap<u8, u64>,
    opcode_times: HashMap<u8, Duration>,
    total_execution_time: Duration,
    memory_usage: usize,
}

impl PerformanceMonitor {
    pub fn start_opcode_timing(&mut self, opcode: u8) -> Instant {
        *self.opcode_counts.entry(opcode).or_insert(0) += 1;
        Instant::now()
    }
    
    pub fn end_opcode_timing(&mut self, opcode: u8, start: Instant) {
        let duration = start.elapsed();
        *self.opcode_times.entry(opcode).or_insert(Duration::ZERO) += duration;
    }
    
    pub fn report_performance(&self) -> PerformanceReport {
        let mut hottest_opcodes: Vec<_> = self.opcode_times.iter()
            .map(|(&opcode, &time)| (opcode, time, self.opcode_counts[&opcode]))
            .collect();
        hottest_opcodes.sort_by_key(|(_, time, _)| std::cmp::Reverse(*time));
        
        PerformanceReport {
            total_time: self.total_execution_time,
            hottest_opcodes: hottest_opcodes.into_iter().take(10).collect(),
            memory_usage: self.memory_usage,
        }
    }
}
```

## Security Implementation Notes

### Input Validation

```rust
pub struct InputValidator;

impl InputValidator {
    pub fn validate_program(program: &[u8]) -> Result<(), ValidationError> {
        if program.is_empty() {
            return Err(ValidationError::EmptyProgram);
        }
        
        if program.len() > MAX_PROGRAM_SIZE {
            return Err(ValidationError::ProgramTooLarge {
                size: program.len(),
                limit: MAX_PROGRAM_SIZE,
            });
        }
        
        // Check for invalid opcodes
        let mut pc = 0;
        while pc < program.len() {
            let opcode = program[pc];
            let spec = OPCODE_SPECS.get(&opcode)
                .ok_or(ValidationError::InvalidOpcode { opcode, pc })?;
            
            // Validate immediate arguments
            let arg_bytes = spec.size - 1;
            if pc + arg_bytes >= program.len() {
                return Err(ValidationError::IncompleteInstruction { pc });
            }
            
            pc += spec.size;
        }
        
        Ok(())
    }
    
    pub fn validate_stack_value(value: &StackValue) -> Result<(), ValidationError> {
        match value {
            StackValue::Uint(_) => Ok(()), // Always valid
            StackValue::Bytes(bytes) => {
                if bytes.len() > MAX_BYTE_VALUE_SIZE {
                    Err(ValidationError::ValueTooLarge {
                        size: bytes.len(),
                        limit: MAX_BYTE_VALUE_SIZE,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}
```

### Constant-Time Operations

```rust
pub struct ConstantTimeCrypto;

impl ConstantTimeCrypto {
    // Constant-time comparison to prevent timing attacks
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (a_byte, b_byte) in a.iter().zip(b.iter()) {
            result |= a_byte ^ b_byte;
        }
        result == 0
    }
    
    // Constant-time conditional selection
    pub fn conditional_select(condition: bool, a: &[u8], b: &[u8]) -> Vec<u8> {
        assert_eq!(a.len(), b.len());
        
        let mask = if condition { 0xFF } else { 0x00 };
        a.iter().zip(b.iter())
            .map(|(&a_byte, &b_byte)| (a_byte & mask) | (b_byte & !mask))
            .collect()
    }
}
```

## Platform-Specific Optimizations

### SIMD Optimizations

```rust
#[cfg(target_arch = "x86_64")]
mod x86_optimizations {
    use std::arch::x86_64::*;
    
    pub fn simd_xor(a: &[u8], b: &[u8], result: &mut [u8]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());
        
        let chunks = a.len() / 32;
        let remainder = a.len() % 32;
        
        unsafe {
            for i in 0..chunks {
                let offset = i * 32;
                let a_chunk = _mm256_loadu_si256(a[offset..].as_ptr() as *const __m256i);
                let b_chunk = _mm256_loadu_si256(b[offset..].as_ptr() as *const __m256i);
                let result_chunk = _mm256_xor_si256(a_chunk, b_chunk);
                _mm256_storeu_si256(result[offset..].as_mut_ptr() as *mut __m256i, result_chunk);
            }
        }
        
        // Handle remainder bytes
        let offset = chunks * 32;
        for i in 0..remainder {
            result[offset + i] = a[offset + i] ^ b[offset + i];
        }
    }
}
```

### Memory-Mapped I/O

```rust
#[cfg(unix)]
mod unix_optimizations {
    use memmap2::{Mmap, MmapOptions};
    use std::fs::File;
    
    pub struct MmapStateStorage {
        mmap: Mmap,
        size: usize,
    }
    
    impl MmapStateStorage {
        pub fn new(file: File, size: usize) -> Result<Self, std::io::Error> {
            let mmap = unsafe {
                MmapOptions::new()
                    .len(size)
                    .map(&file)?
            };
            
            Ok(Self { mmap, size })
        }
        
        pub fn read_state(&self, offset: usize, len: usize) -> &[u8] {
            &self.mmap[offset..offset + len]
        }
        
        // Note: This requires a mutable mapping for write operations
        pub fn write_state(&mut self, offset: usize, data: &[u8]) -> Result<(), std::io::Error> {
            // Would require MmapMut for actual implementation
            todo!("Implement with MmapMut")
        }
    }
}
```

## Integration Patterns

### Async Execution Support

```rust
pub struct AsyncAvmExecutor {
    executor: tokio::runtime::Runtime,
    semaphore: Arc<Semaphore>,
}

impl AsyncAvmExecutor {
    pub async fn execute_async(&self, program: Vec<u8>, config: ExecutionConfig) 
        -> Result<bool, AvmError> {
        let _permit = self.semaphore.acquire().await
            .map_err(|_| AvmError::ResourceExhausted)?;
        
        let result = tokio::task::spawn_blocking(move || {
            let mut vm = VirtualMachine::with_version(config.version);
            let mut ledger = MockLedger::new();
            vm.execute(&program, config, &mut ledger)
        }).await;
        
        match result {
            Ok(execution_result) => execution_result,
            Err(join_error) => Err(AvmError::ExecutionError {
                message: format!("Async execution failed: {}", join_error)
            }),
        }
    }
}
```

### FFI Bindings

```rust
// C FFI for other language integration
#[repr(C)]
pub struct AvmExecutionResult {
    success: bool,
    error_code: i32,
    error_message: *const c_char,
    final_stack_size: usize,
    cost_used: u64,
}

#[no_mangle]
pub extern "C" fn avm_execute(
    program: *const u8,
    program_len: usize,
    version: u8,
    mode: u8,
    result: *mut AvmExecutionResult,
) -> i32 {
    let program_slice = unsafe { 
        std::slice::from_raw_parts(program, program_len) 
    };
    
    let version = TealVersion::from_u8(version).unwrap_or(TealVersion::V1);
    let run_mode = if mode == 0 { RunMode::Signature } else { RunMode::Application };
    
    let config = ExecutionConfig::new(version).with_run_mode(run_mode);
    let mut vm = VirtualMachine::with_version(version);
    let mut ledger = MockLedger::new();
    
    match vm.execute(program_slice, config, &mut ledger) {
        Ok(success) => {
            unsafe {
                (*result).success = success;
                (*result).error_code = 0;
                (*result).error_message = std::ptr::null();
                (*result).cost_used = 0; // Would need to track this
            }
            0
        },
        Err(error) => {
            let error_message = CString::new(error.to_string()).unwrap();
            unsafe {
                (*result).success = false;
                (*result).error_code = -1;
                (*result).error_message = error_message.into_raw();
            }
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn avm_free_error_message(message: *mut c_char) {
    if !message.is_null() {
        unsafe {
            let _ = CString::from_raw(message);
        }
    }
}
```

## Deployment Considerations

### Configuration Management

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvmConfig {
    pub max_stack_size: usize,
    pub max_call_depth: usize,
    pub cost_budgets: HashMap<RunMode, u64>,
    pub enabled_opcodes: HashSet<u8>,
    pub crypto_provider: CryptoProviderConfig,
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
}

impl Default for AvmConfig {
    fn default() -> Self {
        Self {
            max_stack_size: 1000,
            max_call_depth: 8,
            cost_budgets: [
                (RunMode::Signature, 700),
                (RunMode::Application, 20000),
            ].into_iter().collect(),
            enabled_opcodes: HashSet::new(), // Empty = all enabled
            crypto_provider: CryptoProviderConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}
```

### Monitoring and Metrics

```rust
pub struct AvmMetrics {
    pub executions_total: Counter,
    pub execution_duration: Histogram,
    pub opcode_counts: HashMap<u8, Counter>,
    pub error_counts: HashMap<String, Counter>,
    pub memory_usage: Gauge,
}

impl AvmMetrics {
    pub fn record_execution(&self, duration: Duration, success: bool, error: Option<&AvmError>) {
        self.executions_total.increment();
        self.execution_duration.record(duration);
        
        if let Some(error) = error {
            let error_type = error.error_type();
            self.error_counts.entry(error_type).or_insert_with(|| Counter::new()).increment();
        }
    }
    
    pub fn record_opcode(&self, opcode: u8) {
        self.opcode_counts.entry(opcode).or_insert_with(|| Counter::new()).increment();
    }
}
```

These implementation notes provide practical guidance for building production-ready AVM implementations that are performant, secure, and maintainable. The patterns and optimizations shown here represent industry best practices adapted specifically for virtual machine implementation in blockchain environments.
