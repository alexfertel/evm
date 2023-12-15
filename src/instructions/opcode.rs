use super::{arithmetic, control, memory, stack, Instruction};

macro_rules! opcodes {
    ($($val:literal => $name:ident => $f:expr),* $(,)?) => {
        // Constants for each opcode. This also takes care of duplicate names.
        $(
            #[doc = concat!("The `", stringify!($val), "` (\"", stringify!($name),"\") opcode.")]
            pub const $name: u8 = $val;
        )*

        /// Maps each opcode to its name.
        pub const OPCODE_JUMPMAP: [Option<&'static str>; 256] = {
            let mut map = [None; 256];
            let mut prev: u8 = 0;
            $(
                let val: u8 = $val;
                assert!(val == 0 || val > prev, "opcodes must be sorted in ascending order");
                prev = val;
                map[$val] = Some(stringify!($name));
            )*
            let _ = prev;
            map
        };

        /// Returns the instruction function for the given opcode and spec.
        pub fn instruction(opcode: u8) -> Instruction {
            match opcode {
                $($name => $f,)*
                _ => control::unknown,
            }
        }
    };
}

opcodes! {
    0x00 => STOP       => control::stop,

    0x01 => ADD        => arithmetic::add,
    0x02 => MUL        => arithmetic::mul,
    0x03 => SUB        => arithmetic::sub,
    0x04 => DIV        => arithmetic::div,
    // 0x05 => SDIV       => arithmetic::sdiv,
    // 0x06 => MOD        => arithmetic::rem,
    // 0x07 => SMOD       => arithmetic::smod,
    // 0x08 => ADDMOD     => arithmetic::addmod,
    // 0x09 => MULMOD     => arithmetic::mulmod,
    // 0x0A => EXP        => arithmetic::exp,
    // 0x0B => SIGNEXTEND => arithmetic::signextend,
    // 0x0C
    // 0x0D
    // 0x0E
    // 0x0F
    // 0x10 => LT     => bitwise::lt,
    // 0x11 => GT     => bitwise::gt,
    // 0x12 => SLT    => bitwise::slt,
    // 0x13 => SGT    => bitwise::sgt,
    // 0x14 => EQ     => bitwise::eq,
    // 0x15 => ISZERO => bitwise::iszero,
    // 0x16 => AND    => bitwise::bitand,
    // 0x17 => OR     => bitwise::bitor,
    // 0x18 => XOR    => bitwise::bitxor,
    // 0x19 => NOT    => bitwise::not,
    // 0x1A => BYTE   => bitwise::byte,
    // 0x1B => SHL    => bitwise::shl,
    // 0x1C => SHR    => bitwise::shr,
    // 0x1D => SAR    => bitwise::sar,
    // 0x1E
    // 0x1F
    // 0x20 => KECCAK256 => system::keccak256,
    // 0x21
    // 0x22
    // 0x23
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28
    // 0x29
    // 0x2A
    // 0x2B
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30 => ADDRESS        => system::address,
    // 0x31 => BALANCE        => host::balance,
    // 0x32 => ORIGIN         => host_env::origin,
    // 0x33 => CALLER         => system::caller,
    // 0x34 => CALLVALUE      => system::callvalue,
    // 0x35 => CALLDATALOAD   => system::calldataload,
    // 0x36 => CALLDATASIZE   => system::calldatasize,
    // 0x37 => CALLDATACOPY   => system::calldatacopy,
    // 0x38 => CODESIZE       => system::codesize,
    // 0x39 => CODECOPY       => system::codecopy,
    //
    // 0x3A => GASPRICE       => host_env::gasprice,
    // 0x3B => EXTCODESIZE    => host::extcodesize,
    // 0x3C => EXTCODECOPY    => host::extcodecopy,
    // 0x3D => RETURNDATASIZE => system::returndatasize,
    // 0x3E => RETURNDATACOPY => system::returndatacopy,
    // 0x3F => EXTCODEHASH    => host::extcodehash,
    // 0x40 => BLOCKHASH      => host::blockhash,
    // 0x41 => COINBASE       => host_env::coinbase,
    // 0x42 => TIMESTAMP      => host_env::timestamp,
    // 0x43 => NUMBER         => host_env::number,
    // 0x44 => DIFFICULTY     => host_env::difficulty,
    // 0x45 => GASLIMIT       => host_env::gaslimit,
    // 0x46 => CHAINID        => host_env::chainid,
    // 0x47 => SELFBALANCE    => host::selfbalance,
    // 0x48 => BASEFEE        => host_env::basefee,
    // 0x49 => BLOBHASH       => host_env::blob_hash,
    // 0x4A => BLOBBASEFEE    => host_env::blob_basefee,
    // 0x4B
    // 0x4C
    // 0x4D
    // 0x4E
    // 0x4F
    // 0x50 => POP      => stack::pop,
    // 0x51 => MLOAD    => memory::mload,
    // 0x52 => MSTORE   => memory::mstore,
    0x53 => MSTORE8  => memory::mstore8,
    // 0x54 => SLOAD    => host::sload,
    // 0x55 => SSTORE   => host::sstore,
    // 0x56 => JUMP     => control::jump,
    // 0x57 => JUMPI    => control::jumpi,
    // 0x58 => PC       => control::pc,
    // 0x59 => MSIZE    => memory::msize,
    // 0x5A => GAS      => system::gas,
    // 0x5B => JUMPDEST => control::jumpdest,
    // 0x5C => TLOAD    => host::tload,
    // 0x5D => TSTORE   => host::tstore,
    // 0x5E => MCOPY    => memory::mcopy,
    //
    // 0x5F => PUSH0  => stack::push0,
    0x60 => PUSH1  => stack::push::<1>,
    0x61 => PUSH2  => stack::push::<2>,
    0x62 => PUSH3  => stack::push::<3>,
    0x63 => PUSH4  => stack::push::<4>,
    0x64 => PUSH5  => stack::push::<5>,
    0x65 => PUSH6  => stack::push::<6>,
    0x66 => PUSH7  => stack::push::<7>,
    0x67 => PUSH8  => stack::push::<8>,
    0x68 => PUSH9  => stack::push::<9>,
    0x69 => PUSH10 => stack::push::<10>,
    0x6A => PUSH11 => stack::push::<11>,
    0x6B => PUSH12 => stack::push::<12>,
    0x6C => PUSH13 => stack::push::<13>,
    0x6D => PUSH14 => stack::push::<14>,
    0x6E => PUSH15 => stack::push::<15>,
    0x6F => PUSH16 => stack::push::<16>,
    0x70 => PUSH17 => stack::push::<17>,
    0x71 => PUSH18 => stack::push::<18>,
    0x72 => PUSH19 => stack::push::<19>,
    0x73 => PUSH20 => stack::push::<20>,
    0x74 => PUSH21 => stack::push::<21>,
    0x75 => PUSH22 => stack::push::<22>,
    0x76 => PUSH23 => stack::push::<23>,
    0x77 => PUSH24 => stack::push::<24>,
    0x78 => PUSH25 => stack::push::<25>,
    0x79 => PUSH26 => stack::push::<26>,
    0x7A => PUSH27 => stack::push::<27>,
    0x7B => PUSH28 => stack::push::<28>,
    0x7C => PUSH29 => stack::push::<29>,
    0x7D => PUSH30 => stack::push::<30>,
    0x7E => PUSH31 => stack::push::<31>,
    0x7F => PUSH32 => stack::push::<32>,

    // 0x80 => DUP1  => stack::dup::<1>,
    // 0x81 => DUP2  => stack::dup::<2>,
    // 0x82 => DUP3  => stack::dup::<3>,
    // 0x83 => DUP4  => stack::dup::<4>,
    // 0x84 => DUP5  => stack::dup::<5>,
    // 0x85 => DUP6  => stack::dup::<6>,
    // 0x86 => DUP7  => stack::dup::<7>,
    // 0x87 => DUP8  => stack::dup::<8>,
    // 0x88 => DUP9  => stack::dup::<9>,
    // 0x89 => DUP10 => stack::dup::<10>,
    // 0x8A => DUP11 => stack::dup::<11>,
    // 0x8B => DUP12 => stack::dup::<12>,
    // 0x8C => DUP13 => stack::dup::<13>,
    // 0x8D => DUP14 => stack::dup::<14>,
    // 0x8E => DUP15 => stack::dup::<15>,
    // 0x8F => DUP16 => stack::dup::<16>,
    //
    // 0x90 => SWAP1  => stack::swap::<1>,
    // 0x91 => SWAP2  => stack::swap::<2>,
    // 0x92 => SWAP3  => stack::swap::<3>,
    // 0x93 => SWAP4  => stack::swap::<4>,
    // 0x94 => SWAP5  => stack::swap::<5>,
    // 0x95 => SWAP6  => stack::swap::<6>,
    // 0x96 => SWAP7  => stack::swap::<7>,
    // 0x97 => SWAP8  => stack::swap::<8>,
    // 0x98 => SWAP9  => stack::swap::<9>,
    // 0x99 => SWAP10 => stack::swap::<10>,
    // 0x9A => SWAP11 => stack::swap::<11>,
    // 0x9B => SWAP12 => stack::swap::<12>,
    // 0x9C => SWAP13 => stack::swap::<13>,
    // 0x9D => SWAP14 => stack::swap::<14>,
    // 0x9E => SWAP15 => stack::swap::<15>,
    // 0x9F => SWAP16 => stack::swap::<16>,
    //
    // 0xA0 => LOG0 => host::log::<0>,
    // 0xA1 => LOG1 => host::log::<1>,
    // 0xA2 => LOG2 => host::log::<2>,
    // 0xA3 => LOG3 => host::log::<3>,
    // 0xA4 => LOG4 => host::log::<4>,
    // 0xA5
    // 0xA6
    // 0xA7
    // 0xA8
    // 0xA9
    // 0xAA
    // 0xAB
    // 0xAC
    // 0xAD
    // 0xAE
    // 0xAF
    // 0xB0
    // 0xB1
    // 0xB2
    // 0xB3
    // 0xB4
    // 0xB5
    // 0xB6
    // 0xB7
    // 0xB8
    // 0xB9
    // 0xBA
    // 0xBB
    // 0xBC
    // 0xBD
    // 0xBE
    // 0xBF
    // 0xC0
    // 0xC1
    // 0xC2
    // 0xC3
    // 0xC4
    // 0xC5
    // 0xC6
    // 0xC7
    // 0xC8
    // 0xC9
    // 0xCA
    // 0xCB
    // 0xCC
    // 0xCD
    // 0xCE
    // 0xCF
    // 0xD0
    // 0xD1
    // 0xD2
    // 0xD3
    // 0xD4
    // 0xD5
    // 0xD6
    // 0xD7
    // 0xD8
    // 0xD9
    // 0xDA
    // 0xDB
    // 0xDC
    // 0xDD
    // 0xDE
    // 0xDF
    // 0xE0
    // 0xE1
    // 0xE2
    // 0xE3
    // 0xE4
    // 0xE5
    // 0xE6
    // 0xE7
    // 0xE8
    // 0xE9
    // 0xEA
    // 0xEB
    // 0xEC
    // 0xED
    // 0xEE
    // 0xEF
    // 0xF0 => CREATE       => host::create::<false, H, SPEC>,
    // 0xF1 => CALL         => host::call,
    // 0xF2 => CALLCODE     => host::call_code,
    0xF3 => RETURN       => control::ret,
    // 0xF4 => DELEGATECALL => host::delegate_call,
    // 0xF5 => CREATE2      => host::create::<true, H, SPEC>,
    // 0xF6
    // 0xF7
    // 0xF8
    // 0xF9
    // 0xFA => STATICCALL   => host::static_call,
    // 0xFB
    // 0xFC
    // 0xFD => REVERT       => control::revert,
    // 0xFE => INVALID      => control::invalid,
    // 0xFF => SELFDESTRUCT => host::selfdestruct,
}
