# TEAL Version Requirements Analysis: avm-rs vs go-algorand

## Current avm-rs Implementation

### Version 1 Opcodes (min_version: 1)
**Arithmetic Operations:**
- `err` (0x00) - Error/panic
- `+` (0x01) - Addition
- `-` (0x02) - Subtraction
- `/` (0x03) - Division
- `*` (0x04) - Multiplication
- `<` (0x05) - Less than
- `>` (0x06) - Greater than
- `<=` (0x07) - Less than or equal
- `>=` (0x08) - Greater than or equal
- `&&` (0x09) - Logical AND
- `||` (0x0A) - Logical OR
- `==` (0x0B) - Equal
- `!=` (0x0C) - Not equal
- `!` (0x0D) - Logical NOT
- `%` (0x0E) - Modulo
- `|` (0x0F) - Bitwise OR
- `&` (0x10) - Bitwise AND
- `^` (0x11) - Bitwise XOR
- `~` (0x12) - Bitwise NOT
- `mulw` - Multiply with overflow

**Stack Operations:**
- `pop` - Remove top value
- `popn` - Remove N values
- `dupn` - Duplicate Nth value
- `dup` - Duplicate top value

**Flow Control:**
- `bnz` - Branch if not zero

**Constants:**
- `pushint` - Push immediate integer
- `pushbytes` - Push immediate bytes
- `pushbytess` - Push multiple byte arrays
- `pushints` - Push multiple integers
- `intcblock` - Integer constant block
- `intc` - Load integer constant
- `bytecblock` - Byte constant block
- `bytec` - Load byte constant
- `intc_0`, `intc_1`, `intc_2`, `intc_3` - Integer constants 0-3
- `bytec_0`, `bytec_1`, `bytec_2`, `bytec_3` - Byte constants 0-3

**Utility:**
- `len` - Length of byte string
- `itob` - Integer to bytes
- `btoi` - Bytes to integer
- `bzero` - Create zero-filled array

**Crypto:**
- `sha256` - SHA256 hash
- `keccak256` - Keccak256 hash
- `sha512_256` - SHA512_256 hash
- `sha3_256` - SHA3_256 hash
- `ed25519verify` - Ed25519 verification
- `ed25519verify_bare` - Ed25519 bare verification
- `ecdsa_verify` - ECDSA verification
- `ecdsa_pk_decompress` - ECDSA key decompression
- `ecdsa_pk_recover` - ECDSA key recovery

**Scratch Space:**
- `load` - Load from scratch
- `store` - Store to scratch

**Transaction Access:**
- `txn` - Transaction field access
- `gtxn` - Group transaction field access
- `global` - Global field access
- `txna` - Transaction array field access
- `gtxnsa` - Group transaction array field access (stack indices)
- `txnas` - Transaction array field access (stack index)

### Version 2 Opcodes (min_version: 2)
- `addw` - Add with overflow
- `dup2` - Duplicate top two values
- `concat` - Concatenate bytes
- `substring` - Extract substring
- `substring3` - Extract substring (stack args)
- `bz` - Branch if zero
- `b` - Unconditional branch
- `return` - Return from program
- `arg` - Access argument (immediate index)
- `arg_0`, `arg_1`, `arg_2`, `arg_3` - Access arguments 0-3
- `app_global_get` - Get global state
- `app_global_put` - Put global state
- `app_global_del` - Delete global state
- `app_local_get` - Get local state
- `app_local_put` - Put local state
- `app_local_del` - Delete local state
- `balance` - Get account balance
- `app_opted_in` - Check app opt-in
- `app_local_get_ex` - Get local state (extended)
- `app_global_get_ex` - Get global state (extended)
- `asset_holding_get` - Get asset holding
- `asset_params_get` - Get asset parameters
- `app_params_get` - Get app parameters
- `acct_params_get` - Get account parameters
- `gtxna` - Group transaction array field access

### Version 3 Opcodes (min_version: 3)
- `swap` - Swap top two values
- `select` - Select between values
- `min_balance` - Get minimum balance
- `gtxns` - Group transaction field (stack index)
- `getbit` - Get bit from bytes
- `setbit` - Set bit in bytes
- `getbyte` - Get byte from bytes
- `setbyte` - Set byte in bytes
- `assert` - Assert condition

### Version 4 Opcodes (min_version: 4)
- `divmodw` - Division with remainder
- `shl` - Shift left
- `shr` - Shift right
- `sqrt` - Square root
- `bitlen` - Bit length
- `exp` - Exponentiation
- `expw` - Exponentiation with overflow
- `callsub` - Call subroutine
- `retsub` - Return from subroutine
- `b+`, `b-`, `b/`, `b*` - Byte arithmetic
- `b<`, `b>`, `b<=`, `b>=`, `b==`, `b!=` - Byte comparisons
- `b%` - Byte modulo
- `b|`, `b&`, `b^`, `b~` - Byte bitwise operations

### Version 5 Opcodes (min_version: 5)
- `extract` - Extract bytes (immediate)
- `extract3` - Extract bytes (stack args)
- `extract_uint16` - Extract uint16
- `extract_uint32` - Extract uint32
- `extract_uint64` - Extract uint64
- `args` - Access arguments (stack index)
- `log` - Log event
- `itxn_begin` - Begin inner transaction
- `itxn_field` - Set inner transaction field
- `itxn_submit` - Submit inner transaction
- `itxn` - Access inner transaction field
- `itxna` - Access inner transaction array field

### Version 6 Opcodes (min_version: 6)
- `bsqrt` - Byte square root
- `divw` - Division with overflow
- `itxn_next` - Next inner transaction
- `gitxn` - Group inner transaction field access
- `gitxna` - Group inner transaction array field access
- `itxnas` - Inner transaction array field (stack index)
- `gitxnas` - Group inner transaction array field (stack index)

### Version 7 Opcodes (min_version: 7)
- `replace2` - Replace bytes (immediate start)
- `replace3` - Replace bytes (stack args)
- `base64_decode` - Base64 decode
- `json_ref` - JSON reference
- `vrf_verify` - VRF verification

### Version 8 Opcodes (min_version: 8)
- `bury` - Bury value n deep
- `dig` - Dig value n deep
- `cover` - Cover top value
- `uncover` - Uncover value
- `proto` - Function prototype
- `frame_dig` - Access function frame
- `frame_bury` - Store in function frame
- `switch` - Switch statement
- `match` - Match statement
- `box_create` - Create box
- `box_extract` - Extract from box
- `box_replace` - Replace in box
- `box_del` - Delete box
- `box_len` - Get box length
- `box_get` - Get box contents
- `box_put` - Put box contents

### Version 9 Opcodes (min_version: 9)
- `box_splice` - Splice bytes in box
- `box_resize` - Resize box

### Version 10 Opcodes (min_version: 10)
- `ec_add` - Elliptic curve point addition
- `ec_scalar_mul` - Elliptic curve scalar multiplication
- `ec_pairing_check` - Elliptic curve pairing check
- `ec_multi_scalar_mul` - Multi-scalar multiplication
- `ec_subgroup_check` - Subgroup membership check
- `ec_map_to` - Map to curve point

### Version 11 Opcodes (min_version: 11)
- `mimc` - MiMC hash function
- `block` - Blockchain randomness beacon

## Analysis Notes

**Potential Issues Identified:**

1. **Early Version Assignments**: Many opcodes in our implementation are assigned to version 1, which may not match the official go-algorand specifications.

2. **Critical Opcodes to Verify**:
   - Advanced arithmetic operations (mulw, addw, divmodw, etc.)
   - Byte operations (b+, b-, etc.)
   - Stack manipulation (bury, dig, cover, uncover)
   - Inner transaction operations
   - Box storage operations
   - Elliptic curve operations

3. **Missing Opcodes**: Need to verify if our implementation includes all opcodes present in go-algorand.

## Next Steps

To complete this analysis, I need:
1. The go-algorand OpSpecs data with accurate version requirements
2. Comparison of opcode numbers/identifiers
3. Verification of run mode restrictions
4. Cost and size parameter validation

---

**Status**: Awaiting go-algorand OpSpecs data for comprehensive comparison.