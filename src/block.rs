use crate::{helpful_stuff, transaction::Transaction};

// block struct
#[derive(Debug)]
pub struct Block {
    pub height: usize,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u128,
    pub tx: Vec<Transaction>,
    pub tx_hash: String,
    pub timestamp: u64,
    pub difficulty: u16,
}

pub const HALVING_INTERVAL: usize = 300_000;
pub const INITIAL_REWARD: u128 = 50 * 100_000_000; // 50 coins in base units

impl Block {
    // use this to create a new block
    pub fn new(height: usize, prev_hash: String, tx: Vec<Transaction>, difficulty: u16) -> Self {
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

    pub fn genesis_block() -> Self {
        let mut block = Block {
            height: 0,
            hash: String::new(),
            prev_hash: "0".repeat(64),
            nonce: 0,
            tx: vec![Transaction::genesis_tx()],
            tx_hash: String::new(),
            timestamp: 123456, // TODO: change this timestamp to the same one that will be in transactions
            difficulty: 6,
        };

        block.hash_tx();
        block.hash = block.get_hash();

        block
    }

    // this hashes the transactions
    pub fn hash_tx(&mut self) {
        let mut data = String::new();

        for tx in self.tx.iter() {
            data += &format!("tx:{};", tx.get_id().as_str());
        }

        self.tx_hash = helpful_stuff::hash(data.as_str());
        println!("{}", data);
    }

    // this returns the block hash
    pub fn get_hash(&self) -> String {
        helpful_stuff::hash(self.get_hashing_payload().as_str())
    }

    // this returns the hashing payload for the block
    pub fn get_hashing_payload(&self) -> String {
        let mut payload = String::new();

        payload += &format!(
            "h:{};ph:{};n:{};txh:{};t:{};d:{};",
            self.height, self.prev_hash, self.nonce, self.tx_hash, self.timestamp, self.difficulty,
        );

        payload
    }

    // this returns the block reward
    pub fn get_block_reward(&self) -> u128 {
        let halvings = self.height / HALVING_INTERVAL;
        if halvings >= 64 {
            0
        } else {
            INITIAL_REWARD >> halvings
        }
    }
}
