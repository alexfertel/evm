use alloy_primitives::U256;

use crate::Interpreter;

use super::InstructionResult;

pub fn add(interpreter: &Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    interpreter.stack.push(a.wrapping_add(b))?;

    Ok(1)
}

pub fn mul(interpreter: &Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    interpreter.stack.push(a.wrapping_mul(b))?;

    Ok(1)
}

pub fn sub(interpreter: &Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    interpreter.stack.push(a.wrapping_sub(b))?;

    Ok(1)
}

pub fn div(interpreter: &Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = if b.is_zero() {
        U256::ZERO
    } else {
        a.wrapping_div(b)
    };
    interpreter.stack.push(r)?;

    Ok(1)
}
