use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    /// "widening" mul returning (low, high) bits
    pub fn widening_mul(&self, rhs: &Self) -> (Self, Self) {
        let mut res = vec![0u32; NUM_LIMBS * 2];
        let base: u64 = Self::LIMB_SIZE as u64 + 1;

        for i in 0..NUM_LIMBS {
            let mut carry = 0u64;
            for j in 0..NUM_LIMBS {
                // replace with some kind of mul add
                let prod = carry + (self.limbs[i] as u64 * rhs.limbs[j] as u64) + res[i + j] as u64;

                carry = prod / base;
                let prod_res = prod % base;

                res[i + j] = prod_res as u32;
            }

            res[i + NUM_LIMBS] = carry as u32;
        }

        // split the arr into low and high
        let lo = Self::from_slice(&res[0..NUM_LIMBS]);
        let hi = Self::from_slice(&res[NUM_LIMBS..2 * NUM_LIMBS]);

        (lo, hi)
    }

    pub fn _mul(&self, rhs: &Self) -> Self {
        self.widening_mul(rhs).0
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::{Bu64, Bu256};

    #[test]
    fn basic_mul() {
        let a = Bu256::from_u128(5);
        let b = Bu256::from_u128(5);
        assert_eq!(
            a.widening_mul(&b),
            (Bu256::from_u128(25), Bu256::from_u128(0))
        );
    }

    #[test]
    fn mul_cross_limbs() {
        let a = Bu256::from_u128(u32::MAX as u128);
        let b = Bu256::from_u128(4);
        assert_eq!(
            a.widening_mul(&b),
            (Bu256::from_u128(u32::MAX as u128 * 4), Bu256::from_u128(0))
        );
    }

    #[test]
    fn mul_with_carry() {
        let a = Bu64::from_slice(&[0xffff_ffff, 0xffff_ffff]);
        let b = Bu64::from_slice(&[4]);
        assert_eq!(
            a.widening_mul(&b),
            (
                Bu64::from_slice(&[0xffff_fffc, 0xffff_ffff]), // low
                Bu64::from_slice(&[3])                         // high
            )
        );
    }
}
