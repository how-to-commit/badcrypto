use super::biguint::BigUint;

pub fn monty_ladder<const T: usize>(
    xp: &BigUint<T>,
    n: &BigUint<T>,
    field: &BigUint<T>,
    a24: &BigUint<T>,
) -> BigUint<T> {
    let x1 = xp.clone();
    let mut x2 = BigUint::<T>::one();
    let mut z2 = BigUint::<T>::zero();
    let mut x3 = xp.clone();
    let mut z3 = BigUint::<T>::one();

    let limbsz = BigUint::<T>::LIMB_SIZE_BITS;
    let bitlen = xp.num_bits();
    let mut prevbit = 0u32;

    for i in (0..bitlen).rev() {
        let bit = n.limbs[i / limbsz] >> (i % limbsz);
        let swap = bit ^ prevbit;
        prevbit = bit;

        BigUint::<T>::ct_swap(&mut x2, &mut x3, swap);
        BigUint::<T>::ct_swap(&mut z2, &mut z3, swap);

        let (x2_new, z2_new, x3_new, z3_new) = ladder_step(&x2, &z2, &x3, &z3, &x1, field, a24);
        x2 = x2_new;
        z2 = z2_new;
        x3 = x3_new;
        z3 = z3_new;
    }

    let z2_inv = gf_inverse(&z2, field);
    (x2 * z2_inv) % field
}

fn gf_inverse<const T: usize>(x: &BigUint<T>, p: &BigUint<T>) -> BigUint<T> {
    x.pow_mod(&(p - BigUint::<T>::from_u128(2)), p)
}

#[rustfmt::skip] // the variables are grouped in pairs.
fn ladder_step<const T: usize>(
    x2: &BigUint<T>, z2: &BigUint<T>,
    x3: &BigUint<T>, z3: &BigUint<T>,
    x1: &BigUint<T>,
    p: &BigUint<T>, a24: &BigUint<T>
) -> (BigUint<T>, BigUint<T>, BigUint<T>, BigUint<T>) {
    let two = BigUint::<T>::one() + BigUint::<T>::one();

    let t1 = x2 + z2;
    let t2 = x2 - z2;
    let t3 = x3 + z3;
    let t4 = x3 - z3;

    let t5 = &t1 * &t4;
    let t6 = &t2 * &t3;

    let x5 = (&t5 + &t6).pow(&two);
    let z5 = (&t5 - &t6).pow(&two);
    let z5 = &z5 * x1;

    let t7 = t1.pow(&two);
    let t8 = t2.pow(&two);
    let x4 = &t7 * &t8;
    let t9 = &t7 - &t8;
    let z4 = &t9 * a24;
    let z4 = &z4 + &t8;
    let z4 = &z4 * &t9;

    (x4 % p, z4 % p, x5 % p, z5 % p)
}
