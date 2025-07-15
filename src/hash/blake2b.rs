use crate::slice::{pad_trailing, prepend, u8_to_hexstr};

use super::HashFunction;

const BLAKE2B_IV: [u64; 8] = [
    0x6a09e667f3bcc908, // Frac(sqrt(2))
    0xbb67ae8584caa73b, // Frac(sqrt(3))
    0x3c6ef372fe94f82b, // Frac(sqrt(5))
    0xa54ff53a5f1d36f1, // Frac(sqrt(7))
    0x510e527fade682d1, // Frac(sqrt(11))
    0x9b05688c2b3e6c1f, // Frac(sqrt(13))
    0x1f83d9abfb41bd6b, // Frac(sqrt(17))
    0x5be0cd19137e2179, // Frac(sqrt(19))
];

const SIGMA: [[usize; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

pub struct Blake2b<'a> {
    hashlen: usize,
    message: Vec<u8>,
    key: Option<&'a [u8]>,
    state: [u64; 8],
}

impl<'a> Blake2b<'a> {
    /// Initialise a Blake2b hash function.
    /// hashlen is the length of the produced hash in bytes, up to 64.
    pub fn new(hashlen: usize) -> Self {
        if hashlen > 64 {
            panic!("Blake2b hash length greater than supported! {hashlen} > 64 (supported)");
        }

        Self {
            hashlen,
            message: vec![],
            key: None,
            state: BLAKE2B_IV,
        }
    }

    fn compress(&mut self, chunk: &[u8], compressed: usize, is_last_block: bool) {
        let mut v = [0; 16];
        v[..8].copy_from_slice(&self.state);
        v[8..].copy_from_slice(&BLAKE2B_IV);

        // TODO: how is it possible to address u128 addresses on a 64 bit system?
        v[12] ^= compressed as u64;
        // v[13] ^= High(compressed);

        v[14] ^= if is_last_block { !0 } else { 0 };

        let mut m = [0u64; 16];
        for i in 0..16 {
            for j in 0..8 {
                m[i] |= (chunk[i * 8 + j] as u64) << (8 * j);
            }
        }

        for i in 0..12 {
            Self::mix(&mut v, 0, 4, 8, 12, m[SIGMA[i][0]], m[SIGMA[i][1]]);
            Self::mix(&mut v, 1, 5, 9, 13, m[SIGMA[i][2]], m[SIGMA[i][3]]);
            Self::mix(&mut v, 2, 6, 10, 14, m[SIGMA[i][4]], m[SIGMA[i][5]]);
            Self::mix(&mut v, 3, 7, 11, 15, m[SIGMA[i][6]], m[SIGMA[i][7]]);
            Self::mix(&mut v, 0, 5, 10, 15, m[SIGMA[i][8]], m[SIGMA[i][9]]);
            Self::mix(&mut v, 1, 6, 11, 12, m[SIGMA[i][10]], m[SIGMA[i][11]]);
            Self::mix(&mut v, 2, 7, 8, 13, m[SIGMA[i][12]], m[SIGMA[i][13]]);
            Self::mix(&mut v, 3, 4, 9, 14, m[SIGMA[i][14]], m[SIGMA[i][15]]);
        }

        for i in 0..8 {
            self.state[i] ^= v[i] ^ v[i + 8];
        }
    }

    fn mix(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
        v[a] = v[a].overflowing_add(v[b]).0.overflowing_add(x).0;
        v[d] = (v[d] ^ v[a]).rotate_right(32);
        v[c] = v[c].overflowing_add(v[d]).0;
        v[b] = (v[b] ^ v[c]).rotate_right(24);
        v[a] = v[a].overflowing_add(v[b]).0.overflowing_add(y).0;
        v[d] = (v[d] ^ v[a]).rotate_right(16);
        v[c] = v[c].overflowing_add(v[d]).0;
        v[b] = (v[b] ^ v[c]).rotate_right(63);
    }
}

impl HashFunction for Blake2b<'_> {
    fn update(&mut self, message: &[u8]) {
        self.message.extend_from_slice(message);
    }

    fn digest(mut self) -> Vec<u8> {
        let mut bytes_compressed = 0;
        let mut bytes_remaining = self.message.len();

        let keylen = match self.key {
            Some(k) => k.len(),
            None => 0,
        };

        // xor state[0] with 0x0101_kkhh -> k = keylen, h = hashlen
        self.state[0] ^=
            0x0101_0000 | (((keylen as u64) << 8) & 0xFF00) | (self.hashlen as u64 & 0xFF);

        // if there was a key: prepend the key to the 1st 128 bytes of the message
        if let Some(key) = self.key {
            let mut k = key.to_vec();
            pad_trailing(&mut k, 0, 128);
            prepend(&mut self.message, k);
            bytes_remaining += 128;
        }

        // iterate over the message in 128 byte chunks
        // this use of take() feels like a code smell...?
        let message = std::mem::take(&mut self.message);
        let mut chunks = message.chunks(128).peekable();

        // if chunks is empty, it runs this and skips the for loop
        if chunks.peek().is_none() {
            let chunk = [0; 128];
            self.compress(&chunk, 0, true);
        }

        for chunk in chunks {
            bytes_compressed += chunk.len();
            bytes_remaining -= chunk.len();
            let is_last_chunk = bytes_remaining == 0;
            let mut c = chunk.to_vec();
            pad_trailing(&mut c, 0, 128);
            self.compress(&c, bytes_compressed, is_last_chunk);
        }

        let mut result = Vec::new();
        for word in self.state {
            result.extend_from_slice(&word.to_le_bytes());
        }
        result.truncate(self.hashlen);
        result
    }

    fn hash(message: &[u8]) -> Vec<u8> {
        let mut hasher = Self::new(64);
        hasher.update(message);
        hasher.digest()
    }
}

mod tests {
    // rust-analyzer does not detect the use of these? for some reason
    #![allow(unused_imports)]
    use crate::{
        hash::{Blake2b, HashFunction},
        slice::u8_to_hexstr,
    };

    #[test]
    fn basic_blake2b() {
        let res1 = u8_to_hexstr(&Blake2b::hash(b""));
        let exp1 = "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce";
        assert_eq!(res1, exp1);
    }
}
