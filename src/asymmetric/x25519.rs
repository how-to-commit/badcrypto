use rand::CryptoRng;

use crate::math::elliptic_curve::field25519::{self, BASE_POINT};

use super::KeyExchange;

pub struct X25519 {
    private_key: [u8; 32],
}

impl X25519 {
    pub fn new<T>(rng: &mut T) -> Self
    where
        T: CryptoRng,
    {
        let mut new = [0u8; 32];
        rng.fill_bytes(&mut new);
        Self { private_key: new }
    }
}

impl KeyExchange for X25519 {
    fn generate_public_key(&self) -> Vec<u8> {
        field25519::scalarmult(&self.private_key, &BASE_POINT)
    }

    fn get_shared_secret(&self, other_pub_key: &[u8]) -> Vec<u8> {
        field25519::scalarmult(&self.private_key, &other_pub_key)
    }
}
