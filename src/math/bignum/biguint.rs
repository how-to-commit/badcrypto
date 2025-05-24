use crate::math::arith::{borrowing_sub, carrying_add};

#[derive(Debug, PartialEq, Eq)]
pub struct BigUint {
    limbs: [u32; Self::NUM_LIMBS],
}

impl BigUint {
    // 64 limbs * 32 bits = 2048 bits
    const NUM_LIMBS: usize = 64;
    const LIMB_SIZE: u32 = u32::MAX;

    pub const fn zero() -> Self {
        Self {
            limbs: [0; Self::NUM_LIMBS],
        }
    }

    pub fn from_slice(val: &[u32]) -> Self {
        if val.len() > Self::NUM_LIMBS {
            panic!("Attempt to create BigUint with overflow.");
        }

        let mut ret = Self::zero();
        ret.limbs[0..val.len()].copy_from_slice(&val);
        ret
    }

    pub const fn from_u128(val: u128) -> Self {
        let mut ret = Self::zero();
        let mut v = val;

        ret.limbs[0] = (v & (Self::LIMB_SIZE as u128)) as u32;
        v >>= 32;
        ret.limbs[1] = (v & (Self::LIMB_SIZE as u128)) as u32;
        v >>= 32;
        ret.limbs[2] = (v & (Self::LIMB_SIZE as u128)) as u32;
        v >>= 32;
        ret.limbs[3] = (v & (Self::LIMB_SIZE as u128)) as u32;

        ret
    }

    /// perform self + rhs + carry, returning self and the carry
    pub fn carrying_add(&self, rhs: &Self, mut carry: u32) -> (Self, u32) {
        let mut ret = Self::zero();
        for i in 0..Self::NUM_LIMBS {
            (ret.limbs[i], carry) = carrying_add(self.limbs[i], rhs.limbs[i], carry);
        }
        (ret, carry)
    }

    /// perform self - rhs - borrow, returning self and the borrow
    pub fn borrowing_sub(&self, rhs: &Self, mut borrow: u32) -> (Self, u32) {
        let mut ret = Self::zero();
        for i in 0..Self::NUM_LIMBS {
            (ret.limbs[i], borrow) = borrowing_sub(self.limbs[i], rhs.limbs[i], borrow);
        }
        (ret, borrow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        assert_eq!(u32::MAX, 0xFFFF_FFFF); // sanity

        let huge_number = 5;
        let fu128 = BigUint::from_u128(huge_number as u128);
        let fslice = BigUint::from_slice(&[huge_number]);
        assert_eq!(fu128, fslice);

        let big_fu128 = BigUint::from_u128(0x1234_5678_abcd_ef00);
        let big_fslice = BigUint::from_slice(&[0xabcd_ef00, 0x1234_5678]);
        assert_eq!(big_fu128, big_fslice);
    }

    #[test]
    fn carrying_add_no_carry() {
        // bigger than one limb size
        let res1 = BigUint::from_u128(u64::MAX as u128 + 5);
        let op11 = BigUint::from_u128(u64::MAX as u128);
        let op12 = BigUint::from_u128(5 as u128);
        assert_eq!(op11.carrying_add(&op12, 0), (res1, 0));

        // smaller than one limb size
        let res2 = BigUint::from_u128(15 as u128);
        let op21 = BigUint::from_u128(10 as u128);
        let op22 = BigUint::from_u128(5 as u128);
        assert_eq!(op21.carrying_add(&op22, 0), (res2, 0));

        // cross one limb size
        let res3 = BigUint::from_u128(0x1_0000_0004);
        let op31 = BigUint::from_u128(0xFFFF_FFFF);
        let op32 = BigUint::from_u128(5 as u128);
        assert_eq!(op31.carrying_add(&op32, 0), (res3, 0));
    }

    #[test]
    fn borrowing_sub_no_carry() {
        let res1 = BigUint::from_u128(u64::MAX as u128 - 5);
        let op11 = BigUint::from_u128(u64::MAX as u128);
        let op12 = BigUint::from_u128(5 as u128);
        assert_eq!(op11.borrowing_sub(&op12, 0), (res1, 0));
    }
}
