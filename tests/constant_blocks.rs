use rust_avm::assembler::Assembler;
use rust_avm::varuint::encode_varuint;

#[test]
fn test_intcblock_single_constant_varuint() {
    let mut assembler = Assembler::new();
    let teal_program = r#"
#pragma version 6
intcblock 42
"#;
    
    let bytecode = assembler.assemble(teal_program).expect("Assembly failed");
    
    // Expected: OP_INTCBLOCK (0x20) + count varuint (1) + value varuint (42)
    let mut expected = vec![0x20]; // OP_INTCBLOCK
    expected.extend_from_slice(&encode_varuint(1)); // count = 1
    expected.extend_from_slice(&encode_varuint(42)); // value = 42
    
    assert_eq!(bytecode, expected);
}

#[test]
fn test_intcblock_multiple_constants_varuint() {
    let mut assembler = Assembler::new();
    let teal_program = r#"
#pragma version 6
intcblock 10 20 300 0x100 0xFF
"#;
    
    let bytecode = assembler.assemble(teal_program).expect("Assembly failed");
    
    // Expected: OP_INTCBLOCK + count varuint (5) + five varuint values
    let mut expected = vec![0x20]; // OP_INTCBLOCK
    expected.extend_from_slice(&encode_varuint(5)); // count = 5
    expected.extend_from_slice(&encode_varuint(10)); // 10
    expected.extend_from_slice(&encode_varuint(20)); // 20
    expected.extend_from_slice(&encode_varuint(300)); // 300
    expected.extend_from_slice(&encode_varuint(256)); // 0x100 = 256
    expected.extend_from_slice(&encode_varuint(255)); // 0xFF = 255
    
    assert_eq!(bytecode, expected);
}

#[test]
fn test_bytecblock_single_constant_varuint() {
    let mut assembler = Assembler::new();
    let teal_program = r#"
#pragma version 6
bytecblock "hello"
"#;
    
    let bytecode = assembler.assemble(teal_program).expect("Assembly failed");
    
    // Expected: OP_BYTECBLOCK + count varuint (1) + length varuint (5) + "hello"
    let mut expected = vec![0x26]; // OP_BYTECBLOCK
    expected.extend_from_slice(&encode_varuint(1)); // count = 1
    expected.extend_from_slice(&encode_varuint(5)); // length = 5
    expected.extend_from_slice(b"hello"); // "hello"
    
    assert_eq!(bytecode, expected);
}

#[test]
fn test_bytecblock_multiple_constants_varuint() {
    let mut assembler = Assembler::new();
    let teal_program = r#"
#pragma version 6
bytecblock "hi" 0x42 "test"
"#;
    
    let bytecode = assembler.assemble(teal_program).expect("Assembly failed");
    
    // Expected: OP_BYTECBLOCK + count varuint (3) + three length-prefixed byte arrays
    let mut expected = vec![0x26]; // OP_BYTECBLOCK
    expected.extend_from_slice(&encode_varuint(3)); // count = 3
    
    // "hi"
    expected.extend_from_slice(&encode_varuint(2)); // length = 2
    expected.extend_from_slice(b"hi");
    
    // 0x42
    expected.extend_from_slice(&encode_varuint(1)); // length = 1
    expected.push(0x42);
    
    // "test"
    expected.extend_from_slice(&encode_varuint(4)); // length = 4
    expected.extend_from_slice(b"test");
    
    assert_eq!(bytecode, expected);
}

#[test]
fn test_varuint_large_values() {
    let mut assembler = Assembler::new();
    let teal_program = r#"
#pragma version 6
intcblock 127 128 16383 16384
"#;
    
    let bytecode = assembler.assemble(teal_program).expect("Assembly failed");
    
    // Test that large values are properly encoded as varuint
    let mut expected = vec![0x20]; // OP_INTCBLOCK
    expected.extend_from_slice(&encode_varuint(4)); // count = 4
    expected.extend_from_slice(&encode_varuint(127)); // Single byte: 0x7F
    expected.extend_from_slice(&encode_varuint(128)); // Two bytes: 0x80, 0x01
    expected.extend_from_slice(&encode_varuint(16383)); // Two bytes: 0xFF, 0x7F
    expected.extend_from_slice(&encode_varuint(16384)); // Three bytes: 0x80, 0x80, 0x01
    
    assert_eq!(bytecode, expected);
}

#[test]
fn test_disassemble_intcblock_varuint() {
    use rust_avm::assembler::disassemble;
    
    let mut bytecode = vec![0x20]; // OP_INTCBLOCK
    bytecode.extend_from_slice(&encode_varuint(2)); // count = 2
    bytecode.extend_from_slice(&encode_varuint(123)); // 123
    bytecode.extend_from_slice(&encode_varuint(456)); // 456
    
    let result = disassemble(&bytecode).expect("Disassembly failed");
    assert!(result.contains("intcblock 123 456"));
}

#[test]
fn test_disassemble_bytecblock_varuint() {
    use rust_avm::assembler::disassemble;
    
    let mut bytecode = vec![0x26]; // OP_BYTECBLOCK
    bytecode.extend_from_slice(&encode_varuint(2)); // count = 2
    
    // "hi"
    bytecode.extend_from_slice(&encode_varuint(2)); // length = 2
    bytecode.extend_from_slice(b"hi");
    
    // hex bytes
    bytecode.extend_from_slice(&encode_varuint(2)); // length = 2
    bytecode.extend_from_slice(&[0xDE, 0xAD]);
    
    let result = disassemble(&bytecode).expect("Disassembly failed");
    assert!(result.contains("bytecblock \"hi\" 0xdead"));
}

#[test]
fn test_constants_used_in_program() {
    use rust_avm::assembler::Assembler;
    use rust_avm::state::MockLedger;
    use rust_avm::vm::VirtualMachine;
    use rust_avm::types::TealVersion;
    
    let mut assembler = Assembler::new();
    let teal_program = r#"
#pragma version 6
intcblock 100 200 300
bytecblock "hello" "world"
intc_0      // Push 100
intc 1      // Push 200
+           // Add them (should be 300)
intc_2      // Push 300
==          // Should be true (1 on stack)
bytec_0     // Push "hello"
len         // Should be 5
bytec 1     // Push "world"
len         // Should be 5
+           // Add lengths (should be 10)
pushint 10  // Push 10 to compare
==          // Compare 10 == 10 (true)
&&          // AND the two boolean results
return
"#;
    
    let bytecode = assembler.assemble(teal_program).expect("Assembly failed");
    
    // Create VM and execute
    let vm = VirtualMachine::with_version(TealVersion::V6);
    let mut ledger = MockLedger::new();
    let config = rust_avm::vm::ExecutionConfig::new(TealVersion::V6).with_cost_budget(1000);
    
    let result = vm.execute(&bytecode, config, &mut ledger).expect("Execution failed");
    assert!(result, "Program should return true");
}

#[test]
fn test_large_constant_counts() {
    let mut assembler = Assembler::new();
    
    // Test with many constants (>127 to test varuint multi-byte encoding)
    let mut constants = Vec::new();
    for i in 0..200 {
        constants.push(i.to_string());
    }
    
    let teal_program = format!(
        "#pragma version 6\nintcblock {}\nintc_0\nreturn",
        constants.join(" ")
    );
    
    let bytecode = assembler.assemble(&teal_program).expect("Assembly failed");
    
    // Verify the count is properly encoded as varuint
    // Should start with OP_INTCBLOCK (0x20) followed by varuint 200
    assert_eq!(bytecode[0], 0x20);
    
    let count_bytes = encode_varuint(200);
    assert_eq!(&bytecode[1..1 + count_bytes.len()], &count_bytes);
}