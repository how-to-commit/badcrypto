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
}

#[cfg(test)]
mod tests {
    use crate::math::biguint::Bu256;

    #[test]
    fn prime_basic() {
        assert_eq!(Bu256::from_slice(&[1234]).is_prime(), false);
        assert_eq!(Bu256::from_slice(&[877]).is_prime(), true);
        println!("hey");
        assert_eq!(
            Bu256::from_slice(&[0xfd8d_ae87, 0xde9f_786b, 0x7472_2e1e, 0x1ae3_c3bb]).is_prime(),
            false
        );
    }
}
