use crate::math::arith::carrying_add;

#[derive(Debug, PartialEq, Eq)]
pub struct BigUint {
    limbs: [u32; Self::NUM_LIMBS],
}

impl BigUint {
    // 64 limbs * 32 bits = 2048 bits
    const NUM_LIMBS: usize = 64;
    const LIMB_SIZE: u32 = u32::MAX;

    pub fn from_slice(val: &[u32]) -> Self {
        if val.len() > Self::NUM_LIMBS {
            panic!("Attempt to create BigUint with overflow.");
        }

        let mut ret = BigUint {
            limbs: [0; Self::NUM_LIMBS],
        };
        ret.limbs[0..val.len()].copy_from_slice(&val);
        ret
    }

    pub fn from_u128(val: u128) -> Self {
        let mut ret = BigUint {
            limbs: [0; Self::NUM_LIMBS],
        };
        let mut v = val;

        for i in 0..4 {
            ret.limbs[i] = (v & (u32::MAX as u128)) as u32;
            v >>= 32;
        }

        ret
    }

    pub fn carrying_add(&self, rhs: &Self, mut carry: u32) -> (Self, u32) {
        let mut ret = BigUint {
            limbs: [0; Self::NUM_LIMBS],
        };
        for i in 0..Self::NUM_LIMBS {
            (ret.limbs[i], carry) = carrying_add(self.limbs[i], rhs.limbs[i], carry);
        }
        (ret, carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        assert_eq!(u32::MAX, 0xFFFF_FFFF);

        let huge_number = 5;
        let fu128 = BigUint::from_u128(huge_number as u128);
        let fslice = BigUint::from_slice(&[huge_number]);
        assert_eq!(fu128, fslice);

        let big_fu128 = BigUint::from_u128(0x1234_5678_abcd_ef00);
        let big_fslice = BigUint::from_slice(&[0xabcd_ef00, 0x1234_5678]);
        assert_eq!(big_fu128, big_fslice);
    }

    // #[test]
    // fn carrying_add_no_carry() {
    //     // assert_eq!()
    //     todo!()
    // }
}
