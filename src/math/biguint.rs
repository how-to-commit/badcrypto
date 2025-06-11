// big uint.
// limbs are [u32; num_limbs (generic)], stored little-endian
// type aliases are defined for 160 bits, 256 bits, 512 bits, 1024 bits

use std::fmt::Debug;

mod add;
mod bits;
mod cmp;
mod div;
mod modulo;
mod mul;
mod ops;
mod pow;
mod rand;
mod sqrt;
mod sub;
mod utils;

#[derive(Clone)]
pub struct BigUint<const NUM_LIMBS: usize> {
    pub limbs: [u32; NUM_LIMBS],
}

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    pub const LIMBS: usize = NUM_LIMBS;
    pub const LIMB_SIZE: u32 = u32::MAX;
    pub const LIMB_SIZE_BITS: usize = 32;

    pub const fn zero() -> Self {
        Self {
            limbs: [0; NUM_LIMBS],
        }
    }

    pub const fn one() -> Self {
        let mut s = Self {
            limbs: [0; NUM_LIMBS],
        };
        s.limbs[0] = 1;
        s
    }

    pub const fn max() -> Self {
        Self {
            limbs: [Self::LIMB_SIZE; NUM_LIMBS],
        }
    }

    pub fn from_slice(val: &[u32]) -> Self {
        if val.len() > NUM_LIMBS {
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

    /// Select between 2 BigUints in constant-time:
    /// if choice == 1, then choose b, else a
    /// choice > 1 is invalid input
    pub fn ct_select(a: &Self, b: &Self, choice: u32) -> Self {
        let mask = 0u32.wrapping_sub(choice);
        let mut res = Self::zero();

        for i in 0..NUM_LIMBS {
            res.limbs[i] = (a.limbs[i] & !mask) | (b.limbs[i] & mask);
        }

        res
    }

    /// Swap 2 BigUints in constant-time:
    /// if choice == 1, then swap, else no swap
    /// choice > 1 is invalid input
    pub fn ct_swap(a: &mut Self, b: &mut Self, choice: u32) {
        let mask = 0u32.wrapping_sub(choice);

        for i in 0..NUM_LIMBS {
            let t = (a.limbs[i] ^ b.limbs[i]) & mask;
            a.limbs[i] = a.limbs[i] ^ t;
            b.limbs[i] = b.limbs[i] ^ t;
        }
    }

    pub fn is_zero(&self) -> bool {
        for i in self.limbs.iter() {
            if *i != 0 {
                return true;
            }
        }
        false
    }

    pub fn leading_zeros(&self) -> u32 {
        let mut result = 0u32;
        let mut done = 0u32;

        for i in (0..NUM_LIMBS).rev() {
            let is_zero = (self.limbs[i] == 0) as u32; // 1 if all are zeros
            let not_done = !done & 1; // 1 if yet to encounter a nonzero limb

            // if is_zero && not_done
            result += 32 * is_zero * not_done;

            // if !is_zero && not_done
            let leading_zeros = self.limbs[i].leading_zeros();
            result += leading_zeros * (1 - is_zero) * not_done;

            // set done to !is_zero
            done |= 1 - is_zero;
        }

        result
    }
}

impl<const N: usize> Debug for BigUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BigUint<{N}>: ")?;
        for limb in self.limbs.iter().rev() {
            write!(f, "{limb:08x} ")?;
        }
        write!(f, "")
    }
}

// common types:
pub type Bu256 = BigUint<8>; // 8 * 32 = 160
pub type Bu512 = BigUint<16>; // 16 * 32 = 512

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        assert_eq!(u32::MAX, 0xFFFF_FFFF); // sanity

        let huge_number = 5;
        let fu128 = Bu512::from_u128(huge_number as u128);
        let fslice = Bu512::from_slice(&[huge_number]);
        assert_eq!(fu128, fslice);

        let big_fu128 = Bu512::from_u128(0x1234_5678_abcd_ef00);
        let big_fslice = Bu512::from_slice(&[0xabcd_ef00, 0x1234_5678]);
        assert_eq!(big_fu128, big_fslice);
    }

    #[test]
    fn leading_zeros() {
        let x = Bu512::from_slice(&[0xFF00_0000]); // 512 - 32 = 480 trailing zeros
        assert_eq!(x.leading_zeros(), 480);

        let x = Bu512::from_slice(&[0xa9]); // 512 - 8 = 56 trailing zeros
        assert_eq!(x.leading_zeros(), 504);
    }

    #[test]
    fn ct_swap_basic() {
        let ra = Bu512::from_u128(1234);
        let rb = Bu512::from_u128(48);
        let mut a = Bu512::from_u128(48);
        let mut b = Bu512::from_u128(1234);
        Bu512::ct_swap(&mut a, &mut b, 1);
        assert_eq!(a, ra);
        assert_eq!(b, rb);
    }
}
