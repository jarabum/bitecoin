use crate::{helpful_stuff, transaction::Transaction};

// block struct
pub struct Block {
    pub height: u32,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u128,
    pub tx: Vec<Transaction>,
    pub tx_hash: String,
    pub timestamp: u64,
    pub difficulty: u16,
}

impl Block {
    // use this to create a new block
    pub fn new(height: u32, prev_hash: String, tx: Vec<Transaction>, difficulty: u16) -> Self {
        let mut block = Block {
            height,
            hash: String::new(),
            prev_hash,
            nonce: 0,
            tx,
            tx_hash: String::new(),
            timestamp: helpful_stuff::get_time(),
            difficulty,
        };

        block.hash_tx();
        block.hash = block.get_hash();

        block
    }

    // this hashes the transactions
    pub fn hash_tx(&mut self) {
        let data = format!("{:?}", self.tx);

        self.tx_hash = helpful_stuff::hash(data.as_str());
    }

    // this returns the block hash
    pub fn get_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.height, self.prev_hash, self.nonce, self.tx_hash, self.timestamp, self.difficulty
        );

        helpful_stuff::hash(data.as_str())
    }
}
