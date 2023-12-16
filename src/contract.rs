use alloy_primitives::{Address, Bytes};

use crate::Bytecode;

#[derive(Debug, Default)]
pub struct Contract {
    pub bytecode: Bytecode,
    pub address: Address,
    pub input: Bytes,
}

impl Contract {
    pub fn new(address: Address, bytecode: Bytes, input: Bytes) -> Self {
        let bytecode = Bytecode::new(bytecode);
        Self {
            address,
            bytecode,
            input,
        }
    }

    pub fn is_valid_jump(&self, addr: usize) -> bool {
        addr < self.bytecode.len() && self.bytecode.jumpset[addr]
    }
}
