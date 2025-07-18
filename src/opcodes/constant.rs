// Opcode constants
pub const OP_ERR: u8 = 0x00;
pub const OP_SHA256: u8 = 0x01;
pub const OP_KECCAK256: u8 = 0x02;
pub const OP_SHA512_256: u8 = 0x03;
pub const OP_ED25519VERIFY: u8 = 0x04;
pub const OP_ECDSA_VERIFY: u8 = 0x05;
pub const OP_ECDSA_PK_DECOMPRESS: u8 = 0x06;
pub const OP_ECDSA_PK_RECOVER: u8 = 0x07;
pub const OP_PLUS: u8 = 0x08;
pub const OP_MINUS: u8 = 0x09;
pub const OP_DIV: u8 = 0x0a;
pub const OP_MUL: u8 = 0x0b;
pub const OP_LT: u8 = 0x0c;
pub const OP_GT: u8 = 0x0d;
pub const OP_LE: u8 = 0x0e;
pub const OP_GE: u8 = 0x0f;
pub const OP_AND: u8 = 0x10;
pub const OP_OR: u8 = 0x11;
pub const OP_EQ: u8 = 0x12;
pub const OP_NE: u8 = 0x13;
pub const OP_NOT: u8 = 0x14;
pub const OP_LEN: u8 = 0x15;
pub const OP_ITOB: u8 = 0x16;
pub const OP_BTOI: u8 = 0x17;
pub const OP_MOD: u8 = 0x18;
pub const OP_BITWISE_OR: u8 = 0x19;
pub const OP_BITWISE_AND: u8 = 0x1a;
pub const OP_BITWISE_XOR: u8 = 0x1b;
pub const OP_BITWISE_NOT: u8 = 0x1c;
pub const OP_MULW: u8 = 0x1d;
pub const OP_ADDW: u8 = 0x1e;
pub const OP_DIVMODW: u8 = 0x1f;
pub const OP_INTCBLOCK: u8 = 0x20;
pub const OP_INTC: u8 = 0x21;
pub const OP_INTC_0: u8 = 0x22;
pub const OP_INTC_1: u8 = 0x23;
pub const OP_INTC_2: u8 = 0x24;
pub const OP_INTC_3: u8 = 0x25;
pub const OP_BYTECBLOCK: u8 = 0x26;
pub const OP_BYTEC: u8 = 0x27;
pub const OP_BYTEC_0: u8 = 0x28;
pub const OP_BYTEC_1: u8 = 0x29;
pub const OP_BYTEC_2: u8 = 0x2a;
pub const OP_BYTEC_3: u8 = 0x2b;
pub const OP_ARG: u8 = 0x2c;
pub const OP_ARG_0: u8 = 0x2d;
pub const OP_ARG_1: u8 = 0x2e;
pub const OP_ARG_2: u8 = 0x2f;
pub const OP_ARG_3: u8 = 0x30;
pub const OP_TXN: u8 = 0x31;
pub const OP_GLOBAL: u8 = 0x32;
pub const OP_GTXN: u8 = 0x33;
pub const OP_LOAD: u8 = 0x34;
pub const OP_STORE: u8 = 0x35;
pub const OP_TXNA: u8 = 0x36;
pub const OP_GTXNA: u8 = 0x37;
pub const OP_GTXNS: u8 = 0x38;
pub const OP_GTXNSA: u8 = 0x39;
pub const OP_GLOAD: u8 = 0x3a;
pub const OP_GLOADS: u8 = 0x3b;
pub const OP_GAID: u8 = 0x3c;
pub const OP_GAIDS: u8 = 0x3d;
pub const OP_LOADS: u8 = 0x3e;
pub const OP_STORES: u8 = 0x3f;
pub const OP_BNZ: u8 = 0x40;
pub const OP_BZ: u8 = 0x41;
pub const OP_B: u8 = 0x42;
pub const OP_RETURN: u8 = 0x43;
pub const OP_ASSERT: u8 = 0x44;
// Stack manipulation opcodes
pub const OP_BURY: u8 = 0x45;
pub const OP_POPN: u8 = 0x46;
pub const OP_DUPN: u8 = 0x47;
pub const OP_POP: u8 = 0x48;
pub const OP_DUP: u8 = 0x49;
pub const OP_DUP2: u8 = 0x4a;
pub const OP_DIG: u8 = 0x4b;
pub const OP_SWAP: u8 = 0x4c;
pub const OP_SELECT: u8 = 0x4d;
pub const OP_COVER: u8 = 0x4e;
pub const OP_UNCOVER: u8 = 0x4f;
pub const OP_CONCAT: u8 = 0x50;
pub const OP_SUBSTRING: u8 = 0x51;
pub const OP_SUBSTRING3: u8 = 0x52;
pub const OP_GETBIT: u8 = 0x53;
pub const OP_SETBIT: u8 = 0x54;
pub const OP_GETBYTE: u8 = 0x55;
pub const OP_SETBYTE: u8 = 0x56;
pub const OP_EXTRACT: u8 = 0x57;
pub const OP_EXTRACT3: u8 = 0x58;
pub const OP_EXTRACT_UINT16: u8 = 0x59;
pub const OP_EXTRACT_UINT32: u8 = 0x5a;
pub const OP_EXTRACT_UINT64: u8 = 0x5b;
pub const OP_REPLACE2: u8 = 0x5c;
pub const OP_REPLACE3: u8 = 0x5d;
pub const OP_BASE64_DECODE: u8 = 0x5e;
pub const OP_JSON_REF: u8 = 0x5f;
pub const OP_BALANCE: u8 = 0x60;
pub const OP_APP_OPTED_IN: u8 = 0x61;
pub const OP_APP_LOCAL_GET: u8 = 0x62;
pub const OP_APP_LOCAL_GET_EX: u8 = 0x63;
pub const OP_APP_GLOBAL_GET: u8 = 0x64;
pub const OP_APP_GLOBAL_GET_EX: u8 = 0x65;
pub const OP_APP_LOCAL_PUT: u8 = 0x66;
pub const OP_APP_GLOBAL_PUT: u8 = 0x67;
pub const OP_APP_LOCAL_DEL: u8 = 0x68;
pub const OP_APP_GLOBAL_DEL: u8 = 0x69;
pub const OP_ASSET_HOLDING_GET: u8 = 0x70;
pub const OP_ASSET_PARAMS_GET: u8 = 0x71;
pub const OP_APP_PARAMS_GET: u8 = 0x72;
pub const OP_ACCT_PARAMS_GET: u8 = 0x73;
pub const OP_MIN_BALANCE: u8 = 0x78;
pub const OP_PUSHBYTES: u8 = 0x80;
pub const OP_PUSHINT: u8 = 0x81;
pub const OP_PUSHBYTESS: u8 = 0x82;
pub const OP_PUSHINTS: u8 = 0x83;
pub const OP_ED25519VERIFY_BARE: u8 = 0x84;
pub const OP_CALLSUB: u8 = 0x88;
pub const OP_RETSUB: u8 = 0x89;
pub const OP_PROTO: u8 = 0x8a;
pub const OP_FRAME_DIG: u8 = 0x8b;
pub const OP_FRAME_BURY: u8 = 0x8c;
pub const OP_SWITCH: u8 = 0x8d;
pub const OP_MATCH: u8 = 0x8e;
pub const OP_SHL: u8 = 0x90;
pub const OP_SHR: u8 = 0x91;
pub const OP_SQRT: u8 = 0x92;
pub const OP_BITLEN: u8 = 0x93;
pub const OP_EXP: u8 = 0x94;
pub const OP_EXPW: u8 = 0x95;
pub const OP_BSQRT: u8 = 0x96;
pub const OP_DIVW: u8 = 0x97;
pub const OP_SHA3_256: u8 = 0x98;
pub const OP_B_PLUS: u8 = 0xa0;
pub const OP_B_MINUS: u8 = 0xa1;
pub const OP_B_DIV: u8 = 0xa2;
pub const OP_B_MUL: u8 = 0xa3;
pub const OP_B_LT: u8 = 0xa4;
pub const OP_B_GT: u8 = 0xa5;
pub const OP_B_LE: u8 = 0xa6;
pub const OP_B_GE: u8 = 0xa7;
pub const OP_B_EQ: u8 = 0xa8;
pub const OP_B_NE: u8 = 0xa9;
pub const OP_B_MOD: u8 = 0xaa;
pub const OP_B_OR: u8 = 0xab;
pub const OP_B_AND: u8 = 0xac;
pub const OP_B_XOR: u8 = 0xad;
pub const OP_B_NOT: u8 = 0xae;
pub const OP_BZERO: u8 = 0xaf;
pub const OP_LOG: u8 = 0xb0;
pub const OP_ITXN_BEGIN: u8 = 0xb1;
pub const OP_ITXN_FIELD: u8 = 0xb2;
pub const OP_ITXN_SUBMIT: u8 = 0xb3;
pub const OP_ITXN: u8 = 0xb4;
pub const OP_ITXNA: u8 = 0xb5;
pub const OP_ITXN_NEXT: u8 = 0xb6;
pub const OP_GITXN: u8 = 0xb7;
pub const OP_GITXNA: u8 = 0xb8;
pub const OP_BOX_CREATE: u8 = 0xb9;
pub const OP_BOX_EXTRACT: u8 = 0xba;
pub const OP_BOX_REPLACE: u8 = 0xbb;
pub const OP_BOX_DEL: u8 = 0xbc;
pub const OP_BOX_LEN: u8 = 0xbd;
pub const OP_BOX_GET: u8 = 0xbe;
pub const OP_BOX_PUT: u8 = 0xbf;
pub const OP_TXNAS: u8 = 0xc0;
pub const OP_GTXNAS: u8 = 0xc1;
pub const OP_GTXNSAS: u8 = 0xc2;
pub const OP_ARGS: u8 = 0xc3;
pub const OP_GLOADSS: u8 = 0xc4;
pub const OP_ITXNAS: u8 = 0xc5;
pub const OP_GITXNAS: u8 = 0xc6;
pub const OP_VRF_VERIFY: u8 = 0xd0;
pub const OP_BLOCK: u8 = 0xd1;
pub const OP_BOX_SPLICE: u8 = 0xd2;
pub const OP_BOX_RESIZE: u8 = 0xd3;
// Elliptic curve operations (v10)
pub const OP_EC_ADD: u8 = 0xe0;
pub const OP_EC_SCALAR_MUL: u8 = 0xe1;
pub const OP_EC_PAIRING_CHECK: u8 = 0xe2;
pub const OP_EC_MULTI_SCALAR_MUL: u8 = 0xe3;
pub const OP_EC_SUBGROUP_CHECK: u8 = 0xe4;
pub const OP_EC_MAP_TO: u8 = 0xe5;
// v11 operations
pub const OP_MIMC: u8 = 0xe6;
