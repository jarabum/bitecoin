use crate::block::Block;
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis = Block {
            hash: String::new(),
            prev_hash: String::from("0"),
            nonce: 0,
            transactions: Vec::new(),
            timestamp: 0,
            difficulty: 4,
        };

        genesis.timestamp();
        genesis.mine();

        Self {
            chain: vec![genesis]
        }        
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.prev_hash = self.chain.last().unwrap().hash.clone();
        block.timestamp();
        block.mine();

        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
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
}