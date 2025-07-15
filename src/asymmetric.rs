pub mod x25519;

pub struct PaddingScheme {
    // TODO!
}

pub trait AsymmetricEncryption {
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

pub trait MessageSigning {
    fn sign(&self, text: &[u8], padding_scheme: PaddingScheme) -> Vec<u8>;
    fn verify(&self, text: &[u8], padding_scheme: PaddingScheme) -> Result<(), ()>;
}

pub trait KeyExchange {
    fn derive_public_key(&self) -> Vec<u8>;
    fn get_shared_secret(&self, other_pub_key: &[u8]) -> Vec<u8>;
}
