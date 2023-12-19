use std::cmp::Ordering;

use alloy_primitives::U256;

use crate::{utils::ToUsize, Interpreter};

use super::{i256::i256_cmp, InstructionResult};

pub fn lt(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(a < b);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn gt(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(a > b);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn slt(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(i256_cmp(&a, &b) == Ordering::Less);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn sgt(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(i256_cmp(&a, &b) == Ordering::Greater);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn eq(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(a == b);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn iszero(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let r = U256::from(a.is_zero());
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn and(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(a & b);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn or(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(a | b);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn xor(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(a ^ b);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn not(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    interpreter.stack.push(!a)?;
    Ok(1)
}

pub fn byte(interpreter: &mut Interpreter) -> InstructionResult {
    let index = interpreter.stack.pop()?.as_usize_saturated();
    let word = interpreter.stack.pop()?;

    let new_word = if index < 32 {
        // `31 - index` because `byte` returns LE, while we want BE.
        U256::from(word.byte(31 - index))
    } else {
        U256::ZERO
    };

    interpreter.stack.push(new_word)?;

    Ok(1)
}

pub fn shl(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(b << a);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn shr(interpreter: &mut Interpreter) -> InstructionResult {
    let a = interpreter.stack.pop()?;
    let b = interpreter.stack.pop()?;
    let r = U256::from(b >> a);
    interpreter.stack.push(r)?;
    Ok(1)
}

pub fn sar(interpreter: &mut Interpreter) -> InstructionResult {
    let shift = interpreter.stack.pop()?;
    let word = interpreter.stack.pop()?;
    let r = word.arithmetic_shr(shift.as_usize_saturated());
    interpreter.stack.push(r)?;
    Ok(1)
}
