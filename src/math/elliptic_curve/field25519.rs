/// based on tweetnacl
use std::ops::{Add, Mul, Sub};

const A24: FieldElement = FieldElement {
    inner: [0xDB41, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const BASE_POINT: [u8; 32] = {
    let mut temp = [0u8; 32];
    temp[0] = 9;
    temp
};

#[derive(Clone)]
struct FieldElement {
    inner: [i64; 16],
}

impl FieldElement {
    pub fn new() -> Self {
        Self { inner: [0; 16] }
    }

    pub fn one() -> Self {
        let mut n = Self::new();
        n.inner[0] = 1;
        n
    }

    pub fn carry(&mut self) {
        let mut carry = 0i64;
        for i in 0..16 {
            carry = self.inner[i] >> 16;
            self.inner[i] -= carry << 16;

            if i < 15 {
                self.inner[i + 1] += carry;
            } else {
                self.inner[0] += 38 * carry;
            }
        }
    }

    pub fn complete_carry(&mut self) {
        self.carry();
        self.carry();
        self.carry();
    }

    pub fn inverse(&self) -> Self {
        let mut res = self.clone();
        for i in (0..=253).rev() {
            res = &res * &res;
            if i != 2 && i != 4 {
                res = &res * self;
            }
        }
        res
    }

    pub fn swap(p: &mut FieldElement, q: &mut FieldElement, swap: i64) {
        let mask = !(swap - 1);
        for i in 0..16 {
            let diff = mask & (p.inner[i] ^ q.inner[i]);
            p.inner[i] ^= diff;
            q.inner[i] ^= diff;
        }
    }

    pub fn from_bytes(x: &[u8]) -> Self {
        let mut out = Self::new();
        for i in 0..16 {
            // promotion to i64 from u8 is infallible.
            out.inner[i] = x[2 * i] as i64 + ((x[2 * i + 1] as i64) << 8);
        }
        out
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut t = self.clone();
        let mut m = Self::new();
        t.complete_carry();

        for _ in 0..2 {
            m.inner[0] = t.inner[0] - 0xffed;

            for j in 1..15 {
                m.inner[j] = t.inner[j] - 0xffff - ((m.inner[j - 1] >> 16) & 1);
                m.inner[j - 1] &= 0xffff;
            }

            m.inner[15] = t.inner[15] - 0x7fff - ((m.inner[14] >> 16) & 1);
            let carry = (m.inner[15] >> 16) & 1;
            m.inner[14] &= 0xffff;

            Self::swap(&mut t, &mut m, 1 - carry);
        }

        let mut res = vec![0u8; 32];
        for i in 0..16 {
            res[2 * i] = (t.inner[i] & 0xff) as u8;
            res[2 * i + 1] = (t.inner[i] >> 8) as u8;
        }
        res
    }
}

impl<'a> Add<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = FieldElement::new();
        for i in 0..16 {
            out.inner[i] = self.inner[i] + rhs.inner[i];
        }
        out
    }
}

impl<'a> Sub<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = FieldElement::new();
        for i in 0..16 {
            out.inner[i] = self.inner[i] - rhs.inner[i];
        }
        out
    }
}

impl<'a> Mul<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = [0i64; 31];
        let mut out = FieldElement::new();

        for i in 0..16 {
            for j in 0..16 {
                product[i + j] += self.inner[i] * rhs.inner[j];
            }
        }

        for i in 0..15 {
            product[i] += 38 * product[i + 16];
        }

        out.inner.copy_from_slice(&product[..16]);

        out.carry();
        out.carry();
        out
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Curve25519 FieldElement: ")?;
        for limb in self.inner.iter().rev() {
            write!(f, "{limb:04x} ")?;
        }
        write!(f, "")
    }
}

pub fn scalarmult(scalar: &[u8], point: &[u8]) -> Vec<u8> {
    let mut clamped = [0u8; 32];
    clamped.clone_from_slice(scalar);
    clamped[0] &= 0xf8;
    clamped[31] &= 0x7f;
    clamped[31] |= 0x40;

    let mut x = FieldElement::from_bytes(point);
    let mut x2 = FieldElement::one();
    let mut x3 = x.clone();
    let mut z2 = FieldElement::new();
    let mut z3 = FieldElement::one();
    let mut e = FieldElement::new();
    let mut f = FieldElement::new();

    for i in (0..255).rev() {
        let bit = (clamped[i >> 3] >> (i & 7)) & 1;
        FieldElement::swap(&mut x2, &mut x3, bit.into());
        FieldElement::swap(&mut z2, &mut z3, bit.into());

        e = &x2 + &z2;
        x2 = &x2 - &z2;
        z2 = &x3 + &z3;
        x3 = &x3 - &z3;
        z3 = &e * &e;
        f = &x2 * &x2;
        x2 = &x2 * &z2;
        z2 = &x3 * &e;
        e = &x2 + &z2;
        x2 = &x2 - &z2;
        x3 = &x2 * &x2;
        z2 = &z3 - &f;
        x2 = &z2 * &A24;
        x2 = &x2 + &z3;
        z2 = &z2 * &x2;
        x2 = &z3 * &f;
        z3 = &x3 * &x;
        x3 = &e * &e;

        FieldElement::swap(&mut x2, &mut x3, bit.into());
        FieldElement::swap(&mut z2, &mut z3, bit.into());
    }

    z2 = z2.inverse();
    x2 = &x2 * &z2;

    x2.to_bytes()
}

#[cfg(test)]
mod tests {
    use crate::slice::u8_to_hexstr;

    use super::{BASE_POINT, scalarmult};

    #[test]
    fn b() {
        let scalar = {
            let mut temp = [0u8; 32];
            temp[0] = 7;
            temp
        };
        println!("{}", u8_to_hexstr(&scalarmult(&scalar, &BASE_POINT)));
    }
}
