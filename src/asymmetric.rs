pub mod x25519;

pub trait AsymmetricEncryption {
    fn generate_keypair() -> Self;
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

pub trait KeyExchange {
    fn generate_public_key(&self) -> Vec<u8>;
    fn get_shared_secret(&self, other_pub_key: &[u8]) -> Vec<u8>;
}
