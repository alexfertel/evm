use alloy_primitives::U256;
use std::ops::Rem;

use super::InstructionResult;
use crate::{
    constants::{WORD_SIZE, WORD_SIZE_BYTES},
    Interpreter,
};

pub fn mload(interpreter: &Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    let word = interpreter.memory.load_word(addr);
    interpreter.stack.push(word)?;

    Ok(1)
}

pub fn mstore(interpreter: &Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    let word = interpreter.stack.pop()?;
    let bytes = word.to_be_bytes::<WORD_SIZE_BYTES>();
    interpreter.memory.copy_from_bytes(addr, bytes.into());
    Ok(1)
}

pub fn mstore8(interpreter: &Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    let word = interpreter.stack.pop()?;
    let byte = word.rem(U256::from(WORD_SIZE)).byte(0);
    interpreter.memory.store(addr, byte);
    Ok(1)
}
