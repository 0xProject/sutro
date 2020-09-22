/// Ethereum Virtual Machine Opcodes.
/// See <https://ethereum.github.io/yellowpaper/paper.pdf>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opcode {
    // 0x00-0x0B: Stop and Arithmetic Operations
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    SDiv,
    Mod,
    SMod,
    AddMod,
    MulMod,
    Exp,
    SignExtend,

    // 0x10-0x1D: Comparison & Bitwise Logic Operations
    Lt,
    Gt,
    SLt,
    SGt,
    Eq,
    IsZero,
    And,
    Or,
    Xor,
    Not,
    Byte,
    Shl,
    Shr,
    Sar,

    // 0x20-0x20: SHA3
    Sha3,

    // 0x30-0x3F: Environmental Information
    Address,
    Balance,
    Origin,
    Caller,
    CallValue,
    CallDataLoad,
    CallDataSize,
    CallDataCopy,
    CodeSize,
    CodeCopy,
    GasPrice,
    ExtCodeSize,
    ExtCodeCopy,
    ReturnDataSize,
    ReturnDataCopy,
    ExtCodeHash,

    // 0x40-0x45: Block Information
    BlockHash,
    Coinbase,
    Timestamp,
    Number,
    Difficulty,
    GasLimit,

    // 0x50-0x5B: Stack, Memory, Storage and Flow Operations
    Pop,
    MLoad,
    MStore,
    MStore8,
    SLoad,
    SStore,
    Jump,
    JumpI,
    PC,
    MSize,
    Gas,
    JumpDest,

    // 0x60-0x7F: Push Operations
    Push(u8),

    // 0x80-0x8F: Duplication Operations
    Dup(u8),

    // 0x90-0x9F: Exchange Operations
    Swap(u8),

    // 0xA0-0xA4: Logging Operations
    Log(u8),

    // 0xF0-0xFF: System Operations
    Create,
    Call,
    CallCode,
    Return,
    DelegateCall,
    Create2,
    StaticCall,
    Revert,
    Invalid(u8),
    SelfDestruct,
}

use Opcode::*;

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        // PERF: Use a static lookup table
        match opcode {
            0x00 => Stop,
            0x01 => Add,
            0x02 => Mul,
            0x03 => Sub,
            0x04 => Div,
            0x05 => SDiv,
            0x06 => Mod,
            0x07 => SMod,
            0x08 => AddMod,
            0x09 => MulMod,
            0x0a => Exp,
            0x0b => SignExtend,

            0x10 => Lt,
            0x11 => Gt,
            0x12 => SLt,
            0x13 => SGt,
            0x14 => Eq,
            0x15 => IsZero,
            0x16 => And,
            0x17 => Or,
            0x18 => Xor,
            0x19 => Not,
            0x1a => Byte,
            0x1b => Shl,
            0x1c => Shr,
            0x1d => Sar,

            0x20 => Sha3,

            0x30 => Address,
            0x31 => Balance,
            0x32 => Origin,
            0x33 => Caller,
            0x34 => CallValue,
            0x35 => CallDataLoad,
            0x36 => CallDataSize,
            0x37 => CallDataCopy,
            0x38 => CodeSize,
            0x39 => CodeCopy,
            0x3a => GasPrice,
            0x3b => ExtCodeSize,
            0x3c => ExtCodeCopy,
            0x3d => ReturnDataSize,
            0x3e => ReturnDataCopy,
            0x3f => ExtCodeHash,

            0x40 => BlockHash,
            0x41 => Coinbase,
            0x42 => Timestamp,
            0x43 => Number,
            0x44 => Difficulty,
            0x45 => GasLimit,

            0x50 => Pop,
            0x51 => MLoad,
            0x52 => MStore,
            0x53 => MStore8,
            0x54 => SLoad,
            0x55 => SStore,
            0x56 => Jump,
            0x57 => JumpI,
            0x58 => PC,
            0x59 => MSize,
            0x5a => Gas,
            0x5b => JumpDest,

            0x60..=0x7F => Push(1 + opcode - 0x60),
            0x80..=0x8F => Dup(1 + opcode - 0x80),
            0x90..=0x9F => Swap(1 + opcode - 0x90),
            0xA0..=0xA4 => Log(opcode - 0xA0),

            0xf0 => Create,
            0xf1 => Call,
            0xf2 => CallCode,
            0xf3 => Return,
            0xf4 => DelegateCall,
            0xf5 => Create2,
            0xfa => StaticCall,
            0xfd => Revert,
            0xff => SelfDestruct,

            invalid => Invalid(invalid),
        }
    }
}

impl Opcode {
    pub fn to_u8(self) -> u8 {
        for u8 in 0..=255 {
            if Opcode::from(u8) == self {
                return u8;
            }
        }
        panic!("{:?} has no Opcode::from value.", self);
    }

    /// Is this the final instruction in a decoding sequence.
    pub fn is_block_final(self) -> bool {
        match self {
            Stop | Jump | Return | Revert | Invalid(_) => true,
            _ => false,
        }
    }

    /// Stack (consume, produce)
    pub fn stack(self) -> (usize, usize) {
        match self {
            Stop | JumpDest | Invalid(_) => (0, 0),
            Address | Origin | Caller | CallValue | CallDataSize | CodeSize | GasPrice
            | ReturnDataSize | Coinbase | Timestamp | Number | Difficulty | GasLimit | PC
            | MSize | Gas | Push(_) => (0, 1),
            Pop | Jump | SelfDestruct => (1, 0),
            IsZero | Not | Balance | CallDataLoad | ExtCodeSize | ExtCodeHash | BlockHash
            | MLoad | SLoad => (1, 1),
            MStore | MStore8 | SStore | JumpI | Return | Revert => (2, 0),
            Add | Mul | Sub | Div | SDiv | Mod | SMod | Exp | SignExtend | Lt | Gt | SLt | SGt
            | Eq | And | Or | Xor | Byte | Shl | Shr | Sar | Sha3 => (2, 1),
            CallDataCopy | CodeCopy | ReturnDataCopy => (3, 0),
            AddMod | MulMod | Create => (3, 1),
            ExtCodeCopy => (4, 0),
            Create2 => (4, 1),
            DelegateCall | StaticCall => (6, 1),
            Call | CallCode => (7, 1),
            Dup(n) => (n as usize, (n as usize) + 1),
            Swap(n) => ((n as usize) + 1, (n as usize) + 1),
            Log(n) => ((n as usize) + 2, 0),
        }
    }

    /// Minimum amount of gas consumed by the opcode
    /// Does not account for memory growth
    pub fn base_gas(self) -> usize {
        match self {
            // Zero
            Stop | Return | Revert => 0,
            // Base
            Address | Origin | Caller | CallValue | CallDataSize | CodeSize | GasPrice
            | Coinbase | Timestamp | Number | Difficulty | GasLimit | ReturnDataSize | Pop | PC
            | MSize | Gas => 2,
            // Very low
            Add | Sub | Not | Lt | Gt | SLt | SGt | Eq | IsZero | And | Or | Xor | Byte | Shl
            | Shr | Sar | CallDataLoad | MLoad | MStore | MStore8 | Push(_) | Dup(_) | Swap(_) => 3,
            // Low
            Mul | Div | SDiv | Mod | SMod | SignExtend => 5,
            // Mid
            AddMod | MulMod | Jump => 8,
            // High
            JumpI => 10,
            // Special cases with constant gas
            Create => 32000,
            JumpDest => 1,
            SLoad => 200,
            ExtCodeSize => 700,
            ExtCodeHash => 400,
            Balance => 400,
            BlockHash => 20,
            Invalid(_) => 0,
            // Special cases with dynamic gas (returns minimum not accounting for refunds)
            // TODO: Some of these only depend on a size argument on the stack.
            SStore => 5000,
            Exp => 10,
            CallDataCopy | CodeCopy | ReturnDataCopy => 3,
            ExtCodeCopy => 700,
            Log(n) => 375 + (n as usize) * 375,
            Call | CallCode | DelegateCall | StaticCall => 700,
            SelfDestruct => 5000,
            Create2 => 32000,
            Sha3 => 30,
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self.to_u8()
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
