/// performs lhs + rhs + carry, returning (result, carry)
/// carry can be 0, 1, or 2
#[inline]
pub(crate) fn carrying_add(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
    let (interim_sum, overflow1) = lhs.overflowing_add(rhs);
    let (res_sum, overflow2) = interim_sum.overflowing_add(carry);
    let res_overflow = (overflow1 as u64) + (overflow2 as u64);
    (res_sum, res_overflow)
}

/// lhs - rhs - borrow, returning (result, borrow)
///
///    1 0                     lhs rhs cry  res cry
/// \-   9    =>   borrowing_sub(0, 9, 0) = (1, 1)
///  ------        borrowing_sub(1, 0, 1) = (0, 0)
///    0 1         res = 01
#[inline]
pub(crate) fn borrowing_sub(lhs: u64, rhs: u64, borrow: u64) -> (u64, u64) {
    let (interim_res, borrow1) = lhs.overflowing_sub(rhs);
    let (res_sum, borrow2) = interim_res.overflowing_sub(borrow);
    let res_overflow = (borrow1 as u64) + (borrow2 as u64);
    (res_sum, res_overflow)
}

// performs lhs * rhs, returning (lo, hi)
// waiting for bigint_helper_methods to stabilize to replace this method.
#[inline]
pub(crate) fn widening_mul(lhs: u64, rhs: u64) -> (u64, u64) {
    let temp = (lhs as u128) * (rhs as u128);
    return ((temp & 0xFFFF_FFFF_FFFF_FFFF) as u64, (temp << 64) as u64);
}

// constant-time selector.
// if is_x is 1, then return x, else return y
// is_x should be either 1 or 0
pub(crate) fn ct_select_64(x: u64, y: u64, is_x: u64) -> u64 {
    assert!(
        is_x == 0 || is_x == 1,
        "is_x should be 0 (representing false) or 1 (representing true)."
    );
    let mask = is_x.wrapping_sub(1);
    return (!mask & x) | mask & y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(carrying_add(5, 10, 0), (15, 0));
        assert_eq!(carrying_add(5, 10, 1), (16, 0));
        assert_eq!(carrying_add(u64::MAX, 10, 1), (10, 1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(borrowing_sub(10, 5, 0), (5, 0));
        assert_eq!(borrowing_sub(5, 10, 1), (u64::MAX - 5, 1));
    }

    #[test]
    fn test_cts64() {
        assert_eq!(ct_select_64(10, 5, 0), 5);
        assert_eq!(ct_select_64(10, 5, 1), 10);
    }
}
