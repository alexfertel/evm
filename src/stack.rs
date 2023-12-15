use std::{cell::RefCell, fmt::Display};

use alloy_primitives::U256;
use eyre::anyhow;

use crate::constants::STACK_SIZE;

#[derive(Debug)]
pub struct Stack {
    pub size: usize,
    stack: RefCell<Vec<U256>>,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            size: STACK_SIZE,
            stack: Default::default(),
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stack.borrow())
    }
}

impl Stack {
    pub fn new(size: usize) -> Self {
        let stack = Vec::with_capacity(size);
        Self {
            size,
            stack: RefCell::new(stack),
        }
    }

    pub fn push(&self, elem: U256) -> eyre::Result<()> {
        if self.stack.borrow().len() == self.size {
            return Err(anyhow!("stack overflow"));
        }

        self.stack.borrow_mut().push(elem);

        Ok(())
    }

    pub fn push_slice(&self, slice: &[u8]) -> eyre::Result<()> {
        if self.stack.borrow().len() + slice.len().div_ceil(32) > self.size {
            return Err(anyhow!("stack overflow"));
        }

        // The slice `&[0, 1, 2, 3]`, which is the hex `0x00010203`
        // gets read as ABCD, without reversing the slice.
        let elem = U256::from_be_slice(slice);
        self.stack.borrow_mut().push(elem);

        Ok(())
    }

    pub fn peek(&self) -> Option<U256> {
        if self.stack.borrow().len() == 0 {
            return None;
        }

        let stack = self.stack.borrow();
        Some(stack[stack.len() - 1])
    }

    pub fn pop(&self) -> eyre::Result<U256> {
        if self.stack.borrow().len() == 0 {
            return Err(anyhow!("stack underflow"));
        }

        Ok(self
            .stack
            .borrow_mut()
            .pop()
            .expect("should pop from the stack"))
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.stack.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use pretty_assertions::assert_eq;

    use crate::constants::STACK_SIZE;

    use super::Stack;

    #[test]
    fn default_impl() {
        let stack = Stack::default();
        assert_eq!(STACK_SIZE, stack.size);
        assert_eq!(0, stack.len());
    }

    #[test]
    fn pushes_onto_stack() {
        let stack = Stack::new(STACK_SIZE);
        stack
            .push(U256::from(10))
            .expect("should push onto the stack");
        assert_eq!(1, stack.len());
    }

    #[test]
    fn pushes_slice_onto_stack() {
        let stack = Stack::new(STACK_SIZE);
        stack
            .push_slice(&[0, 1, 2, 3])
            .expect("should push onto the stack");
        assert_eq!(66051u64, u64::try_from(stack.peek().unwrap()).unwrap());
    }

    #[test]
    fn pops_from_stack() {
        let stack = Stack::new(STACK_SIZE);
        let expected = U256::from(1);
        stack.push(expected).expect("should push onto the stack");

        let actual = stack.pop().expect("should pop from the stack");
        assert_eq!(expected, actual);
    }

    #[test]
    fn overflows() {
        let stack = Stack::new(STACK_SIZE);

        for _ in 0..STACK_SIZE {
            stack
                .push(U256::from(0))
                .expect("should push onto the stack");
        }
        assert_eq!(STACK_SIZE, stack.len());

        let err = stack.push(U256::from(0)).unwrap_err();
        assert_eq!("stack overflow", err.to_string());
    }

    #[test]
    fn underflows() {
        let stack = Stack::new(STACK_SIZE);
        let err = stack.pop().unwrap_err();
        assert_eq!("stack underflow", err.to_string());
    }

    #[test]
    fn peeks() {
        let stack = Stack::new(STACK_SIZE);
        let expected = U256::from(1);
        stack.push(expected).expect("should push onto the stack");
        let actual = stack.peek().expect("should peek the stack");
        assert_eq!(expected, actual);
    }
}
