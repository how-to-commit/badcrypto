use crate::math::biguint::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
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
        let upper_bound = upper.leading_zeros();
        loop {
            let result = Self::rand_bits(upper_bound as usize, rng);
            if result < *upper && result > *lower {
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::Bu512;
    use rand::rng;

    #[test]
    fn test_rand_gen() {
        let mut r = rng();
        Bu512::rand_bits(33, &mut r);
    }
}
