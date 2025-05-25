mod add;
mod modulo;
mod sub;

#[derive(Debug, PartialEq, Eq)]
pub struct BigUint {
    pub(crate) limbs: [u32; Self::NUM_LIMBS],
}

impl BigUint {
    // 64 limbs * 32 bits = 2048 bits
    pub const NUM_LIMBS: usize = 64;
    pub const LIMB_SIZE: u32 = u32::MAX;

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

    pub fn ct_select(a: &Self, b: &Self, choice: u32) -> Self {
        let mask = 0u32.wrapping_sub(choice);
        let mut res = Self::zero();

        for i in 0..Self::NUM_LIMBS {
            res.limbs[i] = (a.limbs[i] & !mask) | (b.limbs[i] & mask);
        }

        res
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
}
