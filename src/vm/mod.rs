//! Virtual Machine implementation

use crate::error::{AvmError, AvmResult};
use crate::opcodes::OpSpec;
use crate::state::LedgerAccess;
use crate::types::{RunMode, StackValue, TealValue};
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
    pub version: u8,
    pub group_index: usize,
    pub group_size: usize,
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
    version: u8,

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
        version: u8,
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
    pub fn version(&self) -> u8 {
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

impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new() -> Self {
        Self {
            opcodes: HashMap::new(),
        }
    }

    /// Register an opcode specification
    pub fn register_opcode(&mut self, opcode: u8, spec: OpSpec) {
        self.opcodes.insert(opcode, spec);
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
            if spec.min_version > config.version {
                return Err(AvmError::OpcodeNotAvailable {
                    version: config.version,
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

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
