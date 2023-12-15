use crate::Interpreter;

pub fn stop(interpreter: &Interpreter) -> eyre::Result<usize> {
    interpreter.stop();
    Ok(0)
}

pub fn unknown(interpreter: &Interpreter) -> eyre::Result<usize> {
    interpreter.stop();
    Ok(0)
}

pub fn ret(interpreter: &Interpreter) -> eyre::Result<usize> {
    let addr = interpreter.stack.pop()?;
    let length = interpreter.stack.pop()?.byte(0);
    if length != 0 {
        let bytes = interpreter.memory.load_range(addr, length);
        *interpreter.return_data_buffer.borrow_mut() = bytes;
    }
    interpreter.stop();
    Ok(0)
}
