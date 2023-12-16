use std::cell::{Cell, RefCell};

use alloy_primitives::{Bytes, U256};

use crate::{
    instructions::{opcode::instruction, Instruction},
    utils::ToUsize,
    Contract, Memory, Stack,
};

#[derive(Debug)]
pub struct Interpreter {
    pub stack: Stack,
    pub memory: Memory,
    pub contract: Box<Contract>,
    pub gas: Cell<U256>,
    pub instruction_pointer: Cell<U256>,
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
        let ip = self
            .instruction_pointer
            .get()
            .as_usize()
            .expect("ip should fit in a usize");

        // Section 9.4.1 of the yellowpaper, the operation to be executed if the
        // instruction pointer is outside code is STOP.
        if ip > self.contract.bytecode.len() {
            return instruction(0);
        }

        let opcode = self.contract.bytecode.bytes[ip];
        instruction(opcode)
    }

    pub fn execute(&self) -> eyre::Result<Bytes> {
        while !self.stopped.get() {
            let instruction = self.next();
            let offset = instruction(self)?;
            let ip = self.instruction_pointer.get() + U256::from(offset);
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
        // PUSH 0x6
        // PUSH 0x7
        // MUL
        // PUSH 0x0
        // MSTORE8
        // PUSH 0x1
        // PUSH 0x0
        // RETURN
        let bytecode = "0x600660070260005360016000f3";
        let bytes = bytecode.parse().unwrap();
        let contract = Box::new(Contract::new(bytes));
        let interpreter = Interpreter::new(contract);
        let result = interpreter.execute().expect("should finish execution");
        assert_eq!("0x2a".parse::<Bytes>().unwrap(), result);
    }

    #[test]
    fn returns_four_squared() {
        //             # stack
        // PUSH1, 4,   # n=4
        // DUP1,       # n=4, loops=4
        // PUSH1, 0,   # n=4, loops=4, result=0
        //
        // # loop_cond
        // # if loops != 0, jump to loop_body
        // JUMPDEST,
        // DUP2,       # n, loops, result, loops
        // PUSH1, 18,  # n, loops, result, loops, loop_body
        // JUMPI,      # n, loops, result
        //
        // # return result
        // PUSH1, 0,   # n, loops, result, m_result
        // MSTORE8,    # n, loops
        // PUSH1, 1,   # n, loops, mem_length
        // PUSH1, 0,   # n, loops, mem_length, mem_offset
        // RETURN,
        //
        // # loop_body
        // JUMPDEST,
        //
        // # result += n
        // DUP3,       # n, loops, result, n
        // ADD,        # n, loops, result'=n+result
        //
        // # loops -= 1
        // SWAP1,      # n, result', loops
        // PUSH1, 1,   # n, result', loops, 1
        // SWAP1,      # n, result', 1, loops
        // SUB,        # n, result', loops'=loops-1
        //
        // # restore stack
        // SWAP1,      # n, loops', result'
        //
        // # jump to loop_cond
        // PUSH1, 5,   # n, loops', result', loop_cond
        // JUMP,       # -> back to loop_cond
        let bytecode = "60048060005b8160125760005360016000f35b8201906001900390600556";
        let bytes = bytecode.parse().unwrap();
        let contract = Box::new(Contract::new(bytes));
        let interpreter = Interpreter::new(contract);
        let result = interpreter.execute().expect("should finish execution");
        assert_eq!("0x10".parse::<Bytes>().unwrap(), result);
    }
}
