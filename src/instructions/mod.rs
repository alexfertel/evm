use crate::interpreter::Interpreter;

pub mod arithmetic;
pub mod control;
pub mod memory;
pub mod opcode;
pub mod stack;

pub type InstructionResult = eyre::Result<usize>;
pub type Instruction = fn(&Interpreter) -> InstructionResult;
