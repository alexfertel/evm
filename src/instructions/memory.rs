use std::ops::Rem;

use alloy_primitives::U256;

use crate::Interpreter;

pub fn mstore8(interpreter: &Interpreter) -> eyre::Result<usize> {
    let addr = interpreter.stack.pop()?;
    let word = interpreter.stack.pop()?;
    let byte = word.rem(U256::from(256)).byte(0);

    interpreter.memory.store(addr, byte);

    Ok(1)
}
