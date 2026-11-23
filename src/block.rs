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

pub const HALVING_INTERVAL: u32 = 300_000;
pub const INITIAL_SUBSIDY: u128 = 50 * 100_000_000; // 50 coins in base units

// this returns the block reward
pub fn block_subsidy(height: u32) -> u128 {
    let halvings = height / HALVING_INTERVAL;
    if halvings >= 64 {
        0
    } else {
        INITIAL_SUBSIDY >> halvings
    }
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
        let mut data = String::new();

        for tx in self.tx.iter() {
            data += &format!("tx:{};", tx.id().as_str());
        }

        self.tx_hash = helpful_stuff::hash(data.as_str());
        println!("{}", data);
    }

    // this returns the block hash
    pub fn get_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.height, self.prev_hash, self.nonce, self.tx_hash, self.timestamp, self.difficulty
        );

        helpful_stuff::hash(data.as_str())
    }

    // this function checks the coinbase transaction
    pub fn check_coinbase(&self, lookup: &dyn Fn(&str, u32) -> Option<u128>) -> Result<(), String> {
        let first_tx = self.tx.first().ok_or("block has no transactions")?;

        if !first_tx.is_coinbase() {
            return Err("first transaction must be a coinbase transaction".into());
        }
        if first_tx.coinbase_height != Some(self.height) {
            return Err("coinbase height mismatch".into());
        }

        if self.tx.iter().skip(1).any(|t| t.is_coinbase()) {
            return Err("more than one coinbase".into());
        }

        let mut fees: u128 = 0;
        for t in self.tx.iter().skip(1) {
            let mut in_sum: u128 = 0;
            for i in &t.inputs {
                let amt = lookup(&i.prev_tx_id, i.output_index)
                    .ok_or("input references unknown output")?;
                in_sum = in_sum.checked_add(amt).ok_or("overflow")?;
            }
            let out_sum: u128 = t.outputs.iter().map(|o| o.amount).sum();
            fees = fees
                .checked_add(in_sum.checked_sub(out_sum).ok_or("outputs exceed inputs")?)
                .ok_or("overflow")?;
        }

        let allowed = block_subsidy(self.height)
            .checked_add(fees)
            .ok_or("overflow")?;
        let claimed: u128 = first_tx.outputs.iter().map(|o| o.amount).sum();

        if claimed > allowed {
            return Err(format!("coinbase claims {claimed}, max allowed {allowed}"));
        }

        Ok(())
    }
}
