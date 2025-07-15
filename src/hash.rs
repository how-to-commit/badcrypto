mod blake2b;

pub use blake2b::Blake2b;

pub trait HashFunction {
    /// update message
    fn update(&mut self, message: &[u8]);
    /// create digest, consumes self to prevent reuse
    fn digest(self) -> Vec<u8>;
    /// shortcut for init -> update -> digest with default settings
    fn hash(message: &[u8]) -> Vec<u8>;
}
