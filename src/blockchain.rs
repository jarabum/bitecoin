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

    pub fn get_block(&self, height: usize) -> Option<&Block> {
        self.chain.get(height)
    }
}
