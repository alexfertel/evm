use crate::interpreter::Interpreter;

pub mod arithmetic;
pub mod bitwise;
pub mod control;
pub mod i256;
pub mod memory;
pub mod opcode;
pub mod stack;
pub mod system;

pub type InstructionResult = eyre::Result<usize>;
pub type Instruction = fn(&mut Interpreter) -> InstructionResult;
