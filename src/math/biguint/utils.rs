use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    pub fn is_prime(&self) -> bool {
        let mut i = Self::from_slice(&[0]);
        let limit = self.sqrt();

        // test i = 2, i = 3
        if self.modulo(&Self::from_slice(&[2])) == BigUint::zero() {
            return false;
        }

        if self.modulo(&Self::from_slice(&[3])) == BigUint::zero() {
            println!("trigger1");
            return false;
        }

        // test all i = 6k+1, i = 6k+5, for i < sqrt(n)
        while i < limit {
            i = i.carrying_add(&Self::from_slice(&[6]), 0).0;

            if self.modulo(&i.carrying_add(&Self::from_slice(&[1]), 0).0) == BigUint::zero() {
                println!("trigger");
                return false;
            }
            if self.modulo(&i.carrying_add(&Self::from_slice(&[5]), 0).0) == BigUint::zero() {
                println!("trigger2");
                return false;
            }
        }

        return true;
    }

    /// Use the Miller-Rabin Probabilistic Prime Test, as prescribed by FIPS 186-5
    pub fn probably_prime<T>(&self, iter: usize, rng: &mut T) -> bool
    where
        T: rand::CryptoRng,
    {
        let one = Self::one();
        let two = Self::from_u128(2);

        let mut a = 0;
        let even_w = self - &one;
        let mut m = even_w.clone();

        while &m % &two == Self::zero() {
            m = m >> 1;
            a += 1;
        }

        let w_len = self.num_bits();
        for _ in 0..iter {
            let b: Self = loop {
                let candidate = Self::rand_bits(w_len, rng);
                if candidate >= one && candidate < even_w {
                    break candidate;
                }
            };

            let mut z = b.pow_mod(&m, self);
            if z != one && z != even_w {
                let mut composite = true;

                for _ in 1..a {
                    z = z.pow_mod(&two, self);
                    if z == one {
                        return false;
                    } else if z == even_w {
                        composite = false;
                        break;
                    }
                }

                if composite {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::math::biguint::Bu256;

    #[test]
    fn prime_basic() {
        assert_eq!(Bu256::from_slice(&[1234]).is_prime(), false);
        assert_eq!(Bu256::from_slice(&[877]).is_prime(), true);
        println!("hey");
    }

    #[test]
    fn ml_bignum() {
        let mut rng = rand::rngs::StdRng::from_os_rng();
        assert_eq!(
            Bu256::from_u128(5633922075003977699492159071).probably_prime(32, &mut rng),
            true
        );

        assert_eq!(
            Bu256::from_slice(&[0xcccc_dddd, 0xaaaa_bbbb, 0x1234_4567])
                .probably_prime(32, &mut rng),
            false
        );
    }
}
