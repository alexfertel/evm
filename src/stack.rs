use std::cell::RefCell;

use alloy_primitives::U256;
use eyre::anyhow;

#[derive(Debug)]
pub struct Stack {
    pub size: usize,
    stack: RefCell<Vec<U256>>,
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
        {
            if self.stack.borrow().len() == self.size {
                return Err(anyhow!("stack overflow"));
            }
        }

        self.stack.borrow_mut().push(elem);

        Ok(())
    }

    pub fn peek(&self) -> Option<U256> {
        {
            if self.stack.borrow().len() == 0 {
                return None;
            }
        }

        let stack = self.stack.borrow();
        Some(stack[stack.len() - 1])
    }

    pub fn pop(&self) -> eyre::Result<U256> {
        {
            if self.stack.borrow().len() == 0 {
                return Err(anyhow!("stack underflow"));
            }
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
    fn push_onto_stack() {
        let stack = Stack::new(STACK_SIZE);
        stack
            .push(U256::from(10))
            .expect("should push onto the stack");
        assert_eq!(1, stack.len());
    }

    #[test]
    fn pops_from_stack() {
        let stack = Stack::new(STACK_SIZE);
        let expected = U256::from(10);
        stack.push(expected).expect("should push onto the stack");

        let actual = stack.pop().expect("should pop from the stack");
        assert_eq!(expected, actual);
    }

    #[test]
    fn overflows() {
        let stack = Stack::new(STACK_SIZE);

        for _ in 0..1024 {
            stack
                .push(U256::from(0))
                .expect("should push onto the stack");
        }
        assert_eq!(1024, stack.len());

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
