use alloy_primitives::Bytes;

#[derive(Debug, Default)]
pub struct Contract {
    pub bytecode: Bytes,
}

impl Contract {
    pub fn new(bytecode: Bytes) -> Self {
        Self { bytecode }
    }
}
