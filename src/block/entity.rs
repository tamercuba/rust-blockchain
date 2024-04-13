use std::fmt::{ self, Debug, Formatter };
use std::time::{ SystemTime, UNIX_EPOCH };
use crate::hash::{ difficulty_bytes_as_u128, u128_bytes, u32_bytes, u64_bytes, Hash, Hashable };

#[derive(Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\n [Block index  : {}]\n [previous hash: {}]\n [hash         : {}]\n [timestamp    : {}]\n [transactions : {}]\n [nonce        : {}]\n",
            &self.index,
            &hex::encode(&self.prev_block_hash),
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce
        )
    }
}

pub fn now() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_millis()
}

impl Default for Block {
    fn default() -> Self {
        Self {
            index: 0,
            timestamp: now(),
            prev_block_hash: vec![0; 32],
            transactions: String::from("genesis!"),
            nonce: 0,
            hash: vec![0; 32],
            difficulty: 0x0000ffffffffffffffffffffffffffff,
        }
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: String,
        difficulty: u128
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }
    pub fn set_hash(&mut self) {
        for nonce_attempt in 0..u64::max_value() {
            self.nonce = nonce_attempt;
            let hash = self.get_hash();
            if self.difficulty > difficulty_bytes_as_u128(&hash) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}
