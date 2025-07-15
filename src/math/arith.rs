/// performs lhs + rhs + carry, returning (result, carry)
/// carry can be 0, 1, or 2
#[inline]
pub(crate) fn carrying_add(lhs: u32, rhs: u32, carry: u32) -> (u32, u32) {
    let (interim_sum, overflow1) = lhs.overflowing_add(rhs);
    let (res_sum, overflow2) = interim_sum.overflowing_add(carry);
    let res_overflow = (overflow1 as u32) + (overflow2 as u32);
    (res_sum, res_overflow)
}

/// lhs - rhs - borrow, returning (result, borrow)
///
///    1 0                     lhs rhs cry  res cry
/// \-   9    =>   borrowing_sub(0, 9, 0) = (1, 1)
///  ------        borrowing_sub(1, 0, 1) = (0, 0)
///    0 1         res = 01
#[inline]
pub(crate) fn borrowing_sub(lhs: u32, rhs: u32, borrow: u32) -> (u32, u32) {
    let (interim_res, borrow1) = lhs.overflowing_sub(rhs);
    let (res_sum, borrow2) = interim_res.overflowing_sub(borrow);
    let res_overflow = (borrow1 as u32) + (borrow2 as u32);
    (res_sum, res_overflow)
}

#[inline]
pub(crate) fn carrying_mul(lhs: u32, rhs: u32, carry: u32) -> (u32, u32) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(carrying_add(5, 10, 0), (15, 0));
        assert_eq!(carrying_add(5, 10, 1), (16, 0));
        assert_eq!(carrying_add(u32::MAX, 10, 1), (10, 1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(borrowing_sub(10, 5, 0), (5, 0));
        assert_eq!(borrowing_sub(5, 10, 1), (u32::MAX - 5, 1));
    }
}
