use std::cmp;

use super::BigUint;

impl<const NUM_LIMBS: usize> PartialEq for BigUint<NUM_LIMBS> {
    /// attempted constant-time comparison
    fn eq(&self, other: &Self) -> bool {
        let mut res = true;
        for i in 0..NUM_LIMBS {
            if self.limbs[i] != other.limbs[i] {
                res = false;
            }
        }
        res
    }
}

impl<const NUM_LIMBS: usize> Eq for BigUint<NUM_LIMBS> {}

impl<const NUM_LIMBS: usize> Ord for BigUint<NUM_LIMBS> {
    /// attempted constant-time comparison
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let mut gt = 0u32;
        let mut lt = 0u32;

        for i in (0..NUM_LIMBS).rev() {
            let a = self.limbs[i];
            let b = other.limbs[i];

            gt |= ((a > b) as u32) & !gt & !lt;
            lt |= ((a < b) as u32) & !gt & !lt;
        }

        match (gt, lt) {
            (1, 0) => cmp::Ordering::Greater,
            (0, 1) => cmp::Ordering::Less,
            (_, _) => cmp::Ordering::Equal,
        }
    }
}

impl<const NUM_LIMBS: usize> PartialOrd for BigUint<NUM_LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
