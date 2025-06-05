use std::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};

use super::BigUint;

macro_rules! biguint_impl_binops {
    (impl $trait:ident for $t:ty, $method:ident with $internal:ident) => {
        impl<const N: usize> std::ops::$trait for $t {
            type Output = $t;
            fn $method(self, rhs: $t) -> $t {
                self.$internal(&rhs)
            }
        }

        impl<'a, const N: usize> std::ops::$trait<$t> for &'a $t {
            type Output = $t;
            fn $method(self, rhs: $t) -> $t {
                (*self).$internal(&rhs)
            }
        }

        impl<'a, const N: usize> std::ops::$trait<&'a $t> for $t {
            type Output = $t;
            fn $method(self, rhs: &'a $t) -> $t {
                self.$internal(rhs)
            }
        }

        impl<'a, 'b, const N: usize> std::ops::$trait<&'a $t> for &'b $t {
            type Output = $t;
            fn $method(self, rhs: &'a $t) -> $t {
                (*self).$internal(rhs)
            }
        }
    };
}

biguint_impl_binops!(impl Add for BigUint<N>, add with _add);
biguint_impl_binops!(impl Sub for BigUint<N>, sub with _sub);
biguint_impl_binops!(impl Mul for BigUint<N>, mul with _mul);
biguint_impl_binops!(impl Div for BigUint<N>, div with _div);
biguint_impl_binops!(impl Rem for BigUint<N>, rem with modulo);

impl<const N: usize> Shl<u32> for BigUint<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        self.unbounded_shl(rhs)
    }
}

impl<const N: usize> Shr<u32> for BigUint<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        self.unbounded_shr(rhs)
    }
}
