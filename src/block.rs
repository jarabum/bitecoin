use crate::{crypto_functions, transaction};
use std::time::SystemTime;

pub struct Block {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u128,
    pub transactions: Vec<transaction::Transaction>,
    pub timestamp: u64,
    pub difficulty: usize,
}

impl Block {
    pub fn get_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.prev_hash, self.nonce, self.transactions, self.timestamp, self.difficulty
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
        let mut hash = String::new();

        println!("{hash}");

        loop {
            hash = self.get_hash();

            if hash.starts_with("0".repeat(self.difficulty).as_str()) {
                break;
            }
            self.nonce += 1;
        }
        println!("it took {} tries", self.nonce);

        self.hash = hash;
    }
}
