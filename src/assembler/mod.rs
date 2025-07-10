//! TEAL assembler implementation

use crate::error::{AvmError, AvmResult};
use crate::opcodes::*;
use crate::varuint::encode_varuint;
use std::collections::HashMap;

/// TEAL assembler
#[derive(Debug, Default)]
pub struct Assembler {
    /// Program version
    version: u8,
    /// Type tracking enabled
    typetrack: bool,
    /// Label to address mapping
    labels: HashMap<String, usize>,
    /// Forward label references to resolve
    forward_refs: Vec<(usize, String)>,
}

impl Assembler {
    /// Create a new assembler
    pub fn new() -> Self {
        Self::default()
    }

    /// Assemble TEAL source code to bytecode
    pub fn assemble(&mut self, source: &str) -> AvmResult<Vec<u8>> {
        let mut bytecode = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // First pass: collect labels and generate bytecode
        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") || line.starts_with(";") {
                continue;
            }

            // Handle inline comments (remove everything after ; or //)
            let line = if let Some(pos) = line.find(';') {
                line[..pos].trim()
            } else if let Some(pos) = line.find("//") {
                line[..pos].trim()
            } else {
                line
            };

            // Skip if line becomes empty after comment removal
            if line.is_empty() {
                continue;
            }

            // Handle pragma directives
            if line.starts_with("#pragma") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    match parts[1] {
                        "version" => {
                            self.version = parts[2].parse().map_err(|_| {
                                AvmError::assembly_error(format!(
                                    "Invalid version on line {}",
                                    line_num + 1
                                ))
                            })?;
                        }
                        "typetrack" => {
                            self.typetrack = parts[2].parse().map_err(|_| {
                                AvmError::assembly_error(format!(
                                    "Invalid typetrack value on line {}",
                                    line_num + 1
                                ))
                            })?;
                        }
                        _ => {
                            return Err(AvmError::assembly_error(format!(
                                "Unknown pragma directive '{}' on line {}",
                                parts[1],
                                line_num + 1
                            )));
                        }
                    }
                } else {
                    return Err(AvmError::assembly_error(format!(
                        "Invalid pragma syntax on line {}",
                        line_num + 1
                    )));
                }
                continue;
            }

            // Handle labels
            if line.ends_with(':') {
                let label = line.strip_suffix(':').unwrap();
                self.labels.insert(label.to_string(), bytecode.len());
                continue;
            }

            // Parse instruction
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let opcode = parts[0];
            let args = &parts[1..];

            self.assemble_instruction(&mut bytecode, opcode, args, line_num + 1)?;
        }

        // Second pass: resolve forward references
        self.resolve_forward_refs(&mut bytecode)?;

        Ok(bytecode)
    }

    /// Assemble a single instruction
    fn assemble_instruction(
        &mut self,
        bytecode: &mut Vec<u8>,
        opcode: &str,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        match opcode {
            // Arithmetic operations
            "+" => bytecode.push(OP_PLUS),
            "-" => bytecode.push(OP_MINUS),
            "*" => bytecode.push(OP_MUL),
            "/" => bytecode.push(OP_DIV),
            "%" => bytecode.push(OP_MOD),
            "<" => bytecode.push(OP_LT),
            ">" => bytecode.push(OP_GT),
            "<=" => bytecode.push(OP_LE),
            ">=" => bytecode.push(OP_GE),
            "==" => bytecode.push(OP_EQ),
            "!=" => bytecode.push(OP_NE),
            "&&" => bytecode.push(OP_AND),
            "||" => bytecode.push(OP_OR),
            "!" => bytecode.push(OP_NOT),
            "|" => bytecode.push(OP_BITWISE_OR),
            "&" => bytecode.push(OP_BITWISE_AND),
            "^" => bytecode.push(OP_BITWISE_XOR),
            "~" => bytecode.push(OP_BITWISE_NOT),

            // Stack operations
            "pop" => bytecode.push(OP_POP),
            "dup" => bytecode.push(OP_DUP),
            "dup2" => bytecode.push(OP_DUP2),
            "dig" => {
                bytecode.push(OP_DIG);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "bury" => {
                bytecode.push(OP_BURY);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "cover" => {
                bytecode.push(OP_COVER);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "uncover" => {
                bytecode.push(OP_UNCOVER);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "swap" => bytecode.push(OP_SWAP),
            "select" => bytecode.push(OP_SELECT),
            "dupn" => {
                bytecode.push(OP_DUPN);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "popn" => {
                bytecode.push(OP_POPN);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }

            // Flow control
            "bnz" => {
                bytecode.push(OP_BNZ);
                self.assemble_branch_target(bytecode, args, line_num)?;
            }
            "bz" => {
                bytecode.push(OP_BZ);
                self.assemble_branch_target(bytecode, args, line_num)?;
            }
            "b" => {
                bytecode.push(OP_B);
                self.assemble_branch_target(bytecode, args, line_num)?;
            }
            "return" => bytecode.push(OP_RETURN),
            "assert" => bytecode.push(OP_ASSERT),
            "callsub" => {
                bytecode.push(OP_CALLSUB);
                self.assemble_branch_target(bytecode, args, line_num)?;
            }
            "retsub" => bytecode.push(OP_RETSUB),
            "proto" => {
                bytecode.push(OP_PROTO);
                // Proto takes 2 byte immediates: args and returns
                if args.len() < 2 {
                    return Err(AvmError::assembly_error(format!(
                        "proto requires args and returns count on line {line_num}"
                    )));
                }
                self.assemble_byte_immediate(bytecode, &[args[0]], line_num)?;
                self.assemble_byte_immediate(bytecode, &[args[1]], line_num)?;
            }
            "frame_dig" => {
                bytecode.push(OP_FRAME_DIG);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "frame_bury" => {
                bytecode.push(OP_FRAME_BURY);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "switch" => {
                bytecode.push(OP_SWITCH);
                self.assemble_branch_target(bytecode, args, line_num)?;
            }
            "match" => {
                bytecode.push(OP_MATCH);
                self.assemble_branch_target(bytecode, args, line_num)?;
            }

            // Constants (high-level syntax)
            "int" => {
                bytecode.push(OP_PUSHINT);
                self.assemble_int_immediate(bytecode, args, line_num)?;
            }
            "byte" => {
                bytecode.push(OP_PUSHBYTES);
                self.assemble_bytes_immediate(bytecode, args, line_num)?;
            }
            "addr" => {
                bytecode.push(OP_PUSHBYTES);
                self.assemble_addr_immediate(bytecode, args, line_num)?;
            }
            "method" => {
                bytecode.push(OP_PUSHBYTES);
                self.assemble_method_immediate(bytecode, args, line_num)?;
            }
            // Low-level opcodes (for direct use if needed)
            "pushint" => {
                bytecode.push(OP_PUSHINT);
                self.assemble_int_immediate(bytecode, args, line_num)?;
            }
            "pushbytes" => {
                bytecode.push(OP_PUSHBYTES);
                self.assemble_bytes_immediate(bytecode, args, line_num)?;
            }

            // Constant block opcodes
            "intcblock" => {
                bytecode.push(OP_INTCBLOCK);
                self.assemble_intcblock(bytecode, args, line_num)?;
            }
            "intc" => {
                bytecode.push(OP_INTC);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "intc_0" => bytecode.push(OP_INTC_0),
            "intc_1" => bytecode.push(OP_INTC_1),
            "intc_2" => bytecode.push(OP_INTC_2),
            "intc_3" => bytecode.push(OP_INTC_3),
            "bytecblock" => {
                bytecode.push(OP_BYTECBLOCK);
                self.assemble_bytecblock(bytecode, args, line_num)?;
            }
            "bytec" => {
                bytecode.push(OP_BYTEC);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "bytec_0" => bytecode.push(OP_BYTEC_0),
            "bytec_1" => bytecode.push(OP_BYTEC_1),
            "bytec_2" => bytecode.push(OP_BYTEC_2),
            "bytec_3" => bytecode.push(OP_BYTEC_3),

            // Arguments
            "arg" => {
                bytecode.push(OP_ARG);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "arg_0" => bytecode.push(OP_ARG_0),
            "arg_1" => bytecode.push(OP_ARG_1),
            "arg_2" => bytecode.push(OP_ARG_2),
            "arg_3" => bytecode.push(OP_ARG_3),

            // Utility
            "len" => bytecode.push(OP_LEN),
            "itob" => bytecode.push(OP_ITOB),
            "btoi" => bytecode.push(OP_BTOI),
            "concat" => bytecode.push(OP_CONCAT),
            "substring" => {
                bytecode.push(OP_SUBSTRING);
                self.assemble_substring_args(bytecode, args, line_num)?;
            }
            "substring3" => bytecode.push(OP_SUBSTRING3),
            "getbit" => bytecode.push(OP_GETBIT),
            "setbit" => bytecode.push(OP_SETBIT),
            "getbyte" => bytecode.push(OP_GETBYTE),
            "setbyte" => bytecode.push(OP_SETBYTE),
            "extract" => {
                bytecode.push(OP_EXTRACT);
                self.assemble_substring_args(bytecode, args, line_num)?;
            }
            "extract3" => bytecode.push(OP_EXTRACT3),
            "extract_uint16" => {
                bytecode.push(OP_EXTRACT_UINT16);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "extract_uint32" => {
                bytecode.push(OP_EXTRACT_UINT32);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "extract_uint64" => {
                bytecode.push(OP_EXTRACT_UINT64);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "replace2" => {
                bytecode.push(OP_REPLACE2);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "replace3" => bytecode.push(OP_REPLACE3),
            "base64_decode" => {
                bytecode.push(OP_BASE64_DECODE);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "json_ref" => {
                bytecode.push(OP_JSON_REF);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }

            // Crypto
            "sha256" => bytecode.push(OP_SHA256),
            "keccak256" => bytecode.push(OP_KECCAK256),
            "sha512_256" => bytecode.push(OP_SHA512_256),
            "sha3_256" => bytecode.push(OP_SHA3_256),
            "ed25519verify" => bytecode.push(OP_ED25519VERIFY),
            "ed25519verify_bare" => bytecode.push(OP_ED25519VERIFY_BARE),
            "vrf_verify" => bytecode.push(OP_VRF_VERIFY),

            // Scratch space
            "load" => {
                bytecode.push(OP_LOAD);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }
            "store" => {
                bytecode.push(OP_STORE);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }

            // Transaction fields
            "txn" => {
                bytecode.push(OP_TXN);
                self.assemble_txn_field(bytecode, args, line_num)?;
            }
            "gtxn" => {
                bytecode.push(OP_GTXN);
                self.assemble_gtxn_args(bytecode, args, line_num)?;
            }
            "global" => {
                bytecode.push(OP_GLOBAL);
                self.assemble_global_field(bytecode, args, line_num)?;
            }

            // Application state
            "app_opted_in" => bytecode.push(OP_APP_OPTED_IN),
            "app_local_get" => bytecode.push(OP_APP_LOCAL_GET),
            "app_local_get_ex" => bytecode.push(OP_APP_LOCAL_GET_EX),
            "app_global_get" => bytecode.push(OP_APP_GLOBAL_GET),
            "app_global_get_ex" => bytecode.push(OP_APP_GLOBAL_GET_EX),
            "app_local_put" => bytecode.push(OP_APP_LOCAL_PUT),
            "app_global_put" => bytecode.push(OP_APP_GLOBAL_PUT),
            "app_local_del" => bytecode.push(OP_APP_LOCAL_DEL),
            "app_global_del" => bytecode.push(OP_APP_GLOBAL_DEL),
            "asset_holding_get" => bytecode.push(OP_ASSET_HOLDING_GET),
            "asset_params_get" => bytecode.push(OP_ASSET_PARAMS_GET),
            "app_params_get" => bytecode.push(OP_APP_PARAMS_GET),
            "acct_params_get" => bytecode.push(OP_ACCT_PARAMS_GET),
            "balance" => bytecode.push(OP_BALANCE),
            "min_balance" => bytecode.push(OP_MIN_BALANCE),

            // Box operations (v8+)
            "box_create" => bytecode.push(OP_BOX_CREATE),
            "box_extract" => bytecode.push(OP_BOX_EXTRACT),
            "box_replace" => bytecode.push(OP_BOX_REPLACE),
            "box_del" => bytecode.push(OP_BOX_DEL),
            "box_len" => bytecode.push(OP_BOX_LEN),
            "box_get" => bytecode.push(OP_BOX_GET),
            "box_put" => bytecode.push(OP_BOX_PUT),
            "box_splice" => bytecode.push(OP_BOX_SPLICE),
            "box_resize" => bytecode.push(OP_BOX_RESIZE),

            // Block operations
            "block" => {
                bytecode.push(OP_BLOCK);
                self.assemble_byte_immediate(bytecode, args, line_num)?;
            }

            "err" => bytecode.push(OP_ERR),

            _ => {
                return Err(AvmError::assembly_error(format!(
                    "Unknown opcode '{opcode}' on line {line_num}"
                )));
            }
        }

        Ok(())
    }

    /// Assemble branch target (may be forward reference)
    fn assemble_branch_target(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing branch target on line {line_num}"
            )));
        }

        let target = args[0];

        // Check if it's a label
        if let Some(&addr) = self.labels.get(target) {
            // The offset is calculated from the PC position when the branch executes
            // At that point, PC has advanced past the entire instruction (opcode + 2 bytes)
            // So the target calculation is: target_addr = (pc_after_instruction) + offset
            // Therefore: offset = target_addr - pc_after_instruction
            let pc_after_instruction = bytecode.len() + 2; // bytecode.len() is where offset goes, +2 gets past it
            let offset = (addr as i32) - (pc_after_instruction as i32);
            bytecode.extend_from_slice(&(offset as i16).to_be_bytes());
        } else {
            // Forward reference - add placeholder and record for later resolution
            self.forward_refs.push((bytecode.len(), target.to_string()));
            bytecode.extend_from_slice(&[0, 0]); // Placeholder
        }

        Ok(())
    }

    /// Assemble integer immediate value
    fn assemble_int_immediate(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing integer value on line {line_num}"
            )));
        }

        let value = self.parse_integer(args[0], line_num)?;
        bytecode.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }

    /// Parse integer from various formats (decimal, hex, octal, binary)
    fn parse_integer(&self, input: &str, line_num: usize) -> AvmResult<u64> {
        let value = if input.starts_with("0x") || input.starts_with("0X") {
            // Hexadecimal
            u64::from_str_radix(&input[2..], 16)
        } else if input.starts_with("0o") || input.starts_with("0O") {
            // Octal
            u64::from_str_radix(&input[2..], 8)
        } else if input.starts_with("0b") || input.starts_with("0B") {
            // Binary
            u64::from_str_radix(&input[2..], 2)
        } else {
            // Decimal
            input.parse::<u64>()
        };

        value.map_err(|_| {
            AvmError::assembly_error(format!("Invalid integer '{input}' on line {line_num}"))
        })
    }

    /// Assemble byte immediate value
    fn assemble_byte_immediate(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing byte value on line {line_num}"
            )));
        }

        let value: u8 = args[0].parse().map_err(|_| {
            AvmError::assembly_error(format!(
                "Invalid byte value '{}' on line {}",
                args[0], line_num
            ))
        })?;

        bytecode.push(value);
        Ok(())
    }

    /// Assemble bytes immediate value
    fn assemble_bytes_immediate(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing bytes value on line {line_num}"
            )));
        }

        let bytes = self.parse_bytes(args, line_num)?;

        if bytes.len() > 255 {
            return Err(AvmError::assembly_error(format!(
                "Bytes too long ({} > 255) on line {}",
                bytes.len(),
                line_num
            )));
        }

        bytecode.push(bytes.len() as u8);
        bytecode.extend_from_slice(&bytes);
        Ok(())
    }

    /// Parse bytes from various formats
    fn parse_bytes(&self, args: &[&str], line_num: usize) -> AvmResult<Vec<u8>> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing bytes value on line {line_num}"
            )));
        }

        // Handle different byte formats
        if args.len() >= 2 && (args[0] == "base64" || args[0] == "b64") {
            // base64 format: byte base64 AAAA... or byte b64 AAAA...
            let b64_data = args[1..].join("");
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD
                .decode(b64_data)
                .map_err(|_| AvmError::assembly_error(format!("Invalid base64 on line {line_num}")))
        } else if args.len() == 1 {
            let arg = args[0];
            if let Some(stripped) = arg.strip_prefix("0x") {
                // Hex format: byte 0x1234...
                hex::decode(stripped).map_err(|_| {
                    AvmError::assembly_error(format!(
                        "Invalid hex bytes '{arg}' on line {line_num}"
                    ))
                })
            } else if arg.starts_with('"') && arg.ends_with('"') {
                // String literal: byte "hello"
                let content = &arg[1..arg.len() - 1];
                Ok(self.parse_string_literal(content)?)
            } else {
                // Try to parse as base32 (Algorand address)
                self.try_parse_base32(arg, line_num)
            }
        } else {
            Err(AvmError::assembly_error(format!(
                "Invalid bytes format on line {line_num}"
            )))
        }
    }

    /// Parse string literal with escape sequences
    fn parse_string_literal(&self, content: &str) -> AvmResult<Vec<u8>> {
        let mut result = Vec::new();
        let mut chars = content.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push(b'\n'),
                    Some('t') => result.push(b'\t'),
                    Some('r') => result.push(b'\r'),
                    Some('\\') => result.push(b'\\'),
                    Some('"') => result.push(b'"'),
                    Some('x') => {
                        // Hex escape: \x41
                        let c1 = chars.next();
                        let c2 = chars.next();
                        if let (Some(c1), Some(c2)) = (c1, c2) {
                            let hex = format!("{c1}{c2}");
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                result.push(byte);
                            } else {
                                return Err(AvmError::assembly_error(
                                    "Invalid hex escape sequence".to_string(),
                                ));
                            }
                        } else {
                            return Err(AvmError::assembly_error(
                                "Invalid hex escape sequence".to_string(),
                            ));
                        }
                    }
                    Some(c) => result.push(c as u8),
                    None => {
                        return Err(AvmError::assembly_error(
                            "Incomplete escape sequence".to_string(),
                        ));
                    }
                }
            } else {
                result.push(ch as u8);
            }
        }

        Ok(result)
    }

    /// Try to parse as base32 (Algorand address format)
    fn try_parse_base32(&self, input: &str, line_num: usize) -> AvmResult<Vec<u8>> {
        use base32::{Alphabet, decode};
        
        // Algorand uses a specific base32 alphabet (RFC4648 without padding)
        match decode(Alphabet::Rfc4648 { padding: false }, input) {
            Some(bytes) => {
                // Algorand addresses should be 32 bytes after decoding
                if bytes.len() == 32 {
                    Ok(bytes)
                } else {
                    Err(AvmError::assembly_error(format!(
                        "Invalid address length: expected 32 bytes, got {} on line {line_num}",
                        bytes.len()
                    )))
                }
            }
            None => Err(AvmError::assembly_error(format!(
                "Invalid base32 encoding in address '{input}' on line {line_num}"
            )))
        }
    }

    /// Assemble address immediate value
    fn assemble_addr_immediate(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing address value on line {line_num}"
            )));
        }

        // Parse Algorand address (base32) and convert to bytes
        let addr_bytes = self.parse_algorand_address(args[0], line_num)?;

        if addr_bytes.len() > 255 {
            return Err(AvmError::assembly_error(format!(
                "Address too long ({} > 255) on line {}",
                addr_bytes.len(),
                line_num
            )));
        }

        bytecode.push(addr_bytes.len() as u8);
        bytecode.extend_from_slice(&addr_bytes);
        Ok(())
    }

    /// Assemble method selector immediate value
    fn assemble_method_immediate(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing method signature on line {line_num}"
            )));
        }

        // Join all args to form method signature, then compute selector
        let method_sig = args.join(" ");
        let selector = self.compute_method_selector(&method_sig)?;

        bytecode.push(4); // Method selector is always 4 bytes
        bytecode.extend_from_slice(&selector);
        Ok(())
    }

    /// Parse Algorand address from base32 format
    fn parse_algorand_address(&self, addr: &str, line_num: usize) -> AvmResult<Vec<u8>> {
        use base32::{Alphabet, decode};
        use sha2::{Digest, Sha512_256};
        
        // Algorand addresses are 58 characters in base32
        if addr.len() != 58 {
            return Err(AvmError::assembly_error(format!(
                "Invalid Algorand address length on line {line_num}: expected 58 characters, got {}",
                addr.len()
            )));
        }

        // Decode the base32 address
        let decoded = decode(Alphabet::Rfc4648 { padding: false }, addr)
            .ok_or_else(|| AvmError::assembly_error(format!(
                "Invalid base32 encoding in address on line {line_num}"
            )))?;

        // Algorand addresses contain 32 bytes + 4 byte checksum = 36 bytes total
        if decoded.len() != 36 {
            return Err(AvmError::assembly_error(format!(
                "Invalid decoded address length on line {line_num}: expected 36 bytes, got {}",
                decoded.len()
            )));
        }

        // Split address and checksum
        let (address_bytes, checksum) = decoded.split_at(32);
        
        // Verify checksum using SHA512-256 (last 4 bytes)
        let mut hasher = Sha512_256::new();
        hasher.update(address_bytes);
        let hash = hasher.finalize();
        let expected_checksum = &hash[hash.len() - 4..];
        
        if checksum != expected_checksum {
            return Err(AvmError::assembly_error(format!(
                "Invalid address checksum on line {line_num}"
            )));
        }

        Ok(address_bytes.to_vec())
    }

    /// Compute ARC-4 method selector from method signature
    fn compute_method_selector(&self, method_sig: &str) -> AvmResult<[u8; 4]> {
        use sha2::{Digest, Sha256};

        // Compute SHA-256 hash of method signature
        let mut hasher = Sha256::new();
        hasher.update(method_sig.as_bytes());
        let hash = hasher.finalize();

        // Take first 4 bytes as method selector
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&hash[..4]);
        Ok(selector)
    }

    /// Assemble substring arguments
    fn assemble_substring_args(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.len() < 2 {
            return Err(AvmError::assembly_error(format!(
                "substring requires start and length on line {line_num}"
            )));
        }

        let start: u8 = args[0].parse().map_err(|_| {
            AvmError::assembly_error(format!(
                "Invalid start value '{}' on line {}",
                args[0], line_num
            ))
        })?;

        let length: u8 = args[1].parse().map_err(|_| {
            AvmError::assembly_error(format!(
                "Invalid length value '{}' on line {}",
                args[1], line_num
            ))
        })?;

        bytecode.push(start);
        bytecode.push(length);
        Ok(())
    }

    /// Assemble transaction field
    fn assemble_txn_field(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing transaction field on line {line_num}"
            )));
        }

        let field_id = match args[0] {
            "Sender" => 0,
            "Fee" => 1,
            "FirstValid" => 2,
            "FirstValidTime" => 3,
            "LastValid" => 4,
            "Note" => 5,
            "Lease" => 6,
            "Receiver" => 7,
            "Amount" => 8,
            "CloseRemainderTo" => 9,
            "VotePK" => 10,
            "SelectionPK" => 11,
            "VoteFirst" => 12,
            "VoteLast" => 13,
            "VoteKeyDilution" => 14,
            "Type" => 15,
            "TypeEnum" => 16,
            "XferAsset" => 17,
            "AssetAmount" => 18,
            "AssetSender" => 19,
            "AssetReceiver" => 20,
            "AssetCloseTo" => 21,
            "GroupIndex" => 22,
            "TxID" => 23,
            "ApplicationID" => 24,
            "OnCompletion" => 25,
            "ApplicationArgs" => 26,
            "NumAppArgs" => 27,
            "Accounts" => 28,
            "NumAccounts" => 29,
            "ApprovalProgram" => 30,
            "ClearStateProgram" => 31,
            "RekeyTo" => 32,
            "ConfigAsset" => 33,
            "ConfigAssetTotal" => 34,
            "ConfigAssetDecimals" => 35,
            "ConfigAssetDefaultFrozen" => 36,
            "ConfigAssetUnitName" => 37,
            "ConfigAssetName" => 38,
            "ConfigAssetURL" => 39,
            "ConfigAssetMetadataHash" => 40,
            "ConfigAssetManager" => 41,
            "ConfigAssetReserve" => 42,
            "ConfigAssetFreeze" => 43,
            "ConfigAssetClawback" => 44,
            "FreezeAsset" => 45,
            "FreezeAssetAccount" => 46,
            "FreezeAssetFrozen" => 47,
            "Assets" => 48,
            "NumAssets" => 49,
            "Applications" => 50,
            "NumApplications" => 51,
            "GlobalNumUint" => 52,
            "GlobalNumByteSlice" => 53,
            "LocalNumUint" => 54,
            "LocalNumByteSlice" => 55,
            "ExtraProgramPages" => 56,
            "Nonparticipation" => 57,
            "Logs" => 58,
            "NumLogs" => 59,
            "CreatedAssetID" => 60,
            "CreatedApplicationID" => 61,
            "LastLog" => 62,
            "StateProofPK" => 63,
            "ApprovalProgramPages" => 64,
            "NumApprovalProgramPages" => 65,
            "ClearStateProgramPages" => 66,
            "NumClearStateProgramPages" => 67,
            _ => {
                return Err(AvmError::assembly_error(format!(
                    "Unknown transaction field '{}' on line {}",
                    args[0], line_num
                )));
            }
        };

        bytecode.push(field_id);
        Ok(())
    }

    /// Assemble group transaction arguments
    fn assemble_gtxn_args(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.len() < 2 {
            return Err(AvmError::assembly_error(format!(
                "gtxn requires group index and field on line {line_num}"
            )));
        }

        let group_index: u8 = args[0].parse().map_err(|_| {
            AvmError::assembly_error(format!(
                "Invalid group index '{}' on line {}",
                args[0], line_num
            ))
        })?;

        bytecode.push(group_index);
        self.assemble_txn_field(bytecode, &args[1..], line_num)?;
        Ok(())
    }

    /// Assemble global field
    fn assemble_global_field(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "Missing global field on line {line_num}"
            )));
        }

        let field_id = match args[0] {
            "MinTxnFee" => 0,
            "MinBalance" => 1,
            "MaxTxnLife" => 2,
            "ZeroAddress" => 3,
            "GroupSize" => 4,
            "LogicSigVersion" => 5,
            "Round" => 6,
            "LatestTimestamp" => 7,
            "CurrentApplicationID" => 8,
            "CreatorAddress" => 9,
            "CurrentApplicationAddress" => 10,
            "GroupID" => 11,
            "OpcodeBudget" => 12,
            "CallerApplicationID" => 13,
            "CallerApplicationAddress" => 14,
            "AssetCreateMinBalance" => 15,
            "AssetOptInMinBalance" => 16,
            "GenesisHash" => 17,
            _ => {
                return Err(AvmError::assembly_error(format!(
                    "Unknown global field '{}' on line {}",
                    args[0], line_num
                )));
            }
        };

        bytecode.push(field_id);
        Ok(())
    }

    /// Assemble integer constant block
    fn assemble_intcblock(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "intcblock requires at least one integer constant on line {line_num}"
            )));
        }

        // Write count as varuint
        let count_bytes = encode_varuint(args.len() as u64);
        bytecode.extend_from_slice(&count_bytes);

        // Write each integer constant as varuint
        for arg in args {
            let value = self.parse_integer(arg, line_num)?;
            let value_bytes = encode_varuint(value);
            bytecode.extend_from_slice(&value_bytes);
        }

        Ok(())
    }

    /// Assemble byte constant block
    fn assemble_bytecblock(
        &mut self,
        bytecode: &mut Vec<u8>,
        args: &[&str],
        line_num: usize,
    ) -> AvmResult<()> {
        if args.is_empty() {
            return Err(AvmError::assembly_error(format!(
                "bytecblock requires at least one byte constant on line {line_num}"
            )));
        }

        // Write count as varuint
        let count_bytes = encode_varuint(args.len() as u64);
        bytecode.extend_from_slice(&count_bytes);

        // Write each byte constant
        for arg in args {
            let bytes = self.parse_bytes(&[arg], line_num)?;
            
            // Write length as varuint followed by the bytes
            let length_bytes = encode_varuint(bytes.len() as u64);
            bytecode.extend_from_slice(&length_bytes);
            bytecode.extend_from_slice(&bytes);
        }

        Ok(())
    }

    /// Resolve forward label references
    fn resolve_forward_refs(&self, bytecode: &mut [u8]) -> AvmResult<()> {
        for (addr, label) in &self.forward_refs {
            let target_addr = self
                .labels
                .get(label)
                .ok_or_else(|| AvmError::assembly_error(format!("Undefined label: {label}")))?;

            // The offset is calculated from the PC position when the branch executes
            // At that point, PC has advanced past the entire instruction (opcode + 2 bytes)
            // So the target calculation is: target_addr = (pc_after_instruction) + offset
            // Therefore: offset = target_addr - pc_after_instruction
            let pc_after_instruction = *addr + 2; // addr points to offset bytes, +2 gets past them
            let offset = (*target_addr as i32) - (pc_after_instruction as i32);
            let offset_bytes = (offset as i16).to_be_bytes();
            bytecode[*addr] = offset_bytes[0];
            bytecode[*addr + 1] = offset_bytes[1];
        }

        Ok(())
    }
}

/// Disassemble bytecode to TEAL source
pub fn disassemble(bytecode: &[u8]) -> AvmResult<String> {
    use crate::varuint::decode_varuint;
    let mut result = String::new();
    let mut pc = 0;

    while pc < bytecode.len() {
        let opcode = bytecode[pc];

        result.push_str(&format!("{pc:04x}: "));

        let (instruction, size) = match opcode {
            OP_ERR => ("err".to_string(), 1),
            OP_PLUS => ("+".to_string(), 1),
            OP_MINUS => ("-".to_string(), 1),
            OP_MUL => ("*".to_string(), 1),
            OP_DIV => ("/".to_string(), 1),
            OP_MOD => ("%".to_string(), 1),
            OP_LT => ("<".to_string(), 1),
            OP_GT => (">".to_string(), 1),
            OP_LE => ("<=".to_string(), 1),
            OP_GE => (">=".to_string(), 1),
            OP_EQ => ("==".to_string(), 1),
            OP_NE => ("!=".to_string(), 1),
            OP_AND => ("&&".to_string(), 1),
            OP_OR => ("||".to_string(), 1),
            OP_NOT => ("!".to_string(), 1),
            OP_BITWISE_OR => ("|".to_string(), 1),
            OP_BITWISE_AND => ("&".to_string(), 1),
            OP_BITWISE_XOR => ("^".to_string(), 1),
            OP_BITWISE_NOT => ("~".to_string(), 1),
            OP_POP => ("pop".to_string(), 1),
            OP_DUP => ("dup".to_string(), 1),
            OP_DUP2 => ("dup2".to_string(), 1),
            OP_SWAP => ("swap".to_string(), 1),
            OP_SELECT => ("select".to_string(), 1),
            OP_BNZ => {
                if pc + 2 < bytecode.len() {
                    let offset = i16::from_be_bytes([bytecode[pc + 1], bytecode[pc + 2]]);
                    let target = (pc as i32 + 3 + offset as i32) as usize;
                    (format!("bnz {target:04x}"), 3)
                } else {
                    ("bnz <invalid>".to_string(), 1)
                }
            }
            OP_BZ => {
                if pc + 2 < bytecode.len() {
                    let offset = i16::from_be_bytes([bytecode[pc + 1], bytecode[pc + 2]]);
                    let target = (pc as i32 + 3 + offset as i32) as usize;
                    (format!("bz {target:04x}"), 3)
                } else {
                    ("bz <invalid>".to_string(), 1)
                }
            }
            OP_B => {
                if pc + 2 < bytecode.len() {
                    let offset = i16::from_be_bytes([bytecode[pc + 1], bytecode[pc + 2]]);
                    let target = (pc as i32 + 3 + offset as i32) as usize;
                    (format!("b {target:04x}"), 3)
                } else {
                    ("b <invalid>".to_string(), 1)
                }
            }
            OP_RETURN => ("return".to_string(), 1),
            OP_ASSERT => ("assert".to_string(), 1),
            OP_RETSUB => ("retsub".to_string(), 1),
            OP_SHA256 => ("sha256".to_string(), 1),
            OP_KECCAK256 => ("keccak256".to_string(), 1),
            OP_SHA512_256 => ("sha512_256".to_string(), 1),
            OP_ED25519VERIFY => ("ed25519verify".to_string(), 1),
            OP_LEN => ("len".to_string(), 1),
            OP_ITOB => ("itob".to_string(), 1),
            OP_BTOI => ("btoi".to_string(), 1),
            OP_CONCAT => ("concat".to_string(), 1),
            OP_SUBSTRING3 => ("substring3".to_string(), 1),
            OP_APP_GLOBAL_GET => ("app_global_get".to_string(), 1),
            OP_APP_GLOBAL_PUT => ("app_global_put".to_string(), 1),
            OP_APP_GLOBAL_DEL => ("app_global_del".to_string(), 1),
            OP_APP_LOCAL_GET => ("app_local_get".to_string(), 1),
            OP_APP_LOCAL_PUT => ("app_local_put".to_string(), 1),
            OP_APP_LOCAL_DEL => ("app_local_del".to_string(), 1),
            OP_BALANCE => ("balance".to_string(), 1),
            OP_MIN_BALANCE => ("min_balance".to_string(), 1),

            // Constant block opcodes
            OP_INTCBLOCK => {
                let mut offset = pc + 1;
                
                // Read count as varuint
                if let Ok((count, consumed)) = decode_varuint(&bytecode[offset..]) {
                    offset += consumed;
                    let count = count as usize;
                    let mut constants = Vec::new();
                    
                    // Read each integer constant as varuint
                    for _ in 0..count {
                        if let Ok((value, consumed)) = decode_varuint(&bytecode[offset..]) {
                            constants.push(value.to_string());
                            offset += consumed;
                        } else {
                            break;
                        }
                    }
                    
                    if constants.len() == count {
                        (format!("intcblock {}", constants.join(" ")), offset - pc)
                    } else {
                        ("intcblock <invalid>".to_string(), 1)
                    }
                } else {
                    ("intcblock <invalid>".to_string(), 1)
                }
            }
            OP_INTC => {
                if pc + 1 < bytecode.len() {
                    let index = bytecode[pc + 1];
                    (format!("intc {index}"), 2)
                } else {
                    ("intc <invalid>".to_string(), 1)
                }
            }
            OP_INTC_0 => ("intc_0".to_string(), 1),
            OP_INTC_1 => ("intc_1".to_string(), 1),
            OP_INTC_2 => ("intc_2".to_string(), 1),
            OP_INTC_3 => ("intc_3".to_string(), 1),
            OP_BYTECBLOCK => {
                let mut offset = pc + 1;
                
                // Read count as varuint
                if let Ok((count, consumed)) = decode_varuint(&bytecode[offset..]) {
                    offset += consumed;
                    let count = count as usize;
                    let mut constants = Vec::new();
                    
                    // Read each byte constant (length-prefixed)
                    for _ in 0..count {
                        if let Ok((length, consumed)) = decode_varuint(&bytecode[offset..]) {
                            offset += consumed;
                            let length = length as usize;
                            
                            if offset + length <= bytecode.len() {
                                let bytes = &bytecode[offset..offset + length];
                                if bytes.iter().all(|&b| b.is_ascii() && !b.is_ascii_control()) {
                                    constants.push(format!("\"{}\"", String::from_utf8_lossy(bytes)));
                                } else {
                                    constants.push(format!("0x{}", hex::encode(bytes)));
                                }
                                offset += length;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    
                    if constants.len() == count {
                        (format!("bytecblock {}", constants.join(" ")), offset - pc)
                    } else {
                        ("bytecblock <invalid>".to_string(), 1)
                    }
                } else {
                    ("bytecblock <invalid>".to_string(), 1)
                }
            }
            OP_BYTEC => {
                if pc + 1 < bytecode.len() {
                    let index = bytecode[pc + 1];
                    (format!("bytec {index}"), 2)
                } else {
                    ("bytec <invalid>".to_string(), 1)
                }
            }
            OP_BYTEC_0 => ("bytec_0".to_string(), 1),
            OP_BYTEC_1 => ("bytec_1".to_string(), 1),
            OP_BYTEC_2 => ("bytec_2".to_string(), 1),
            OP_BYTEC_3 => ("bytec_3".to_string(), 1),

            // Argument opcodes
            OP_ARG => {
                if pc + 1 < bytecode.len() {
                    let index = bytecode[pc + 1];
                    (format!("arg {index}"), 2)
                } else {
                    ("arg <invalid>".to_string(), 1)
                }
            }
            OP_ARG_0 => ("arg_0".to_string(), 1),
            OP_ARG_1 => ("arg_1".to_string(), 1),
            OP_ARG_2 => ("arg_2".to_string(), 1),
            OP_ARG_3 => ("arg_3".to_string(), 1),

            OP_PUSHINT => {
                if pc + 8 < bytecode.len() {
                    let value = u64::from_be_bytes(bytecode[pc + 1..pc + 9].try_into().unwrap());
                    (format!("int {value}"), 9)
                } else {
                    ("int <invalid>".to_string(), 1)
                }
            }

            OP_PUSHBYTES => {
                if pc + 1 < bytecode.len() {
                    let len = bytecode[pc + 1] as usize;
                    if pc + 1 + len < bytecode.len() {
                        let bytes = &bytecode[pc + 2..pc + 2 + len];
                        if bytes.iter().all(|&b| b.is_ascii() && !b.is_ascii_control()) {
                            (
                                format!("byte \"{}\"", String::from_utf8_lossy(bytes)),
                                2 + len,
                            )
                        } else {
                            (format!("byte 0x{}", hex::encode(bytes)), 2 + len)
                        }
                    } else {
                        ("byte <invalid>".to_string(), 1)
                    }
                } else {
                    ("byte <invalid>".to_string(), 1)
                }
            }

            _ => (format!("unknown_{opcode:02x}"), 1),
        };

        result.push_str(&instruction);
        result.push('\n');

        pc += size;
    }

    Ok(result)
}
