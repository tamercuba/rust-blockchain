pub trait Hashable {
    fn get_bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let bytes = self.get_bytes();
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}
