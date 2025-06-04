use super::BigUint;

impl<const NUM_LIMBS: usize> BigUint<NUM_LIMBS> {
    pub fn modulo(&self, modulus: &Self) -> Self {
        self.div_rem(modulus).1
    }
}
