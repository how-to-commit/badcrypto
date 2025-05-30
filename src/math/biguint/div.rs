use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    /// eompute self / rhs, returning tuple (quotient, remainder)
    /// if rhs is zero, returns (0, self)
    /// if self is zero, returns (0, 0)
    pub(crate) fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let divisor_safe = !rhs.is_zero() as u32;

        let mut quotient = Self::zero();
        let mut remainder = Self::zero();
        let divisor = Self::ct_select(rhs, &Self::one(), divisor_safe);

        // restoring divide
        for bit_pos in (0..NUM_LIMBS * Self::LIMB_SIZE_BITS).rev() {
            remainder = remainder.unbounded_shl(1);

            // set LSB of remainder to current bit of dividend
            let limb_idx = bit_pos / Self::LIMB_SIZE_BITS;
            let bit_idx = bit_pos % Self::LIMB_SIZE_BITS;
            let dividend_bit = (self.limbs[limb_idx] >> bit_idx) & 1;
            remainder.limbs[0] |= dividend_bit;

            let (temp, borrow) = remainder.borrowing_sub(&divisor, 0);

            // if remainder >= divisor, use subtraction result
            let no_borrow = (borrow ^ 1) as u32;
            remainder = Self::ct_select(&remainder, &temp, no_borrow);

            // set bit in quotient if subtraction succeeded
            let q_limb_idx = bit_pos / 32;
            let q_bit_idx = bit_pos % 32;
            quotient.limbs[q_limb_idx] |= no_borrow << q_bit_idx;
        }

        // flip the things around if not divisor is zero.
        remainder = Self::ct_select(&remainder, &quotient, divisor_safe);
        quotient = Self::ct_select(&quotient, &Self::zero(), divisor_safe);

        (quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::Bu64;

    #[test]
    fn divide() {
        // small
        let s_op1 = Bu64::from_slice(&[10]);
        let s_op2 = Bu64::from_slice(&[5]);
        assert_eq!(
            s_op1.div_rem(&s_op2),
            (Bu64::from_slice(&[2]), Bu64::from_slice(&[0]))
        );

        // large
        let l_op1 = Bu64::from_slice(&[0xFFFF_FFFF, 0x1]);
        let s_op3 = Bu64::from_slice(&[0xF]);

        assert_eq!(
            l_op1.div_rem(&s_op3),
            (Bu64::from_slice(&[0x2222_2222]), Bu64::from_slice(&[1]))
        );

        // both large, small result
        let l_op2 = Bu64::from_slice(&[0xFFFF_FF77, 0xFFFF]);
        let l_op3 = Bu64::from_slice(&[0xFFFF_FF77]);

        assert_eq!(
            l_op2.div_rem(&l_op3),
            (Bu64::from_slice(&[0x10000]), Bu64::from_slice(&[0x88ff77]))
        );
    }

    #[test]
    fn divide_denominator_larger() {
        let op1 = Bu64::from_slice(&[5]);
        let op2 = Bu64::from_slice(&[10]);
        assert_eq!(
            op1.div_rem(&op2),
            (Bu64::from_slice(&[0]), Bu64::from_slice(&[5]))
        );
    }

    #[test]
    fn divide_by_zero() {
        let op1 = Bu64::from_slice(&[0xFFFF_1234, 0xABCD]);
        let op2 = Bu64::from_slice(&[0]);
        assert_eq!(
            op1.div_rem(&op2),
            (
                Bu64::from_slice(&[0]),
                Bu64::from_slice(&[0xFFFF_1234, 0xABCD])
            )
        );
    }
}
