#[inline]
pub(crate) fn pad_trailing(slice: &mut Vec<u8>, pad_with: u8, pad_to_len: usize) {
    if slice.len() > pad_to_len {
        return;
    }

    for _ in slice.len()..pad_to_len {
        slice.push(pad_with);
    }
}

#[inline]
pub(crate) fn prepend(slice: &mut Vec<u8>, mut prepend_with: Vec<u8>) {
    slice.splice(..0, prepend_with.drain(..));
}

#[inline]
pub(crate) fn u8_to_hexstr(slice: &[u8]) -> String {
    slice
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
