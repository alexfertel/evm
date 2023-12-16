use alloy_primitives::U256;

/// Extension for U256 to cast it to a usize.
pub trait ToUsize {
    fn as_usize(&self) -> eyre::Result<usize>;
    fn as_usize_saturated(&self) -> usize;
}

impl ToUsize for U256 {
    fn as_usize(&self) -> eyre::Result<usize> {
        let x: &[u64; 4] = self.as_limbs();
        if x[1] != 0 || x[2] != 0 || x[3] != 0 {
            return Err(eyre::anyhow!("invalid operand"));
        }
        Ok(x[0] as usize)
    }

    fn as_usize_saturated(&self) -> usize {
        let x: &[u64; 4] = self.as_limbs();
        if x[1] == 0 && x[2] == 0 && x[3] == 0 {
            x[0] as usize
        } else {
            usize::MAX
        }
    }
}
