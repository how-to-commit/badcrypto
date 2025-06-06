use super::BigUint;
use crate::math::arith::carrying_add;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    /// perform self + rhs + carry, returning self and the carry
    pub fn carrying_add(&self, rhs: &Self, mut carry: u32) -> (Self, u32) {
        let mut ret = Self::zero();
        for i in 0..NUM_LIMBS {
            (ret.limbs[i], carry) = carrying_add(self.limbs[i], rhs.limbs[i], carry);
        }
        (ret, carry)
    }

    /// add, no carry
    pub fn _add(&self, rhs: &Self) -> Self {
        self.carrying_add(rhs, 0).0
    }

    /// perform self + rhs (mod p), where self + rhs < 2p
    /// let x = self + rhs
    /// if x < 2p,
    /// x mod p = (x-p) : x ? x - p > 0;
    pub fn add_mod_lt2p(&self, rhs: &Self, modulo: &Self) -> Self {
        let (sum, _) = self.carrying_add(rhs, 0); // discard carry, self + rhs small
        let (sum_minus_mod, borrow) = sum.borrowing_sub(modulo, 0);
        BigUint::ct_select(&sum, &sum_minus_mod, borrow)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::*;

    #[test]
    fn carrying_add_no_carry() {
        // bigger than one limb size
        let res1 = Bu512::from_u128(u64::MAX as u128 + 5);
        let op11 = Bu512::from_u128(u64::MAX as u128);
        let op12 = Bu512::from_u128(5 as u128);
        assert_eq!(op11.carrying_add(&op12, 0), (res1, 0));

        // smaller than one limb size
        let res2 = Bu512::from_u128(15 as u128);
        let op21 = Bu512::from_u128(10 as u128);
        let op22 = Bu512::from_u128(5 as u128);
        assert_eq!(op21.carrying_add(&op22, 0), (res2, 0));

        // cross one limb size
        let res3 = Bu512::from_u128(0x1_0000_0004);
        let op31 = Bu512::from_u128(0xFFFF_FFFF);
        let op32 = Bu512::from_u128(5 as u128);
        assert_eq!(op31.carrying_add(&op32, 0), (res3, 0));
    }

    #[test]
    fn carrying_add_w_carry() {
        // 0xFF + 0x05 = 0x1_04; 1 overflow + 4 rem
        let op1 = Bu512::from_slice(&[0xFFFF_FFFF; Bu512::LIMBS]);
        let op2 = Bu512::from_u128(5);
        assert_eq!(op1.carrying_add(&op2, 0), (Bu512::from_u128(4), 1));
    }
}
