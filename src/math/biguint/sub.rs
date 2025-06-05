use super::BigUint;
use crate::math::arith::borrowing_sub;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    /// perform self - rhs - borrow, returning self and the borrow
    pub fn borrowing_sub(&self, rhs: &Self, mut borrow: u32) -> (Self, u32) {
        let mut ret = Self::zero();
        for i in 0..NUM_LIMBS {
            (ret.limbs[i], borrow) = borrowing_sub(self.limbs[i], rhs.limbs[i], borrow);
        }
        (ret, borrow)
    }

    pub fn _sub(&self, rhs: &Self) -> Self {
        self.borrowing_sub(rhs, 0).0
    }

    /// perform self - rhs (mod p), where 0 < self - rhs < 2p
    pub fn sub_mod_lt2p(&self, rhs: &Self, modulo: &Self) -> Self {
        let (sum, _) = self.borrowing_sub(rhs, 0);
        let (sum_minus_mod, borrow) = sum.borrowing_sub(modulo, 0);
        BigUint::ct_select(&sum, &sum_minus_mod, borrow)
    }

    /// perform self - rhs (mod p). conducts full modulo operation after
    pub fn sub_mod_full(&self, rhs: &Self, modulo: &Self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::*;
    #[test]
    fn borrowing_sub_no_carry() {
        let res1 = Bu256::from_u128(u64::MAX as u128 - 5);
        let op11 = Bu256::from_u128(u64::MAX as u128);
        let op12 = Bu256::from_u128(5 as u128);
        assert_eq!(op11.borrowing_sub(&op12, 0), (res1, 0));
    }

    #[test]
    fn borrowing_sub_w_carry() {
        let op1 = Bu256::from_u128(0);
        let op2 = Bu256::from_u128(5);
        let mut res_slice = vec![0xFFFF_FFFF - 4];
        res_slice.extend_from_slice(&[0xFFFF_FFFF; Bu256::LIMBS - 1]);
        assert_eq!(
            op1.borrowing_sub(&op2, 0),
            (Bu256::from_slice(&res_slice), 1)
        );
    }
}
