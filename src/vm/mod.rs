//! Virtual Machine implementation

use crate::error::{AvmError, AvmResult};
use crate::opcodes::{OpSpec, get_standard_opcodes};
use crate::state::LedgerAccess;
use crate::types::{RunMode, StackValue, TealValue, TealVersion};
use std::collections::HashMap;

/// Maximum stack size
pub const MAX_STACK_SIZE: usize = 1000;

/// Maximum call stack depth
pub const MAX_CALL_STACK_DEPTH: usize = 8;

/// Scratch space size
pub const SCRATCH_SIZE: usize = 256;

/// Execution configuration
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub run_mode: RunMode,
    pub cost_budget: u64,
    pub version: TealVersion,
    pub group_index: usize,
    pub group_size: usize,
}

impl ExecutionConfig {
    /// Create a new execution configuration with defaults
    pub fn new(version: TealVersion) -> Self {
        Self {
            run_mode: RunMode::Signature,
            cost_budget: 700, // Default budget for signature mode
            version,
            group_index: 0,
            group_size: 1,
        }
    }

    /// Create configuration for application mode
    pub fn application(version: TealVersion) -> Self {
        Self {
            run_mode: RunMode::Application,
            cost_budget: 20000, // Higher budget for application mode
            version,
            group_index: 0,
            group_size: 1,
        }
    }

    /// Set the cost budget
    pub fn with_cost_budget(mut self, budget: u64) -> Self {
        self.cost_budget = budget;
        self
    }

    /// Set the group configuration
    pub fn with_group(mut self, index: usize, size: usize) -> Self {
        self.group_index = index;
        self.group_size = size;
        self
    }

    /// Set the run mode
    pub fn with_run_mode(mut self, run_mode: RunMode) -> Self {
        self.run_mode = run_mode;
        self
    }
}

/// Evaluation context for the AVM
#[derive(Debug)]
pub struct EvalContext<'a> {
    /// Stack for operands
    stack: Vec<StackValue>,

    /// Program bytecode
    program: &'a [u8],

    /// Program counter
    pc: usize,

    /// Execution mode
    run_mode: RunMode,

    /// Cost budget
    cost_budget: u64,

    /// Current cost
    cost: u64,

    /// TEAL version
    version: TealVersion,

    /// Scratch space (256 slots)
    scratch: [StackValue; SCRATCH_SIZE],

    /// Call stack for subroutines
    call_stack: Vec<usize>,

    /// Group index in transaction group
    group_index: usize,

    /// Transaction group size
    group_size: usize,

    /// Ledger access interface
    ledger: &'a dyn LedgerAccess,

    /// Global state cache
    #[allow(dead_code)]
    global_state_cache: HashMap<String, TealValue>,

    /// Local state cache
    #[allow(dead_code)]
    local_state_cache: HashMap<(Vec<u8>, String), TealValue>,

    /// Execution trace (for debugging)
    trace: Vec<String>,

    /// Enable tracing
    trace_enabled: bool,
}

impl<'a> EvalContext<'a> {
    /// Create a new evaluation context
    pub fn new(
        program: &'a [u8],
        run_mode: RunMode,
        cost_budget: u64,
        version: TealVersion,
        group_index: usize,
        group_size: usize,
        ledger: &'a dyn LedgerAccess,
    ) -> Self {
        Self {
            stack: Vec::new(),
            program,
            pc: 0,
            run_mode,
            cost_budget,
            cost: 0,
            version,
            scratch: std::array::from_fn(|_| StackValue::uint(0)),
            call_stack: Vec::new(),
            group_index,
            group_size,
            ledger,
            global_state_cache: HashMap::new(),
            local_state_cache: HashMap::new(),
            trace: Vec::new(),
            trace_enabled: false,
        }
    }

    /// Enable or disable execution tracing
    pub fn set_trace_enabled(&mut self, enabled: bool) {
        self.trace_enabled = enabled;
    }

    /// Get the execution trace
    pub fn trace(&self) -> &[String] {
        &self.trace
    }

    /// Add a trace entry
    pub fn add_trace(&mut self, message: String) {
        if self.trace_enabled {
            self.trace.push(message);
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: StackValue) -> AvmResult<()> {
        if self.stack.len() >= MAX_STACK_SIZE {
            return Err(AvmError::StackOverflow {
                limit: MAX_STACK_SIZE,
            });
        }
        self.stack.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> AvmResult<StackValue> {
        self.stack.pop().ok_or(AvmError::StackUnderflow)
    }

    /// Peek at the top value on the stack without removing it
    pub fn peek(&self) -> AvmResult<&StackValue> {
        self.stack.last().ok_or(AvmError::StackUnderflow)
    }

    /// Get the current stack size
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Check if the stack is empty
    pub fn stack_is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Get the current program counter
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Get the program length
    pub fn program_len(&self) -> usize {
        self.program.len()
    }

    /// Set the program counter
    pub fn set_pc(&mut self, pc: usize) -> AvmResult<()> {
        if pc > self.program.len() {
            return Err(AvmError::ProgramCounterOutOfBounds {
                pc,
                program_len: self.program.len(),
            });
        }
        self.pc = pc;
        Ok(())
    }

    /// Advance the program counter
    pub fn advance_pc(&mut self, offset: usize) -> AvmResult<()> {
        let new_pc = self.pc + offset;
        self.set_pc(new_pc)
    }

    /// Get the current program byte at PC
    pub fn current_byte(&self) -> AvmResult<u8> {
        if self.pc >= self.program.len() {
            return Err(AvmError::ProgramCounterOutOfBounds {
                pc: self.pc,
                program_len: self.program.len(),
            });
        }
        Ok(self.program[self.pc])
    }

    /// Read bytes from program starting at PC
    pub fn read_bytes(&self, count: usize) -> AvmResult<&[u8]> {
        if self.pc + count > self.program.len() {
            return Err(AvmError::ProgramCounterOutOfBounds {
                pc: self.pc + count,
                program_len: self.program.len(),
            });
        }
        Ok(&self.program[self.pc..self.pc + count])
    }

    /// Add to the execution cost
    pub fn add_cost(&mut self, cost: u64) -> AvmResult<()> {
        self.cost += cost;
        if self.cost > self.cost_budget {
            return Err(AvmError::CostBudgetExceeded {
                actual: self.cost,
                limit: self.cost_budget,
            });
        }
        Ok(())
    }

    /// Get the current execution cost
    pub fn cost(&self) -> u64 {
        self.cost
    }

    /// Get the cost budget
    pub fn cost_budget(&self) -> u64 {
        self.cost_budget
    }

    /// Get the TEAL version
    pub fn version(&self) -> TealVersion {
        self.version
    }

    /// Get the run mode
    pub fn run_mode(&self) -> RunMode {
        self.run_mode
    }

    /// Get the group index
    pub fn group_index(&self) -> usize {
        self.group_index
    }

    /// Get the group size
    pub fn group_size(&self) -> usize {
        self.group_size
    }

    /// Get a value from scratch space
    pub fn get_scratch(&self, index: u8) -> AvmResult<&StackValue> {
        let idx = index as usize;
        if idx >= SCRATCH_SIZE {
            return Err(AvmError::ScratchIndexOutOfBounds {
                index,
                limit: SCRATCH_SIZE as u8,
            });
        }
        Ok(&self.scratch[idx])
    }

    /// Set a value in scratch space
    pub fn set_scratch(&mut self, index: u8, value: StackValue) -> AvmResult<()> {
        let idx = index as usize;
        if idx >= SCRATCH_SIZE {
            return Err(AvmError::ScratchIndexOutOfBounds {
                index,
                limit: SCRATCH_SIZE as u8,
            });
        }
        self.scratch[idx] = value;
        Ok(())
    }

    /// Call a subroutine
    pub fn call_subroutine(&mut self, target: usize) -> AvmResult<()> {
        if self.call_stack.len() >= MAX_CALL_STACK_DEPTH {
            return Err(AvmError::CallStackOverflow {
                limit: MAX_CALL_STACK_DEPTH,
            });
        }
        self.call_stack.push(self.pc);
        self.set_pc(target)?;
        Ok(())
    }

    /// Return from a subroutine
    pub fn return_from_subroutine(&mut self) -> AvmResult<()> {
        let return_pc = self.call_stack.pop().ok_or(AvmError::CallStackUnderflow)?;
        self.set_pc(return_pc)?;
        Ok(())
    }

    /// Get the ledger access interface
    pub fn ledger(&self) -> &dyn LedgerAccess {
        self.ledger
    }

    /// Check if execution is finished
    pub fn is_finished(&self) -> bool {
        self.pc >= self.program.len()
    }
}

/// Virtual Machine for executing TEAL programs
#[derive(Debug)]
pub struct VirtualMachine {
    /// Opcode specifications
    opcodes: HashMap<u8, OpSpec>,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::with_version(TealVersion::latest())
    }
}

/// Builder for creating VirtualMachine instances with fluent API
#[derive(Debug, Default)]
pub struct VirtualMachineBuilder {
    version: Option<TealVersion>,
    custom_opcodes: Vec<OpSpec>,
}

impl VirtualMachineBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the TEAL version
    pub fn version(mut self, version: TealVersion) -> Self {
        self.version = Some(version);
        self
    }

    /// Add a custom opcode
    pub fn with_opcode(mut self, spec: OpSpec) -> Self {
        self.custom_opcodes.push(spec);
        self
    }

    /// Build the VirtualMachine
    pub fn build(self) -> VirtualMachine {
        let version = self.version.unwrap_or_default();
        let mut vm = VirtualMachine::with_version(version);

        // Add custom opcodes
        for spec in self.custom_opcodes {
            vm.register_opcode(spec.opcode, spec);
        }

        vm
    }
}

impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new() -> Self {
        Self {
            opcodes: HashMap::new(),
        }
    }

    /// Create a virtual machine with standard opcodes for a specific TEAL version
    pub fn with_version(version: TealVersion) -> Self {
        let mut vm = Self::new();
        vm.load_standard_opcodes(version);
        vm
    }

    /// Create a builder for fluent VM construction
    pub fn builder() -> VirtualMachineBuilder {
        VirtualMachineBuilder::new()
    }

    /// Load standard opcodes for a specific TEAL version
    pub fn load_standard_opcodes(&mut self, version: TealVersion) {
        for spec in get_standard_opcodes() {
            // Only register opcodes that are available in the specified version
            if Self::is_opcode_available_in_version(&spec, version) {
                self.opcodes.insert(spec.opcode, spec);
            }
        }
    }

    /// Check if an opcode is available in a specific TEAL version
    fn is_opcode_available_in_version(spec: &OpSpec, version: TealVersion) -> bool {
        // For now, we'll include all opcodes since we don't have version restrictions implemented
        // In a full implementation, this would check the minimum version for each opcode
        match spec.name.as_str() {
            // Subroutines were added in v4
            "callsub" | "retsub" => version >= TealVersion::V4,
            // Box operations were added in v8
            "box_create" | "box_extract" | "box_replace" | "box_del" | "box_len" | "box_get"
            | "box_put" => version >= TealVersion::V8,
            // Most basic opcodes are available from v1
            _ => true,
        }
    }

    /// Register an opcode specification
    pub fn register_opcode(&mut self, opcode: u8, spec: OpSpec) {
        self.opcodes.insert(opcode, spec);
    }

    /// Get the number of registered opcodes (for debugging)
    pub fn opcode_count(&self) -> usize {
        self.opcodes.len()
    }

    /// Check if an opcode is registered (for debugging)
    pub fn has_opcode(&self, opcode: u8) -> bool {
        self.opcodes.contains_key(&opcode)
    }

    /// Execute a TEAL program with automatic configuration
    pub fn execute_simple(
        &self,
        program: &[u8],
        version: TealVersion,
        ledger: &dyn LedgerAccess,
    ) -> AvmResult<bool> {
        let config = ExecutionConfig::new(version);
        self.execute(program, config, ledger)
    }

    /// Execute a TEAL program in application mode
    pub fn execute_application(
        &self,
        program: &[u8],
        version: TealVersion,
        ledger: &dyn LedgerAccess,
    ) -> AvmResult<bool> {
        let config = ExecutionConfig::application(version);
        self.execute(program, config, ledger)
    }

    /// Execute a TEAL program
    pub fn execute(
        &self,
        program: &[u8],
        config: ExecutionConfig,
        ledger: &dyn LedgerAccess,
    ) -> AvmResult<bool> {
        if program.is_empty() {
            return Err(AvmError::invalid_program("Empty program"));
        }

        let mut ctx = EvalContext::new(
            program,
            config.run_mode,
            config.cost_budget,
            config.version,
            config.group_index,
            config.group_size,
            ledger,
        );

        // Main execution loop
        while !ctx.is_finished() {
            let opcode = ctx.current_byte()?;

            // Look up opcode specification
            let spec = self.opcodes.get(&opcode).ok_or(AvmError::InvalidOpcode {
                opcode,
                pc: ctx.pc(),
            })?;

            // Check if opcode is available in this version
            if spec.min_version > config.version.as_u8() {
                return Err(AvmError::OpcodeNotAvailable {
                    version: config.version.as_u8(),
                    opcode: spec.name.clone(),
                });
            }

            // Check if opcode is allowed in this mode
            if !spec.modes.contains(&config.run_mode) {
                return Err(AvmError::invalid_program(format!(
                    "Opcode {} not allowed in {:?} mode",
                    spec.name, config.run_mode
                )));
            }

            // Add execution cost
            ctx.add_cost(spec.cost)?;

            // Add trace entry
            ctx.add_trace(format!(
                "PC:{:04} {} (cost: {})",
                ctx.pc(),
                spec.name,
                spec.cost
            ));

            // Execute the opcode
            (spec.execute)(&mut ctx)?;

            // Advance PC by 1 (opcode size) - opcodes handle their own PC advancement
        }

        // Check final result
        if ctx.stack_size() != 1 {
            let stack_size = ctx.stack_size();
            return Err(AvmError::invalid_program(format!(
                "Program ended with {stack_size} values on stack, expected 1"
            )));
        }

        let result = ctx.pop()?;
        result.as_bool()
    }
}
