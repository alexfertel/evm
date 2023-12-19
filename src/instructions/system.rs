use alloy_primitives::{B256, U256};

use crate::{constants::WORD_SIZE_BYTES, utils::ToUsize, Interpreter};

use super::InstructionResult;

pub fn calldataload(interpreter: &mut Interpreter) -> InstructionResult {
    let index = interpreter.stack.pop()?.as_usize_saturated();
    let bytes = if index < interpreter.contract.input.len() {
        let bytes = WORD_SIZE_BYTES.min(interpreter.contract.input.len() - index);
        let mut slice = [0u8; WORD_SIZE_BYTES];
        let data_slice = &interpreter.contract.input[index..index + bytes];
        slice[..bytes].copy_from_slice(data_slice);
        B256::new(slice)
    } else {
        B256::ZERO
    };

    interpreter.stack.push(bytes.into())?;
    Ok(1)
}

pub fn calldatasize(interpreter: &mut Interpreter) -> InstructionResult {
    let len = U256::from(interpreter.contract.input.len());
    interpreter.stack.push(len)?;
    Ok(1)
}

pub fn calldatacopy(interpreter: &mut Interpreter) -> InstructionResult {
    let mem_offset = interpreter.stack.pop()?.as_usize()?;
    let data_offset = interpreter.stack.pop()?;
    let length = interpreter.stack.pop()?.as_usize()?;
    if length == 0 {
        return Ok(1);
    }

    let data_offset = data_offset.as_usize_saturated();
    let bytes = if data_offset < interpreter.contract.input.len() {
        let bytes = WORD_SIZE_BYTES.min(interpreter.contract.input.len() - data_offset);
        let mut slice = [0u8; WORD_SIZE_BYTES];
        let data_slice = &interpreter.contract.input[data_offset..data_offset + bytes];
        slice[..bytes].copy_from_slice(data_slice);
        slice
    } else {
        [0u8; WORD_SIZE_BYTES]
    };

    interpreter.memory.set(mem_offset, &bytes);

    Ok(1)
}
