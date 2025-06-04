// // its really difficult to represent this with a struct that has a variable size for its internal
// // associated biguint type, so we'll just use a function generic over biguint types.
//
// use super::biguint::BigUint;
//
// fn generate_superincreasing<const S: usize>(
//     len: usize,
//     target_sum: BigUint<{ S }>,
// ) -> Vec<BigUint<{ S }>> {
//     let a = BigUint<S>::zero();
//
//     let seq = vec![Num; len];
//
//     todo!()
// }
