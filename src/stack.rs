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

    pub fn peek(&self, index: usize) -> Option<U256> {
        if self.stack.borrow().len() < index + 1 {
            return None;
        }

        let stack = self.stack.borrow();
        Some(stack[stack.len() - index - 1])
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

    pub fn dup(&self, index: usize) -> eyre::Result<()> {
        if self.stack.borrow().len() < index + 1 {
            return Err(anyhow!("stack underflow"));
        }

        let mut stack = self.stack.borrow_mut();
        let index = stack.len() - index - 1;
        let elem = stack[index];
        stack.push(elem);
        Ok(())
    }

    pub fn swap(&self, index: usize) -> eyre::Result<()> {
        if index == 0 {
            return Err(anyhow!("invalid index"));
        } else if self.stack.borrow().len() < index + 1 {
            return Err(anyhow!("stack underflow"));
        }

        let mut stack = self.stack.borrow_mut();
        let len = stack.len();
        let index = len - index - 1;
        stack.swap(index, len - 1);
        Ok(())
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.stack.borrow().len()
    }

    #[cfg(test)]
    fn into_iter(&self) -> std::vec::IntoIter<alloy_primitives::Uint<256, 4>> {
        let stack = self.stack.borrow().clone();
        stack.into_iter()
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
        assert_eq!(66051u64, u64::try_from(stack.peek(0).unwrap()).unwrap());
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
        let actual = stack.peek(0).expect("should peek the stack");
        assert_eq!(expected, actual);

        stack
            .push(U256::from(2))
            .expect("should push onto the stack");
        let actual = stack.peek(1).expect("should peek the stack");
        assert_eq!(expected, actual);
    }

    #[test]
    fn dups_last() {
        let stack = Stack::new(STACK_SIZE);
        let expected = U256::from(1);
        stack.push(expected).expect("should push onto the stack");
        stack
            .dup(0)
            .expect("should dup the last element on the stack");
        assert_eq!(2, stack.len());
        let actual = stack.peek(0).expect("should peek the stack");
        assert_eq!(expected, actual);
    }

    #[test]
    fn dups_second_to_last() {
        let stack = Stack::new(STACK_SIZE);
        let expected = U256::from(1);
        stack.push(expected).expect("should push onto the stack");
        stack
            .push(U256::from(2))
            .expect("should push onto the stack");
        stack
            .dup(1)
            .expect("should dup the second to last element on the stack");
        assert_eq!(3, stack.len());
        let actual = stack.peek(0).expect("should peek the stack");
        assert_eq!(expected, actual);
    }

    #[test]
    fn swaps_zero_errors() {
        let stack = Stack::new(STACK_SIZE);
        let expected = U256::from(1);
        stack.push(expected).expect("should push onto the stack");
        stack
            .push(U256::from(2))
            .expect("should push onto the stack");
        let e = stack.swap(0);
        assert_eq!(true, matches!(e, Err(_)));
    }

    #[test]
    fn swaps_last() {
        let stack = Stack::new(STACK_SIZE);
        stack
            .push(U256::from(1))
            .expect("should push onto the stack");
        stack
            .push(U256::from(2))
            .expect("should push onto the stack");
        stack
            .swap(1)
            .expect("should swap the last two elements on the stack");
        let actual = stack.peek(0).expect("should peek the stack");
        assert_eq!(U256::from(1), actual);
        let actual = stack.peek(1).expect("should peek the stack");
        assert_eq!(U256::from(2), actual);
    }

    #[test]
    fn swaps_second_to_last() {
        let stack = Stack::new(STACK_SIZE);
        stack
            .push(U256::from(1))
            .expect("should push onto the stack");
        stack
            .push(U256::from(2))
            .expect("should push onto the stack");
        stack
            .push(U256::from(3))
            .expect("should push onto the stack");
        stack
            .swap(2)
            .expect("should dup the second to last element on the stack");
        let all_equal = vec![U256::from(3), U256::from(2), U256::from(1)]
            .into_iter()
            .zip(stack.into_iter())
            .all(|(a, b)| a == b);
        assert_eq!(true, all_equal);
    }
}
