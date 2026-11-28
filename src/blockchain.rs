use crate::block::Block;

// main blockchain struct
#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // use this to initiate the blockchain
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis_block()],
        }
    }

    // use this to add a block
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    // this returns the latest block
    pub fn tip(&self) -> Option<&Block> {
        self.chain.last()
    }

    // this returns the chain height
    pub fn height(&self) -> usize {
        self.chain.len() - 1
    }

    // this returns a block at a specific height
    pub fn get_block(&self, height: usize) -> Option<&Block> {
        self.chain.get(height)
    }

    // this returns true if the blockchain is valid
    // TODO: make this check the transactions and everything in the block
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

            if !current
                .hash
                .starts_with("0".repeat(current.difficulty).as_str())
            {
                return false;
            }

            if current.height != previous.height + 1 {
                return false;
            }
        }

        true
    }
}
