//! Property-based tests for stack invariants using quickcheck

use avm_rs::{
    opcodes::*,
    state::MockLedger,
    types::{StackValue, TealVersion},
    vm::{EvalContext, ExecutionConfig, MAX_STACK_SIZE, VirtualMachine},
};
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

// Wrapper type for StackValue to implement Arbitrary
#[derive(Debug, Clone)]
struct TestStackValue(StackValue);

impl Arbitrary for TestStackValue {
    fn arbitrary(g: &mut Gen) -> Self {
        if bool::arbitrary(g) {
            TestStackValue(StackValue::Uint(u64::arbitrary(g)))
        } else {
            let len = usize::arbitrary(g) % 100; // Reasonable byte array sizes
            let bytes: Vec<u8> = (0..len).map(|_| u8::arbitrary(g)).collect();
            TestStackValue(StackValue::Bytes(bytes))
        }
    }
}

fn create_test_context() -> (VirtualMachine, EvalContext<'static>, Box<MockLedger>) {
    let vm = VirtualMachine::with_version(TealVersion::V11);
    let mut ledger = Box::new(MockLedger::new());
    let ledger_ptr = &mut *ledger as *mut dyn avm_rs::state::LedgerAccess;

    // Create a minimal program that won't interfere with our tests
    let program = vec![OP_RETURN];
    let config = ExecutionConfig::new(TealVersion::V11).with_cost_budget(1_000_000);

    let ctx = unsafe {
        EvalContext::new(
            Box::leak(Box::new(program.clone())),
            config.run_mode,
            config.cost_budget,
            config.version,
            config.group_index,
            config.group_size,
            &mut *ledger_ptr,
            #[cfg(feature = "tracing")]
            config.tracing.clone(),
        )
    };

    (vm, ctx, ledger)
}

#[quickcheck]
fn prop_stack_never_exceeds_max_size(values: Vec<TestStackValue>) -> bool {
    let (_vm, mut ctx, _ledger) = create_test_context();

    // Try to push many values and ensure stack size is bounded
    for TestStackValue(value) in values {
        let _ = ctx.push(value);
        // Invariant: stack size is always within bounds
        if ctx.stack_size() > MAX_STACK_SIZE {
            return false;
        }
    }

    true
}

#[quickcheck]
fn prop_push_pop_symmetry(value: TestStackValue) -> bool {
    let (_vm, mut ctx, _ledger) = create_test_context();

    let initial_size = ctx.stack_size();
    let TestStackValue(stack_value) = value;

    // Push then pop should preserve stack state
    if ctx.push(stack_value.clone()).is_ok() {
        let popped = ctx.pop();
        match popped {
            Ok(v) => v == stack_value && ctx.stack_size() == initial_size,
            Err(_) => false,
        }
    } else {
        // If push fails (stack overflow), that's OK for this test
        true
    }
}

#[quickcheck]
fn prop_dup_increases_size(initial_values: Vec<TestStackValue>) -> bool {
    use avm_rs::opcodes::stack::*;
    let (_vm, mut ctx, _ledger) = create_test_context();

    // Setup initial stack
    for TestStackValue(value) in initial_values {
        if ctx.push(value).is_err() {
            return true; // Stack overflow is OK
        }
    }

    if ctx.stack_size() == 0 {
        // DUP on empty stack should fail
        return op_dup(&mut ctx).is_err();
    }

    let initial_size = ctx.stack_size();
    let top_value = ctx.peek().unwrap().clone();

    match op_dup(&mut ctx) {
        Ok(_) => {
            // Size should increase by 1
            ctx.stack_size() == initial_size + 1 &&
            // Top value should be unchanged
            ctx.peek().unwrap() == &top_value
        }
        Err(_) => {
            // Should only fail if we hit stack limit
            initial_size == MAX_STACK_SIZE
        }
    }
}

#[quickcheck]
fn prop_swap_preserves_size(values: Vec<TestStackValue>) -> bool {
    use avm_rs::opcodes::stack::*;
    let (_vm, mut ctx, _ledger) = create_test_context();

    // Need at least 2 values for swap
    if values.len() < 2 {
        return true;
    }

    for TestStackValue(value) in values {
        if ctx.push(value).is_err() {
            return true; // Stack overflow is OK
        }
    }

    let initial_size = ctx.stack_size();
    if initial_size < 2 {
        // SWAP on stack with < 2 elements should fail
        return op_swap(&mut ctx).is_err();
    }

    // Get top two values before swap
    let top = ctx.pop().unwrap();
    let second = ctx.pop().unwrap();
    ctx.push(second.clone()).unwrap();
    ctx.push(top.clone()).unwrap();

    match op_swap(&mut ctx) {
        Ok(_) => {
            // Size should be unchanged
            if ctx.stack_size() != initial_size {
                return false;
            }
            // Values should be swapped
            let new_top = ctx.pop().unwrap();
            let new_second = ctx.pop().unwrap();
            new_top == second && new_second == top
        }
        Err(_) => false, // SWAP should not fail with >= 2 elements
    }
}

#[quickcheck]
fn prop_stack_underflow_safety(_unused: u8) -> bool {
    use avm_rs::opcodes::stack::*;
    let (_vm, mut ctx, _ledger) = create_test_context();

    // Empty stack - operations requiring stack elements should fail
    op_pop(&mut ctx).is_err()
        && op_dup(&mut ctx).is_err()
        && op_dup2(&mut ctx).is_err()
        && op_swap(&mut ctx).is_err()
}

#[quickcheck]
fn prop_dup2_behavior(initial_values: Vec<TestStackValue>) -> bool {
    use avm_rs::opcodes::stack::*;
    let (_vm, mut ctx, _ledger) = create_test_context();

    // Setup initial stack
    for TestStackValue(value) in initial_values {
        if ctx.push(value).is_err() {
            return true; // Stack overflow is OK
        }
    }

    if ctx.stack_size() < 2 {
        // DUP2 on stack with < 2 elements should fail
        return op_dup2(&mut ctx).is_err();
    }

    let initial_size = ctx.stack_size();

    // Get top two values before dup2
    let top = ctx.peek_at_depth(0).unwrap().clone();
    let second = ctx.peek_at_depth(1).unwrap().clone();

    match op_dup2(&mut ctx) {
        Ok(_) => {
            // Size should increase by 2
            if ctx.stack_size() != initial_size + 2 {
                return false;
            }
            // Check the TEAL spec: [A, B] -> [A, B, A, B]
            // Top should be B, second should be A
            ctx.peek_at_depth(0).unwrap() == &top && ctx.peek_at_depth(1).unwrap() == &second
        }
        Err(_) => {
            // Should only fail if we hit stack limit
            initial_size + 2 > MAX_STACK_SIZE
        }
    }
}

#[quickcheck]
fn prop_stack_overflow_safety(extra_pushes: Vec<TestStackValue>) -> bool {
    let (_vm, mut ctx, _ledger) = create_test_context();

    // Fill stack to near capacity
    for i in 0..MAX_STACK_SIZE - 1 {
        if ctx.push(StackValue::Uint(i as u64)).is_err() {
            return false; // Should not fail before reaching limit
        }
    }

    // Try to push more values
    for TestStackValue(value) in extra_pushes {
        match ctx.push(value) {
            Ok(_) => {
                if ctx.stack_size() > MAX_STACK_SIZE {
                    return false; // Should never exceed MAX_STACK_SIZE
                }
            }
            Err(_) => {
                // Should fail when at capacity
                if ctx.stack_size() != MAX_STACK_SIZE {
                    return false;
                }
            }
        }
    }

    true
}
