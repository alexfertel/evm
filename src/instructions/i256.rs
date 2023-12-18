use alloy_primitives::U256;
use core::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum Sign {
    Negative = -1,
    Positive = 1,
}

pub fn i256_sign(val: &U256) -> Sign {
    if val.bit(U256::BITS - 1) {
        Sign::Negative
    } else {
        Sign::Positive
    }
}

pub fn i256_cmp(a: &U256, b: &U256) -> Ordering {
    let a_sign = i256_sign(a);
    let b_sign = i256_sign(b);
    match a_sign.cmp(&b_sign) {
        Ordering::Equal => a.cmp(b),
        o => o,
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, ops::Sub};

    use alloy_primitives::U256;

    use super::i256_cmp;

    const MSB_BITMASK_U64: u64 = 0x8000000000000000;

    fn u256_flip_sign(val: &mut U256) {
        // SAFETY: U256 does not have any padding bytes.
        unsafe { val.as_limbs_mut()[3] ^= MSB_BITMASK_U64 }
    }

    fn u(value: usize) -> U256 {
        U256::from(value)
    }

    #[test]
    fn cmps_i256() {
        let zero = U256::ZERO;
        let max = U256::MAX;
        let mut max_positive = U256::MAX;
        u256_flip_sign(&mut max_positive);
        assert_eq!(Ordering::Equal, i256_cmp(&zero, &zero));
        assert_eq!(Ordering::Less, i256_cmp(&u(0), &u(1)));
        assert_eq!(Ordering::Greater, i256_cmp(&u(1), &u(0)));
        assert_eq!(Ordering::Equal, i256_cmp(&max, &max));
        assert_eq!(Ordering::Less, i256_cmp(&max, &zero));
        assert_eq!(Ordering::Greater, i256_cmp(&zero, &max));
        assert_eq!(Ordering::Less, i256_cmp(&max.sub(u(1)), &max));
        assert_eq!(Ordering::Greater, i256_cmp(&max, &max.sub(u(1))));
        assert_eq!(Ordering::Greater, i256_cmp(&max, &max.sub(u(1))));
        assert_eq!(Ordering::Less, i256_cmp(&max, &max_positive));
        assert_eq!(Ordering::Greater, i256_cmp(&max_positive, &max));
        assert_eq!(Ordering::Equal, i256_cmp(&max_positive, &max_positive));
    }
}
