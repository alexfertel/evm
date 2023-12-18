use alloy_primitives::Bytes;
use bitvec::prelude::{bitvec, BitVec, Lsb0};

use crate::instructions::opcode;

type JumpSet = BitVec<usize, Lsb0>;

#[derive(Debug)]
pub struct Bytecode {
    pub bytes: Bytes,
    pub jumpset: JumpSet,
}

impl Default for Bytecode {
    fn default() -> Self {
        // A single STOP opcode.
        let bytes = Bytes::from([0; 2]);
        let jumpset = create_jumpset(&bytes);
        Self { bytes, jumpset }
    }
}

impl Bytecode {
    pub fn new(bytes: Bytes) -> Self {
        let jumpset = create_jumpset(&bytes);
        Self { bytes, jumpset }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

fn create_jumpset(bytes: &Bytes) -> JumpSet {
    let mut jumpset: JumpSet = bitvec![usize, Lsb0; 0; bytes.len()];
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == opcode::JUMPDEST {
            jumpset.set(i, true);
            i += 1;
        } else {
            let push_offset = bytes[i].wrapping_sub(opcode::PUSH1) as usize;
            let is_push = push_offset < 32;
            if is_push {
                i += push_offset + 2;
            } else {
                i += 1;
            }
        }
    }

    jumpset
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::create_jumpset;
    use bitvec::prelude::{bits, Lsb0};

    #[test]
    fn creates_jumpset() {
        // PUSH 0x6
        // PUSH 0x7
        // MUL
        // JUMPDEST
        // PUSH 0x0
        // MSTORE8
        // PUSH 0x1
        // PUSH 0x0
        // STOP
        // JUMPDEST
        // SELFDESTRUCT
        let bytecode = "0x60066007025b60005360016000005bFF";
        let bytes = bytecode.parse().unwrap();
        let jumpset = create_jumpset(&bytes);
        let expected = bits![u8, Lsb0; 0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0];
        assert_eq!(expected, jumpset);
    }
}
