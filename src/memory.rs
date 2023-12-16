use std::{cell::RefCell, collections::HashMap};

use alloy_primitives::{Bytes, U256};

#[derive(Debug, Default)]
pub struct Memory {
    inner: RefCell<HashMap<U256, u8>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn load(&self, addr: U256) -> u8 {
        *self.inner.borrow_mut().entry(addr).or_default()
    }

    pub fn load_range(&self, addr: U256, length: usize) -> Bytes {
        let mut bytes = Vec::with_capacity(length);
        for i in 0..length {
            let byte = self.load(addr.saturating_add(U256::from(i)));
            bytes.push(byte);
        }
        Bytes::from(bytes)
    }

    pub fn store(&self, addr: U256, byte: u8) {
        self.inner.borrow_mut().insert(addr, byte);
    }

    pub fn len(&self) -> usize {
        self.inner.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use pretty_assertions::assert_eq;

    use super::Memory;

    #[test]
    fn stores() {
        let memory = Memory::new();
        memory.store(U256::from(0), u8::from(1));
        assert_eq!(1, memory.len());
    }

    #[test]
    fn loads() {
        let memory = Memory::new();
        let addr = U256::from(0);
        let expected = u8::from(1);
        memory.store(addr, expected);
        let actual = memory.load(addr);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cold_load_stores_default() {
        let memory = Memory::new();
        let loaded = memory.load(U256::from(0));
        assert_eq!(u8::from(0), loaded);
        assert_eq!(1, memory.len())
    }
}
