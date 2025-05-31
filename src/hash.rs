mod blake;

pub use blake::Blake2b;

pub trait HashFunction {
    fn hash(&self, message: &[u8]) -> Vec<u8>;
}
