use std::cell::{Cell, RefCell};

use alloy_primitives::{Bytes, U256};

use crate::{
    instructions::{opcode::instruction, Instruction},
    Contract, Memory, Stack,
};

#[derive(Debug)]
pub struct Interpreter {
    pub stack: Stack,
    pub memory: Memory,
    pub contract: Box<Contract>,
    pub gas: Cell<U256>,
    pub instruction_pointer: Cell<usize>,
    pub return_data_buffer: RefCell<Bytes>,
    pub stopped: Cell<bool>,
}

impl Interpreter {
    pub fn new(contract: Box<Contract>) -> Self {
        Self {
            stack: Stack::default(),
            memory: Memory::default(),
            contract,
            gas: Default::default(),
            instruction_pointer: Default::default(),
            return_data_buffer: Default::default(),
            stopped: Default::default(),
        }
    }

    pub fn stop(&self) {
        self.stopped.set(true);
    }

    pub fn next(&self) -> Instruction {
        let ip = self.instruction_pointer.get();

        // Section 9.4.1 of the yellowpaper, the operation to be executed if the
        // instruction pointer is outside code is STOP.
        if ip > self.contract.bytecode.len() {
            return instruction(0);
        }

        let opcode = self.contract.bytecode[ip];
        instruction(opcode)
    }

    pub fn execute(&self) -> eyre::Result<Bytes> {
        while !self.stopped.get() {
            let instruction = self.next();
            let offset = instruction(self)?;
            let ip = self.instruction_pointer.get() + offset;
            self.instruction_pointer.set(ip);
        }

        Ok(self.return_data_buffer.borrow().clone())
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::Bytes;
    use pretty_assertions::assert_eq;

    use crate::Contract;

    use super::Interpreter;

    #[test]
    fn stops() {
        let interpreter = Interpreter::new(Default::default());
        assert_eq!(false, interpreter.stopped.get());
        interpreter.stop();
        assert_eq!(true, interpreter.stopped.get());
    }

    #[test]
    fn returns_mul_result() {
        let bytecode = "0x600660070260005360016000f3";
        let bytes = bytecode.parse().unwrap();
        let contract = Box::new(Contract::new(bytes));
        let interpreter = Interpreter::new(contract);
        let result = interpreter.execute().expect("should finish execution");
        assert_eq!("0x2a".parse::<Bytes>().unwrap(), result);
    }
}
