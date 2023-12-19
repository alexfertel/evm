use alloy_primitives::U256;
use std::ops::Rem;

use super::InstructionResult;
use crate::{
    constants::{WORD_SIZE, WORD_SIZE_BYTES},
    utils::ToUsize,
    Interpreter,
};

pub fn mload(interpreter: &mut Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?.as_usize()?;
    let size = addr + WORD_SIZE_BYTES;
    if size > interpreter.memory.len() {
        interpreter.memory.resize(size);
    }
    let word = interpreter.memory.get_u256(addr);
    interpreter.stack.push(word)?;

    Ok(1)
}

pub fn mstore(interpreter: &mut Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?.as_usize()?;
    let word = interpreter.stack.pop()?;
    let size = addr + WORD_SIZE_BYTES;
    if size > interpreter.memory.len() {
        interpreter.memory.resize(size);
    }
    interpreter.memory.set_u256(addr, &word);
    Ok(1)
}

pub fn mstore8(interpreter: &mut Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?.as_usize()?;
    let word = interpreter.stack.pop()?;
    let size = addr + 1;
    if size > interpreter.memory.len() {
        interpreter.memory.resize(size);
    }
    let byte = word.rem(U256::from(WORD_SIZE)).byte(0);
    interpreter.memory.set_byte(addr, byte);
    Ok(1)
}

pub fn msize(interpreter: &mut Interpreter) -> InstructionResult {
    let size = interpreter.memory.len();
    interpreter.stack.push(U256::from(size))?;
    Ok(1)
}
