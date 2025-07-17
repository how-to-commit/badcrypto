use crate::{
    arith,
    mac::{OneTimeAuthenticator, poly1305},
};

// magical constants are from rfc8493.
// deriving constants from poly1305aes_test_clamp (annotated):
// (hex representation of 15 is 0x0f and 252 is 0xfc)
//    r[3] &= 0x0f ---|
//    r[7] &= 0x0f    | -> creates mask 0xffff_ff0f_ffff_ff0f_ffff_ff0f_ffff_ff0f
//    r[11] &= 0x0f   |
//    r[15] &= 0x0f --|
//    r[4] &= 0xfc    | -> creates mask 0xffff_ffff_fcff_ffff_fcff_ffff_fcff_ffff
//    r[8] &= 0xfc    |
//    r[12] &= 0xfc --|
// both combine to:                     0xffff_ff0f_fcff_ff0f_fcff_ff0f_fcff_ff0f (LE)
// in big endian:                       0x0fff_fffc_0fff_fffc_0fff_fffc_0fff_ffff (BE)
const POLY1305_R_CLAMP_LO: u64 = 0x0fff_fffc_0fff_ffff;
const POLY1305_R_CLAMP_HI: u64 = 0x0fff_fffc_0fff_fffc;
const POLY1305_MSG_CHUNK_SIZE_BYTES: usize = 16;

pub struct Poly1305 {
    secret_r: (u64, u64),   // lo, hi
    secret_s: (u64, u64),   // lo, hi
    accum: (u64, u64, u64), // split into lo, mid, hi -> can exceed
}

impl Poly1305 {
    fn from_le_bytes(r: [u8; 16], s: [u8; 16]) -> Self {
        let mut clamped_r = r;

        let r0 = u64::from_le_bytes(r[0..8].try_into().expect("len 8")) & POLY1305_R_CLAMP_LO;
        let r1 = u64::from_le_bytes(r[8..16].try_into().expect("len 8")) & POLY1305_R_CLAMP_HI;
        let s0 = u64::from_le_bytes(s[0..8].try_into().expect("len 8"));
        let s1 = u64::from_le_bytes(s[8..16].try_into().expect("len 8"));

        Self {
            secret_r: (r0, r1),
            secret_s: (s0, s1),
            accum: (0, 0, 0),
        }
    }
}

impl OneTimeAuthenticator for Poly1305 {
    fn update(&mut self, message: &[u8]) {
        let (mut h0, mut h1, mut h2) = self.accum;

        for window in message.chunks(POLY1305_MSG_CHUNK_SIZE_BYTES) {
            let mut owned_w: Vec<u8> = window.to_vec();

            // step 0.5: resize to chunksize and add a bit above the message
            if owned_w.len() < POLY1305_MSG_CHUNK_SIZE_BYTES {
                owned_w.push(1);
                owned_w.resize(POLY1305_MSG_CHUNK_SIZE_BYTES, 0);
            }

            let mut c = 0u64;
            // step 1: h (the accumulator) + m
            (h0, c) = arith::carrying_add(
                h0,
                u64::from_le_bytes(owned_w[0..8].try_into().expect("Good!")),
                0,
            );
            (h1, c) = arith::carrying_add(
                h1,
                u64::from_le_bytes(owned_w[8..16].try_into().expect("Good!")),
                c,
            );

            // step 0.5: set a bit above the message size
            if window.len() == POLY1305_MSG_CHUNK_SIZE_BYTES {
                h2 = c + 1; // if len is chunk size, adding 1 to the 2^128 limb is fine
            }

            // step 2: h * r (long mul)
            //
            //    h0    h1    h2
            //    r1    r2         x
            // =========================
            //   h0r0  h1r0  h2r0               <--- 128 bit products
            //         h0r1  h1r1  h2r1   +     <--|
            // ==============================
            //    t0    t1    t2    t3          <--- 128 bit intermediates with
            //                                       overlapping limbs (!)
            // CARRYING THE INTERMEDIATE LIMBS
            // ===============================
            //         t0.1  t1.1  t2.1         <--- t3 does not have a higher half (h2r1)
            //   t0.0  t1.0  t2.0  t3.0        +
            // =====================================
            //   res0  res1  res2  res3
            //
            // having r clamped means that no overflow can occur while adding
            // to the intermediates t1 and t2.
            //
            // because h2 is capped to 5, and r has its first 4 bits clamped,
            // h2r0 and h2r1 do not have a higher half, meaning there is no
            // result limb res4.
            //
            // takeaways and optimisations:
            //  - overflow cannot occur while calculating t* intermediates,
            //  - there isnt a result limb res4 (h2r1/t3 higher half), and
            //  - t2's higher half is equal to h1r1's higher half.

            let h0r0 = arith::widening_mul(h0, self.secret_r.0);
            let h1r0 = arith::widening_mul(h1, self.secret_r.0);
            let h2r0 = h2 * self.secret_r.0; // h2 is <= 5, r is clamped
            let h0r1 = arith::widening_mul(h0, self.secret_r.1);
            let h1r1 = arith::widening_mul(h1, self.secret_r.1);
            let h2r1 = h2 * self.secret_r.1; // h2 is <= 5, r is clamped

            // intermediates
            let (t1lo, mut c) = arith::carrying_add(h1r0.0, h0r1.0, 0);
            let t1hi = arith::carrying_add(h1r0.1, h0r1.1, c).0;
            let (t2lo, mut c) = arith::carrying_add(h2r0, h1r1.0, 0);
            let t2hi = arith::carrying_add(h2r0, h1r1.0, c).0;

            // results
            let res0 = h0r0.0;
            let (res1, mut c) = arith::carrying_add(h0r0.1, t1lo, 0);
            let (res2, mut c) = arith::carrying_add(t1hi, t2lo, c);
            let (res3, mut c) = arith::carrying_add(t2hi, h2r1, c);

            // step 3: reducing the result mod 2^130 - 5
            // this is the same (?) as working with Curve25519 using Solinas primes
            // c * 2^130 + n = c * 5 + n (mod 2^130 - 5)

            // split the result into below and above 2^130 (carry)
            // the carry is c, below is n.
            let lower_2_mask = 0b11;
            let n2 = res2 & lower_2_mask;
            let mut carry = (res2 & !lower_2_mask, res3); // this is actually 4c

            // add 4c to h
            (h0, c) = arith::carrying_add(res0, carry.0, 0);
            (h1, c) = arith::carrying_add(res1, carry.1, c);
            h2 = n2 + c;

            // calculate c from 4c
            carry.0 = carry.0 >> 2 | carry.1 << 62;
            carry.1 = carry.1 >> 2;

            // add the last c to h
            (h0, c) = arith::carrying_add(res0, carry.0, 0);
            (h1, c) = arith::carrying_add(res1, carry.1, c);
            h2 += c;
        }

        self.accum = (h0, h1, h2);
    }

    fn finalize(self) {
        todo!()
    }

    fn verify(self, tag: &[u8]) {
        todo!()
    }
}
