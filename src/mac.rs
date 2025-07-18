mod poly1305;

pub trait OneTimeAuthenticator {
    fn update(&mut self, message: &[u8]);
    fn finalize(self) -> Vec<u8>;
    fn verify(self, tag: &[u8]);
}
