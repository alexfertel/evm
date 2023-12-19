use crate::{utils::ToUsize, Interpreter};

use super::InstructionResult;

pub fn stop(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stop();
    Ok(0)
}

pub fn jump(interpreter: &mut Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    if interpreter.contract.is_valid_jump(addr.as_usize()?) {
        // We set `ip` here for the sake of explicitness. The alternative is
        // to return the difference between `ip` and `addr`.
        interpreter.instruction_pointer.set(addr);
    }
    Ok(0)
}

pub fn jumpi(interpreter: &mut Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    let value = interpreter.stack.pop()?;
    if !value.is_zero() {
        let addr_usize = addr.as_usize()?;
        if interpreter.contract.is_valid_jump(addr_usize) {
            // We set `ip` here for the sake of explicitness. The alternative is
            // to return the difference between `ip` and `addr`.
            interpreter.instruction_pointer.set(addr);
        }
        Ok(0)
    } else {
        Ok(1)
    }
}

pub fn pc(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter
        .stack
        .push(interpreter.instruction_pointer.get())?;
    Ok(1)
}

pub fn jumpdest(_: &mut Interpreter) -> InstructionResult {
    Ok(1)
}

pub fn ret(interpreter: &mut Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?.as_usize()?;
    let length = interpreter.stack.pop()?.as_usize()?;
    if length != 0 {
        let size = addr + length;
        if size > interpreter.memory.len() {
            interpreter.memory.resize(size);
        }
        let bytes = interpreter.memory.slice(addr, length).to_vec();
        *interpreter.return_data_buffer.borrow_mut() = bytes.into();
    }
    interpreter.stop();
    Ok(0)
}

pub fn invalid(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stop();
    Ok(0)
}

pub fn unknown(interpreter: &mut Interpreter) -> InstructionResult {
    interpreter.stop();
    Ok(0)
}
