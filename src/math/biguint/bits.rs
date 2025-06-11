use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    pub fn num_bits(&self) -> usize {
        (Self::LIMB_SIZE_BITS * NUM_LIMBS) - self.leading_zeros() as usize
    }

    pub fn get_bit(&self, bit: usize) -> u32 {
        let limb_index = bit / 32;
        let bit_offset = bit % 32;

        (self.limbs[limb_index] >> bit_offset) & 1
    }

    pub fn bitand(&self, mask: &Self) -> Self {
        let mut res = Self::zero();
        for i in 0..NUM_LIMBS {
            res.limbs[i] = self.limbs[i] & mask.limbs[i];
        }
        res
    }

    pub fn bitnot(&self) -> Self {
        let mut res = Self::zero();
        for i in 0..NUM_LIMBS {
            res.limbs[i] = !self.limbs[i];
        }
        res
    }

    pub fn bitor(&self, rhs: &Self) -> Self {
        let mut res = Self::zero();
        for i in 0..NUM_LIMBS {
            res.limbs[i] = self.limbs[i] | rhs.limbs[i];
        }
        res
    }

    pub fn bitxor(&self, rhs: &Self) -> Self {
        let mut res = Self::zero();
        for i in 0..NUM_LIMBS {
            res.limbs[i] = self.limbs[i] ^ rhs.limbs[i];
        }
        res
    }

    /// performs right shift, returning result
    /// !: variable time with respect to shift
    pub fn unbounded_shr(&self, shift: u32) -> Self {
        let mut res = Self::zero();

        let inner_shift = shift % 32;
        let block_shift = shift / 32;

        for i in 0..NUM_LIMBS {
            let op_idx = i + (block_shift as usize);
            if op_idx >= NUM_LIMBS {
                break;
            }

            let lower = self.limbs[op_idx];
            let upper = if op_idx + 1 < NUM_LIMBS {
                self.limbs[op_idx + 1]
            } else {
                0
            };

            res.limbs[i] = (lower >> inner_shift) | (upper.unbounded_shl((32 - inner_shift) as u32))
        }
        res
    }

    /// performs left shift, returning (result, overflow bits)
    /// !: variable time with respect to shift
    pub fn unbounded_shl(&self, shift: u32) -> Self {
        let mut res = Self::zero();

        let inner_shift = shift % 32;
        let block_shift = shift / 32;

        for i in (0..NUM_LIMBS).rev() {
            if i < block_shift as usize {
                break;
            }

            let op_idx = i - (block_shift as usize);
            let lower = self.limbs[op_idx];
            let upper = if op_idx >= 1 {
                self.limbs[op_idx - 1]
            } else {
                0
            };

            res.limbs[i] = (lower << inner_shift) | (upper.unbounded_shr((32 - inner_shift) as u32))
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::*;

    #[test]
    fn shift_right() {
        let op1 = Bu256::from_slice(&[0x12_34_56_78]);
        assert_eq!(op1.unbounded_shr(17), Bu256::from_slice(&[0x91a]));
    }

    #[test]
    fn shift_right_when_zero() {
        let op1 = Bu256::from_slice(&[0x12_34_56_78]);
        assert_eq!(op1.unbounded_shr(0), Bu256::from_slice(&[0x12_34_56_78]));
    }

    #[test]
    fn shift_right_with_shift_gt_len() {
        let op1 = Bu256::from_slice(&[0x12_34_56_78]);
        assert_eq!(op1.unbounded_shr(200), Bu256::from_slice(&[0]));
    }

    #[test]
    fn shift_right_across_limbs() {
        let op1 = Bu256::from_slice(&[0xde_ad_be_ef, 0x12_34_56_78]);
        assert_eq!(op1.unbounded_shr(49), Bu256::from_slice(&[0x91a]));
    }

    #[test]
    fn shift_left() {
        let op1 = Bu256::from_slice(&[0x2_34_56_78]);
        assert_eq!(op1.unbounded_shl(4), Bu256::from_slice(&[0x23_45_67_80]));
    }

    #[test]
    fn shift_left_when_zero() {
        let op1 = Bu256::from_slice(&[0x12_34_56_78]);
        assert_eq!(op1.unbounded_shl(0), Bu256::from_slice(&[0x12_34_56_78]));
    }

    #[test]
    fn shift_left_with_shift_gt_len() {
        let op1 = Bu256::from_slice(&[0x12_34_56_78]);
        assert_eq!(op1.unbounded_shl(20000), Bu256::from_slice(&[0]));
    }

    #[test]
    fn shift_left_across_limbs() {
        let op1 = Bu256::from_slice(&[0x1234]);
        assert_eq!(
            op1.unbounded_shl(23),
            Bu256::from_slice(&[0x1a00_0000, 0x9])
        );
    }
}
