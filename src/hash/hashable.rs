use super::Hash;

pub trait Hashable {
    fn get_bytes(&self) -> Hash;

    fn get_hash(&self) -> Hash {
        let bytes = self.get_bytes();
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}
