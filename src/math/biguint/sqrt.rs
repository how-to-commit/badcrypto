use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    pub fn sqrt(&self) -> Self {
        // TODO: consider implementing a faster algorithm?
        let num_bits = Self::LIMB_SIZE_BITS as u32 * NUM_LIMBS as u32;
        let self_bitlen = num_bits - self.leading_zeros();
        let num_iter = num_bits.ilog2();

        let mut x = self.unbounded_shr(self_bitlen / 2);
        for _ in 0..=num_iter {
            // x_n+1 = (x_n + S/x_n) / 2
            x = x.carrying_add(&self.div_rem(&x).0, 0).0.unbounded_shr(1);
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::{BigUint, Bu256};

    #[test]
    fn sqrt_edge_cases() {
        assert_eq!(Bu256::zero().sqrt(), Bu256::zero());
        assert_eq!(Bu256::one().sqrt(), Bu256::one());
    }

    #[test]
    fn sqrt_basic() {
        assert_eq!(Bu256::from_slice(&[169]).sqrt(), Bu256::from_slice(&[13]));
        assert_eq!(Bu256::from_slice(&[144]).sqrt(), Bu256::from_slice(&[12]));
        assert_eq!(Bu256::from_slice(&[10]).sqrt(), Bu256::from_slice(&[3]));
    }

    #[test]
    fn sqrt2048() {
        let a = BigUint::<128>::from_slice(&[0xFFFF_FFFF; 64]).sqrt();
        println!("{a:#?}");
    }
}
