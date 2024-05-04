use alloy_primitives::{B256, U256};

use crate::constants::WORD_SIZE_BYTES;

const FOUR_KB: usize = 4 * 1024; // From evmone.

#[derive(Debug)]
pub struct Memory {
    buffer: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            buffer: Vec::with_capacity(FOUR_KB),
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn resize(&mut self, new_len: usize) {
        self.buffer.resize(new_len, 0);
    }

    pub fn slice(&self, addr: usize, size: usize) -> &[u8] {
        self.buffer
            .get(addr..addr + size)
            .expect("should slice within bounds")
    }

    pub fn slice_mut(&mut self, addr: usize, size: usize) -> &mut [u8] {
        self.buffer
            .get_mut(addr..addr + size)
            .expect("should slice_mut within bounds")
    }

    pub fn get_byte(&self, addr: usize) -> u8 {
        self.slice(addr, 1)[0]
    }

    pub fn get_word(&self, addr: usize) -> B256 {
        self.slice(addr, WORD_SIZE_BYTES)
            .try_into()
            .expect("should convert [u8; 32] to B256")
    }

    pub fn get_u256(&self, addr: usize) -> U256 {
        self.get_word(addr).into()
    }

    pub fn set(&mut self, addr: usize, slice: &[u8]) {
        if !slice.is_empty() {
            self.slice_mut(addr, slice.len()).copy_from_slice(slice);
        }
    }

    pub fn set_byte(&mut self, addr: usize, byte: u8) {
        self.set(addr, &[byte]);
    }

    pub fn set_word(&mut self, addr: usize, bytes: &B256) {
        self.set(addr, &bytes[..])
    }

    pub fn set_u256(&mut self, addr: usize, bytes: &U256) {
        self.set(addr, &bytes.to_be_bytes::<WORD_SIZE_BYTES>())
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{B256, U256};
    use pretty_assertions::assert_eq;

    use crate::constants::WORD_SIZE_BYTES;

    use super::Memory;

    #[test]
    fn resizes() {
        let mut memory = Memory::default();
        assert_eq!(0, memory.len());

        memory.resize(WORD_SIZE_BYTES);
        assert_eq!(WORD_SIZE_BYTES, memory.len());
        assert_eq!(0, memory.get_byte(WORD_SIZE_BYTES - 1));
    }

    #[test]
    #[should_panic]
    fn panics_get_byte_oob() {
        let memory = Memory::default();
        memory.get_byte(0);
    }

    #[test]
    #[should_panic]
    fn panics_get_word_oob() {
        let memory = Memory::default();
        memory.get_word(0);
    }

    #[test]
    #[should_panic]
    fn panics_get_u256_oob() {
        let memory = Memory::default();
        memory.get_u256(0);
    }

    #[test]
    fn sets_byte() {
        let mut memory = Memory::default();
        memory.resize(WORD_SIZE_BYTES);
        memory.set_byte(0, 1);
        assert_eq!(1, memory.get_byte(0));
    }

    #[test]
    #[should_panic]
    fn panics_sets_byte_oob() {
        let mut memory = Memory::default();
        memory.set_byte(0, 1);
    }

    #[test]
    fn sets_word() {
        let mut memory = Memory::default();
        memory.resize(WORD_SIZE_BYTES);
        let word = B256::from(U256::from(1));
        memory.set_word(0, &word);
        assert_eq!(word, memory.get_word(0));
    }

    #[test]
    #[should_panic]
    fn panics_sets_word_oob() {
        let mut memory = Memory::default();
        let word = B256::from(U256::from(1));
        memory.set_word(0, &word);
    }

    #[test]
    fn sets_u256() {
        let mut memory = Memory::default();
        memory.resize(WORD_SIZE_BYTES);
        let word = U256::from(1);
        memory.set_u256(0, &word);
        assert_eq!(word, memory.get_u256(0));
    }

    #[test]
    #[should_panic]
    fn panics_sets_u256_oob() {
        let mut memory = Memory::default();
        let word = U256::from(1);
        memory.set_u256(0, &word);
    }
}
