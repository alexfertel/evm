use crate::{utils::ToUsize, Interpreter};

use super::InstructionResult;

pub fn stop(interpreter: &Interpreter) -> InstructionResult {
    interpreter.stop();
    Ok(0)
}

pub fn jump(interpreter: &Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    if interpreter.contract.is_valid_jump(addr.as_usize()?) {
        // We set `ip` here for the sake of explicitness. The alternative is
        // to return the difference between `ip` and `addr`.
        interpreter.instruction_pointer.set(addr);
    }
    Ok(0)
}

pub fn jumpi(interpreter: &Interpreter) -> InstructionResult {
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

pub fn pc(interpreter: &Interpreter) -> InstructionResult {
    interpreter
        .stack
        .push(interpreter.instruction_pointer.get())?;
    Ok(1)
}

pub fn jumpdest(_: &Interpreter) -> InstructionResult {
    Ok(1)
}

pub fn ret(interpreter: &Interpreter) -> InstructionResult {
    let addr = interpreter.stack.pop()?;
    let length = interpreter.stack.pop()?.as_usize()?;
    if length != 0 {
        let bytes = interpreter.memory.load_bytes(addr, length);
        *interpreter.return_data_buffer.borrow_mut() = bytes;
    }
    interpreter.stop();
    Ok(0)
}

pub fn unknown(interpreter: &Interpreter) -> InstructionResult {
    interpreter.stop();
    Ok(0)
}
