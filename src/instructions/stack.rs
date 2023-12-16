use crate::{utils::ToUsize, Interpreter};

use super::InstructionResult;

pub fn push<const N: usize>(interpreter: &Interpreter) -> InstructionResult {
    let ip = interpreter.instruction_pointer.get().as_usize()?;

    let mut slice = [0u8; N];
    let bytecode = &interpreter.contract.bytecode.bytes[ip + 1..];
    let bytecode = if bytecode.len() > N {
        &bytecode[..N]
    } else {
        bytecode
    };

    slice.copy_from_slice(bytecode);
    interpreter.stack.push_slice(&slice)?;

    Ok(N + 1)
}

pub fn dup<const N: usize>(interpreter: &Interpreter) -> InstructionResult {
    interpreter.stack.dup(N - 1)?;
    Ok(1)
}

pub fn swap<const N: usize>(interpreter: &Interpreter) -> InstructionResult {
    interpreter.stack.swap(N)?;
    Ok(1)
}
