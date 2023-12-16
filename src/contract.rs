use alloy_primitives::Bytes;

use crate::Bytecode;

#[derive(Debug, Default)]
pub struct Contract {
    pub bytecode: Bytecode,
}

impl Contract {
    pub fn new(bytecode: Bytes) -> Self {
        let bytecode = Bytecode::new(bytecode);
        Self { bytecode }
    }

    pub fn is_valid_jump(&self, addr: usize) -> bool {
        addr < self.bytecode.len() && self.bytecode.jumpset[addr]
    }
}
