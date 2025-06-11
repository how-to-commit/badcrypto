use std::ops::Mul;

use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    /// Calculate x^n with Montgomery's Ladder
    pub fn pow_mod(&self, p: &Self, n: &Self) -> Self {
        let mut x1 = Self::one();
        let mut x2 = self.clone();
        let len = Self::LIMB_SIZE_BITS * NUM_LIMBS;

        for i in (0..len - 1).rev() {
            if (p.limbs[i / 32] >> (i % 32)) & 1 == 0 {
                x2 = (&x1 * &x2) % n;
                x1 = (&x1 * &x1) % n;
            } else {
                x1 = (&x1 * &x2) % n;
                x2 = (&x2 * &x2) % n;
            }
        }

        x1
    }

    pub fn pow_mod_innerwide<const WIDE: usize>(&self, exp: &Self, modulus: &Self) -> Self {
        let mut result = BigUint::one();
        let mut base = self % modulus;

        for limb_idx in 0..NUM_LIMBS {
            let exp_limb = exp.limbs[limb_idx];

            for bit_pos in 0..32 {
                let bit = (exp_limb >> bit_pos) & 1;
                let temp_result = result.mul_mod::<WIDE>(&base, modulus);

                for i in 0..NUM_LIMBS {
                    let mask = (bit as u32).wrapping_neg(); // 0xFFFFFFFF if bit=1, 0x00000000 if bit=0
                    result.limbs[i] = (mask & temp_result.limbs[i]) | (!mask & result.limbs[i]);
                }

                // Square the base for next iteration
                base = base.mul_mod::<WIDE>(&base, modulus);
            }
        }

        result
    }

    pub fn pow(&self, p: &Self) -> Self {
        let mut x1 = Self::one();
        let mut x2 = self.clone();
        let len = Self::LIMB_SIZE_BITS * NUM_LIMBS;

        for i in (0..len - 1).rev() {
            if (p.limbs[i / 32] >> (i % 32)) & 1 == 0 {
                x2 = &x1 * &x2;
                x1 = &x1 * &x1;
            } else {
                x1 = &x1 * &x2;
                x2 = &x2 * &x2;
            }
        }

        x1
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::Bu256;

    #[test]
    fn test_pow_basic() {
        assert_eq!(
            Bu256::from_u128(5).pow(&Bu256::from_u128(2)),
            Bu256::from_u128(25)
        );
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(
            Bu256::from_u128(5).pow_mod(&Bu256::from_u128(2), &Bu256::from_u128(10)),
            Bu256::from_u128(5)
        );
    }
}
