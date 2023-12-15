use crate::Interpreter;

pub fn push<const N: usize>(interpreter: &Interpreter) -> eyre::Result<usize> {
    let ip = interpreter.instruction_pointer.get();

    let mut slice = [0u8; N];
    let bytecode = &interpreter.contract.bytecode[ip + 1..];
    let bytecode = if bytecode.len() > N {
        &bytecode[..N]
    } else {
        bytecode
    };

    slice.copy_from_slice(bytecode);
    interpreter.stack.push_slice(&slice)?;

    Ok(N + 1)
}
