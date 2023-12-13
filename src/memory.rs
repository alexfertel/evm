use std::{cell::RefCell, collections::HashMap};

use alloy_primitives::{U256, U8};

#[derive(Debug)]
pub struct Memory {
    inner: RefCell<HashMap<U256, U8>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn load(&self, addr: U256) -> U8 {
        *self.inner.borrow_mut().entry(addr).or_default()
    }

    pub fn store(&self, addr: U256, byte: U8) {
        self.inner.borrow_mut().insert(addr, byte);
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.inner.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{U256, U8};
    use pretty_assertions::assert_eq;

    use super::Memory;

    #[test]
    fn stores() {
        let memory = Memory::new();
        memory.store(U256::from(0), U8::from(1));
        assert_eq!(1, memory.len());
    }

    #[test]
    fn loads() {
        let memory = Memory::new();
        let addr = U256::from(0);
        let expected = U8::from(1);
        memory.store(addr, expected);
        let actual = memory.load(addr);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cold_load_stores_default() {
        let memory = Memory::new();
        let loaded = memory.load(U256::from(0));
        assert_eq!(U8::from(0), loaded);
        assert_eq!(1, memory.len())
    }
}
