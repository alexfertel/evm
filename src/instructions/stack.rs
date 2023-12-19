use alloy_primitives::U256;

use crate::{utils::ToUsize, Interpreter};

use super::InstructionResult;

pub fn push0(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stack.push(U256::ZERO)?;
    Ok(1)
}

pub fn push<const N: usize>(interpreter: &mut Interpreter) -> InstructionResult {
    let ip = interpreter.instruction_pointer.get().as_usize()?;

    let bytecode = &interpreter.contract.bytecode.bytes[ip + 1..];
    let bytes = N.min(bytecode.len());
    let mut slice = [0u8; N];
    slice.copy_from_slice(&bytecode[..bytes]);
    interpreter.stack.push_slice(&slice)?;

    Ok(N + 1)
}

pub fn pop(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stack.pop()?;
    Ok(1)
}

pub fn dup<const N: usize>(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stack.dup(N - 1)?;
    Ok(1)
}

pub fn swap<const N: usize>(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stack.swap(N)?;
    Ok(1)
}
