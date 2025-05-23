#[inline]
pub(crate) fn carrying_add(lhs: u32, rhs: u32, carry: u32) -> (u32, u32) {
    let (interim_sum, overflow1) = lhs.overflowing_add(rhs);
    let (res_sum, overflow2) = interim_sum.overflowing_add(carry);
    let res_overflow = (overflow1 as u32) + (overflow2 as u32);
    (res_sum, res_overflow)
}
