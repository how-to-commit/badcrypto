use super::BigUint;
use crate::math::arith::carrying_add;

impl BigUint {
    /// perform self + rhs + carry, returning self and the carry
    pub fn carrying_add(&self, rhs: &Self, mut carry: u32) -> (Self, u32) {
        let mut ret = Self::zero();
        for i in 0..Self::NUM_LIMBS {
            (ret.limbs[i], carry) = carrying_add(self.limbs[i], rhs.limbs[i], carry);
        }
        (ret, carry)
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

    /// perform self + rhs (mod p). more expensive than _le2p variant
    pub fn add_mod_ex(&self, rhs: &Self, modulo: &Self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn carrying_add_w_carry() {
        // 0xFF + 0x05 = 0x1_04; 1 overflow + 4 rem
        let op1 = BigUint::from_slice(&[0xFFFF_FFFF; BigUint::NUM_LIMBS]);
        let op2 = BigUint::from_u128(5);
        assert_eq!(op1.carrying_add(&op2, 0), (BigUint::from_u128(4), 1));
    }
}
