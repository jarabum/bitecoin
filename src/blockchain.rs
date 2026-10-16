use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis = Block {
            id: 0,
            hash: String::new(),
            prev_hash: "0".repeat(64),
            nonce: 0,
            transactions: Vec::new(),
            timestamp: 0,
            difficulty: 4,
        };

        println!("genesis");

        genesis.timestamp();
        genesis.mine();

        println!("mined");

        Blockchain {
            chain: vec![genesis],
            pending_transactions: Vec::new(),
            difficulty: 4,
        }
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.prev_hash = self.chain.last().unwrap().hash.clone();
        block.timestamp();
        block.mine();

        self.chain.push(block);
    }

    pub fn is_valid(&mut self) -> bool {
        for i in 1..self.chain.len() {
            let previous = &self.chain[i - 1];
            let current = &self.chain[i];

            if current.prev_hash != previous.hash {
                return false;
            }

            if current.hash != current.get_hash() {
                return false;
            }

            if !current.hash.starts_with(&"0".repeat(current.difficulty)) {
                return false;
            }
        }
        true
    }

    pub fn get_newest_hash(&self) -> String {
        self.chain.last().unwrap().hash.clone()
    }

    pub fn mine_pending(&mut self) {
        let mut block = Block {
            id: self.chain.len(),
            hash: String::new(),
            nonce: 0,
            prev_hash: self.get_newest_hash(),
            transactions: std::mem::take(&mut self.pending_transactions),
            timestamp: 0,
            difficulty: self.difficulty,
        };

        block.timestamp();
        block.mine();

        self.chain.push(block);
    }

    pub fn add_transaction_pending(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
}
