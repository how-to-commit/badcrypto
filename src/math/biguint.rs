// big uint.
// limbs are [u32; num_limbs (generic)], stored little-endian
// type aliases are defined for 160 bits, 256 bits, 512 bits, 1024 bits

mod add;
mod bits;
mod cmp;
mod div;
mod modulo;
mod mul;
mod sub;

#[derive(Debug)]
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

    pub fn ct_select(a: &Self, b: &Self, choice: u32) -> Self {
        let mask = 0u32.wrapping_sub(choice);
        let mut res = Self::zero();

        for i in 0..NUM_LIMBS {
            res.limbs[i] = (a.limbs[i] & !mask) | (b.limbs[i] & mask);
        }

        res
    }

    pub fn is_zero(&self) -> bool {
        for i in self.limbs.iter() {
            if *i != 0 {
                return true;
            }
        }
        false
    }

    pub fn rand_bits<T>(bits_to_generate: usize, rng: &mut T) -> Self
    where
        T: rand::Rng,
    {
        let mut limbs = [0u32; NUM_LIMBS];
        let chunks = bits_to_generate / 32;
        let rem = bits_to_generate % 32;

        // generate in blocks of 32 bits
        for i in 0..chunks {
            limbs[i] = rng.next_u32();
        }

        let bits = 1 << rem;
        // finish by generating the last few bits
        limbs[chunks] = rng.random_range(0..bits);

        Self { limbs }
    }

    pub fn rand_between<T>(lower: &Self, upper: &Self, rng: &mut T) -> Self
    where
        T: rand::Rng,
    {
        todo!()
    }
}

// common types:
pub type Bu64 = BigUint<2>; // for testing!
pub type Bu160 = BigUint<5>; // 5 * 32 = 160
pub type Bu256 = BigUint<8>; // 8 * 32 = 160
pub type Bu512 = BigUint<16>; // 16 * 32 = 512
pub type Bu1024 = BigUint<32>; // 32 * 32 = 1024

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
    fn test_rand_gen() {
        let mut r = rand::rng();
        Bu512::rand_bits(33, &mut r);
    }
}
