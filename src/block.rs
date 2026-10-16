use crate::{crypto_functions, transaction::Transaction};
use std::time::SystemTime;

pub struct Block {
    pub id: usize,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u128,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub difficulty: usize,
}

impl Block {
    pub fn new(
        id: usize,
        prev_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Self {
        Block {
            id,
            hash: String::new(),
            prev_hash,
            nonce: 0,
            transactions,
            timestamp: 0,
            difficulty,
        }
    }

    pub fn get_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}{}",
            self.prev_hash, self.nonce, self.transactions, self.timestamp, self.difficulty, self.id
        );

        crypto_functions::hash(data)
    }

    pub fn timestamp(&mut self) {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => self.timestamp = n.as_secs(),
            Err(_) => panic!("Invalid Systemtime"),
        }
    }

    pub fn mine(&mut self) {
        let hash = loop {
            let hash = self.get_hash();

            if hash.starts_with("0".repeat(self.difficulty).as_str()) {
                break hash;
            }

            self.nonce += 1;
        };

        println!("  Mined successfully ({} attempts)", self.nonce);
        self.hash = hash;
    }
}
