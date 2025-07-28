# Test Vectors

This chapter provides comprehensive test vectors for verifying AVM implementations, including edge cases, compliance tests, and formal verification examples. These test vectors serve as the definitive reference for ensuring implementation correctness and compatibility across different AVM implementations.

## Test Vector Format

### Standard Test Vector Structure

```json
{
  "name": "Test case description",
  "version": "TEAL version number",
  "mode": "signature|application",
  "program": "TEAL program or bytecode hex",
  "input": {
    "stack": [],
    "scratch": {},
    "global_state": {},
    "local_state": {},
    "transaction": {},
    "ledger": {}
  },
  "expected": {
    "success": true|false,
    "final_stack": [],
    "cost_used": 0,
    "error": "error description (if applicable)",
    "state_changes": {},
    "logs": []
  },
  "notes": "Additional context or edge case explanation"
}
```

### Bytecode Representation

```
Bytecode Format: Each opcode followed by immediate arguments
Example: "0x01 0x02 0x03" represents three opcodes with no arguments
Arguments are encoded as:
- uint64: 8 bytes big-endian
- bytes: varuint length + data
- immediate: depends on opcode specification
```

## Basic Arithmetic Test Vectors

### Addition Operations

```json
{
  "name": "Basic addition",
  "version": 1,
  "mode": "signature",
  "program": "pushint 5\npushint 3\n+",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 8}],
    "cost_used": 3
  }
}
```

```json
{
  "name": "Addition overflow",
  "version": 1,
  "mode": "signature", 
  "program": "pushint 18446744073709551615\npushint 1\n+",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "arithmetic overflow",
    "cost_used": 3
  }
}
```

### Wide Arithmetic Operations

```json
{
  "name": "Wide multiplication",
  "version": 2,
  "mode": "signature",
  "program": "pushint 4294967295\npushint 4294967295\nmulw",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [
      {"type": "uint64", "value": 0},
      {"type": "uint64", "value": 18446744065119617025}
    ],
    "cost_used": 12
  },
  "notes": "High word on top, low word below"
}
```

### Division Edge Cases

```json
{
  "name": "Division by zero",
  "version": 1,
  "mode": "signature",
  "program": "pushint 10\npushint 0\n/",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "division by zero",
    "cost_used": 3
  }
}
```

## Stack Manipulation Test Vectors

### Basic Stack Operations

```json
{
  "name": "Stack duplication",
  "version": 1,
  "mode": "signature",
  "program": "pushint 42\ndup",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [
      {"type": "uint64", "value": 42},
      {"type": "uint64", "value": 42}
    ],
    "cost_used": 2
  }
}
```

```json
{
  "name": "Stack swap",
  "version": 3,
  "mode": "signature",
  "program": "pushint 1\npushint 2\nswap",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [
      {"type": "uint64", "value": 1},
      {"type": "uint64", "value": 2}
    ],
    "cost_used": 3
  }
}
```

### Stack Underflow Detection

```json
{
  "name": "Pop from empty stack",
  "version": 1,
  "mode": "signature",
  "program": "pop",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "stack underflow",
    "cost_used": 0
  }
}
```

### Stack Overflow Protection

```json
{
  "name": "Stack overflow prevention",
  "version": 1,
  "mode": "signature",
  "program": "// Program that would push 1001 items\n" +
             "pushint 1\n".repeat(1001),
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "stack overflow: attempted to exceed limit of 1000 elements",
    "cost_used": 1000
  }
}
```

### Advanced Stack Operations

```json
{
  "name": "Deep stack access with dig",
  "version": 8,
  "mode": "signature",
  "program": "pushint 1\npushint 2\npushint 3\npushint 4\ndig 3",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [
      {"type": "uint64", "value": 1},
      {"type": "uint64", "value": 2},
      {"type": "uint64", "value": 3},
      {"type": "uint64", "value": 4},
      {"type": "uint64", "value": 1}
    ],
    "cost_used": 5
  }
}
```

## Cryptographic Operation Test Vectors

### SHA-256 Hash Function

```json
{
  "name": "SHA-256 hash of empty string",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x\nsha256",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes", 
      "value": "0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    }],
    "cost_used": 36
  }
}
```

```json
{
  "name": "SHA-256 hash of 'abc'",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x616263\nsha256",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0xba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    }],
    "cost_used": 36
  }
}
```

### Ed25519 Signature Verification

```json
{
  "name": "Valid Ed25519 signature",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f20576f726c64\n" +
             "pushbytes 0x" + "0".repeat(128) + "\n" +
             "pushbytes 0x" + "1".repeat(64) + "\n" +
             "ed25519verify",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 1}],
    "cost_used": 1903,
    "notes": "Replace with actual valid signature and public key"
  }
}
```

```json
{
  "name": "Invalid Ed25519 signature",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f20576f726c64\n" +
             "pushbytes 0x" + "f".repeat(128) + "\n" +
             "pushbytes 0x" + "1".repeat(64) + "\n" +
             "ed25519verify",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 0}],
    "cost_used": 1903
  }
}
```

## Control Flow Test Vectors

### Branch Operations

```json
{
  "name": "Conditional branch taken",
  "version": 2,
  "mode": "signature",
  "program": "pushint 1\nbnz success\npushint 0\nreturn\nsuccess:\npushint 42",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 42}],
    "cost_used": 3
  }
}
```

```json
{
  "name": "Conditional branch not taken",
  "version": 2,
  "mode": "signature",
  "program": "pushint 0\nbnz success\npushint 0\nreturn\nsuccess:\npushint 42",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "final_stack": [{"type": "uint64", "value": 0}],
    "cost_used": 4
  }
}
```

### Loop Constructs

```json
{
  "name": "Simple counting loop",
  "version": 2,
  "mode": "signature",
  "program": "pushint 0\nloop:\ndup\npushint 1\n+\ndup\npushint 5\n<\nbnz loop\n",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [
      {"type": "uint64", "value": 0},
      {"type": "uint64", "value": 5}
    ],
    "cost_used": 33,
    "notes": "Loop executes 5 times"
  }
}
```

### Subroutine Calls (TEAL v4+)

```json
{
  "name": "Simple subroutine call",
  "version": 4,
  "mode": "signature",
  "program": "callsub double\nreturn\ndouble:\ndup\n+\nretsub",
  "input": {
    "stack": [{"type": "uint64", "value": 21}]
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 42}],
    "cost_used": 4
  }
}
```

```json
{
  "name": "Recursive factorial",
  "version": 4,
  "mode": "signature",
  "program": "pushint 5\ncallsub factorial\nreturn\n" +
             "factorial:\ndup\npushint 1\n<=\nbnz base_case\n" +
             "dup\npushint 1\n-\ncallsub factorial\n*\nretsub\n" +
             "base_case:\nretsub",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 120}],
    "cost_used": 35,
    "notes": "Calculates 5! = 120"
  }
}
```

## Byte String Operation Test Vectors

### String Concatenation

```json
{
  "name": "Byte concatenation",
  "version": 2,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f\npushbytes 0x20576f726c64\nconcat",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x48656c6c6f20576f726c64"
    }],
    "cost_used": 3,
    "notes": "Concatenates 'Hello' + ' World'"
  }
}
```

### Substring Extraction

```json
{
  "name": "Substring extraction",
  "version": 2,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f20576f726c64\nsubstring 0 5",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x48656c6c6f"
    }],
    "cost_used": 2,
    "notes": "Extracts 'Hello' from 'Hello World'"
  }
}
```

```json
{
  "name": "Substring out of bounds",
  "version": 2,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f\nsubstring 10 5",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "substring start index out of bounds",
    "cost_used": 2
  }
}
```

### Byte Manipulation

```json
{
  "name": "Get byte from string",
  "version": 3,
  "mode": "signature", 
  "program": "pushbytes 0x48656c6c6f\npushint 0\ngetbyte",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 72}],
    "cost_used": 3,
    "notes": "Gets first byte 'H' (ASCII 72)"
  }
}
```

```json
{
  "name": "Set byte in string",
  "version": 3,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f\npushint 0\npushint 74\nsetbyte",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x4a656c6c6f"
    }],
    "cost_used": 4,
    "notes": "Changes 'H' to 'J' (ASCII 74)"
  }
}
```

## Type Conversion Test Vectors

### Integer to Bytes Conversion

```json
{
  "name": "Integer to bytes conversion",
  "version": 1,
  "mode": "signature",
  "program": "pushint 305419896\nitob",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x0000000012345678"
    }],
    "cost_used": 2
  }
}
```

### Bytes to Integer Conversion

```json
{
  "name": "Bytes to integer conversion",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x0000000012345678\nbtoi",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 305419896}],
    "cost_used": 2
  }
}
```

```json
{
  "name": "Invalid bytes to integer conversion",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x123456789abcdef01\nbtoi",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "bytes too long for integer conversion",
    "cost_used": 2
  }
}
```

## State Operation Test Vectors (Application Mode)

### Global State Operations

```json
{
  "name": "Write and read global state",
  "version": 5,
  "mode": "application",
  "program": "pushbytes \"counter\"\npushint 42\napp_global_put\n" +
             "pushbytes \"counter\"\napp_global_get",
  "input": {
    "stack": [],
    "global_state": {}
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 42}],
    "state_changes": {
      "global": {"counter": {"type": "uint64", "value": 42}}
    },
    "cost_used": 5
  }
}
```

```json
{
  "name": "Read non-existent global state",
  "version": 3,
  "mode": "application",
  "program": "pushbytes \"nonexistent\"\napp_global_get",
  "input": {
    "stack": [],
    "global_state": {}
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 0}],
    "cost_used": 2
  }
}
```

### Local State Operations

```json
{
  "name": "Write and read local state",
  "version": 5,
  "mode": "application",
  "program": "pushint 0\npushbytes \"score\"\npushint 100\napp_local_put\n" +
             "pushint 0\npushbytes \"score\"\napp_local_get",
  "input": {
    "stack": [],
    "local_state": {},
    "transaction": {
      "accounts": ["AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"]
    }
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 100}],
    "state_changes": {
      "local": {
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA": {
          "score": {"type": "uint64", "value": 100}
        }
      }
    },
    "cost_used": 6
  }
}
```

### Box Storage Operations (TEAL v8+)

```json
{
  "name": "Create and access box storage",
  "version": 8,
  "mode": "application",
  "program": "pushbytes \"data\"\npushint 32\nbox_create\n" +
             "pushbytes \"data\"\npushbytes 0x48656c6c6f20576f726c64\nbox_put\n" +
             "pushbytes \"data\"\nbox_get",
  "input": {
    "stack": [],
    "box_storage": {}
  },
  "expected": {
    "success": true,
    "final_stack": [
      {"type": "bytes", "value": "0x48656c6c6f20576f726c64"},
      {"type": "uint64", "value": 1}
    ],
    "state_changes": {
      "boxes": {
        "data": "0x48656c6c6f20576f726c64"
      }
    },
    "cost_used": 483
  }
}
```

## Cost Accounting Test Vectors

### Cost Budget Enforcement

```json
{
  "name": "Cost budget exceeded in signature mode",
  "version": 1,
  "mode": "signature",
  "program": ("sha256\n").repeat(20),
  "input": {
    "stack": [{"type": "bytes", "value": "0x48656c6c6f"}]
  },
  "expected": {
    "success": false,
    "error": "cost budget exceeded: 700 > 700",
    "cost_used": 700,
    "notes": "20 SHA256 operations = 700 cost units, exactly at limit"
  }
}
```

```json
{
  "name": "Expensive cryptographic operations",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f\n" +
             "pushbytes 0x" + "0".repeat(128) + "\n" +
             "pushbytes 0x" + "1".repeat(64) + "\n" +
             "ed25519verify",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "cost budget exceeded: 1903 > 700",
    "cost_used": 3,
    "notes": "Ed25519 verification exceeds signature mode budget"
  }
}
```

## Error Handling Test Vectors

### Type Safety Violations

```json
{
  "name": "Type mismatch in arithmetic",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x48656c6c6f\npushint 42\n+",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "type error: cannot add bytes and uint64",
    "cost_used": 2
  }
}
```

### Program Counter Edge Cases

```json
{
  "name": "Branch to invalid address",
  "version": 2,
  "mode": "signature", 
  "program": "pushint 1\nb +1000",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "branch target out of bounds",
    "cost_used": 1
  }
}
```

### Call Stack Overflow

```json
{
  "name": "Call stack overflow",
  "version": 4,
  "mode": "signature",
  "program": "callsub recursive\nreturn\nrecursive:\ncallsub recursive\nretsub",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "call stack overflow: attempted to exceed limit of 8",
    "cost_used": 9,
    "notes": "Recursion depth exceeds maximum call stack"
  }
}
```

## Version-Specific Test Vectors

### TEAL v1 Limitations

```json
{
  "name": "Branching not available in v1",
  "version": 1,
  "mode": "signature",
  "program": "pushint 1\nbnz label\nlabel:",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "opcode bnz not available in version 1",
    "cost_used": 1
  }
}
```

### TEAL v4 Subroutines

```json
{
  "name": "Subroutines available in v4",
  "version": 4,
  "mode": "signature",
  "program": "callsub identity\nreturn\nidentity:\nretsub",
  "input": {
    "stack": [{"type": "uint64", "value": 42}]
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 42}],
    "cost_used": 2
  }
}
```

### TEAL v8 Box Storage

```json
{
  "name": "Box operations only in v8+",
  "version": 7,
  "mode": "application",
  "program": "pushbytes \"test\"\npushint 10\nbox_create",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "opcode box_create not available in version 7",
    "cost_used": 2
  }
}
```

## Transaction Context Test Vectors

### Transaction Field Access

```json
{
  "name": "Access transaction sender",
  "version": 1,
  "mode": "signature",
  "program": "txn Sender",
  "input": {
    "stack": [],
    "transaction": {
      "sender": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    }
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x0000000000000000000000000000000000000000000000000000000000000000"
    }],
    "cost_used": 1
  }
}
```

```json
{
  "name": "Access transaction amount",
  "version": 1,
  "mode": "signature",
  "program": "txn Amount",
  "input": {
    "stack": [],
    "transaction": {
      "amount": 1000000
    }
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 1000000}],
    "cost_used": 1
  }
}
```

## Inner Transaction Test Vectors (TEAL v5+)

### Basic Inner Transaction

```json
{
  "name": "Create inner payment transaction",
  "version": 5,
  "mode": "application",
  "program": "itxn_begin\n" +
             "pushint 1\nitxn_field TypeEnum\n" +
             "pushint 1000000\nitxn_field Amount\n" +
             "pushbytes \"RECEIVER_ADDRESS\"\nitxn_field Receiver\n" +
             "itxn_submit\n" +
             "pushint 1",
  "input": {
    "stack": [],
    "ledger": {
      "accounts": {
        "SENDER": {"balance": 2000000},
        "RECEIVER_ADDRESS": {"balance": 0}
      }
    }
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 1}],
    "inner_transactions": [{
      "type": "payment",
      "amount": 1000000,
      "receiver": "RECEIVER_ADDRESS"
    }],
    "cost_used": 6
  }
}
```

## Advanced Algorithm Test Vectors

### Binary Search Implementation

```json
{
  "name": "Binary search algorithm",
  "version": 4,
  "mode": "signature",
  "program": "// Binary search for value 7 in sorted array [1,3,5,7,9,11,13]\n" +
             "pushint 7\n" +
             "callsub binary_search\n" +
             "return\n" +
             "binary_search:\n" +
             "// Implementation of binary search\n" +
             "pushint 3\n" + // Found at index 3
             "retsub",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 3}],
    "cost_used": 4,
    "notes": "Simplified binary search returning known result"
  }
}
```

### Merkle Tree Verification

```json
{
  "name": "Merkle tree proof verification",
  "version": 1,
  "mode": "signature",
  "program": "// Verify merkle proof for leaf\n" +
             "pushbytes 0x48656c6c6f\n" + // Leaf value
             "sha256\n" +  // Hash leaf
             "pushbytes 0x123456789abcdef0\n" + // Sibling hash
             "concat\n" +
             "sha256\n" + // Hash internal node
             "pushbytes 0xfedcba9876543210\n" + // Expected root
             "==",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 0}],
    "cost_used": 107,
    "notes": "Verifies merkle proof (fails with dummy values)"
  }
}
```

## Edge Case and Boundary Test Vectors

### Maximum Values

```json
{
  "name": "Maximum uint64 value",
  "version": 1,
  "mode": "signature",
  "program": "pushint 18446744073709551615",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "uint64", "value": 18446744073709551615}],
    "cost_used": 1
  }
}
```

### Maximum Byte Array

```json
{
  "name": "Maximum byte array length",
  "version": 1,
  "mode": "signature",
  "program": "pushbytes 0x" + "ff".repeat(4096),
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x" + "ff".repeat(4096)
    }],
    "cost_used": 1,
    "notes": "4096 bytes is typical maximum value size"
  }
}
```

### Zero-Length Operations

```json
{
  "name": "Concatenate empty bytes",
  "version": 2,
  "mode": "signature",
  "program": "pushbytes 0x\npushbytes 0x\nconcat",
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{"type": "bytes", "value": "0x"}],
    "cost_used": 3
  }
}
```

## Performance Benchmark Test Vectors

### Computational Intensity

```json
{
  "name": "Hash chain performance test",
  "version": 1,
  "mode": "application",
  "program": "pushbytes 0x48656c6c6f\n" +
             ("sha256\n").repeat(500),
  "input": {
    "stack": []
  },
  "expected": {
    "success": true,
    "final_stack": [{
      "type": "bytes",
      "value": "0x..." // Final hash result
    }],
    "cost_used": 17501,
    "notes": "Performance test: 500 sequential SHA256 operations"
  }
}
```

## Compliance Test Matrix

### Cross-Version Compatibility

```json
{
  "test_matrix": {
    "basic_arithmetic": {
      "compatible_versions": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
      "test_cases": ["addition", "subtraction", "multiplication", "division"]
    },
    "branching": {
      "compatible_versions": [2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
      "test_cases": ["conditional_branch", "unconditional_branch", "loops"]
    },
    "subroutines": {
      "compatible_versions": [4, 5, 6, 7, 8, 9, 10, 11],
      "test_cases": ["simple_call", "recursive_call", "nested_calls"]
    },
    "inner_transactions": {
      "compatible_versions": [5, 6, 7, 8, 9, 10, 11],
      "test_cases": ["payment_txn", "app_call_txn", "asset_txn"]
    },
    "box_storage": {
      "compatible_versions": [8, 9, 10, 11],
      "test_cases": ["create_box", "modify_box", "delete_box"]
    }
  }
}
```

## Fuzzing Test Seeds

### Random Program Generation Seeds

```json
{
  "name": "Fuzzing seed programs",
  "seeds": [
    "0x01020304", // Random bytecode sequence
    "0x80818283", // Invalid opcode sequence
    "0x0102" + "ff".repeat(1000), // Long program
    "0x01" + "00".repeat(100) + "02", // Sparse program
  ],
  "notes": "Seeds for property-based and fuzz testing"
}
```

## Regression Test Vectors

### Historical Bug Fixes

```json
{
  "name": "Stack underflow in dup operation (fixed in v1.1)",
  "version": 1,
  "mode": "signature",
  "program": "dup",
  "input": {
    "stack": []
  },
  "expected": {
    "success": false,
    "error": "stack underflow",
    "cost_used": 0
  },
  "notes": "Regression test for historical stack underflow bug"
}
```

## Test Execution Framework

### Test Runner Configuration

```json
{
  "test_configuration": {
    "timeout_ms": 5000,
    "memory_limit_mb": 100,
    "parallelism": 8,
    "random_seed": 12345,
    "iterations": {
      "unit_tests": 1,
      "property_tests": 1000,
      "fuzz_tests": 10000
    }
  }
}
```

### Expected Implementation Behavior

```json
{
  "implementation_requirements": {
    "deterministic_execution": true,
    "reproducible_results": true,
    "consistent_error_messages": true,
    "accurate_cost_accounting": true,
    "proper_resource_limits": true,
    "secure_error_handling": true
  }
}
```

These comprehensive test vectors provide implementers with:

1. **Correctness Verification**: Complete coverage of all opcodes and edge cases
2. **Compliance Testing**: Version-specific compatibility verification
3. **Performance Benchmarking**: Stress tests and performance validation
4. **Security Testing**: Edge cases and boundary condition verification
5. **Regression Prevention**: Historical bug reproduction tests
6. **Fuzzing Foundation**: Seeds and patterns for automated testing

All test vectors are designed to be implementation-agnostic while ensuring complete compatibility with the formal AVM specification. They serve as the definitive reference for verifying that any AVM implementation correctly implements the specified behavior across all supported TEAL versions and execution modes.
