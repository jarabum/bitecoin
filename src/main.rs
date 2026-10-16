mod block;
mod blockchain;
mod crypto_functions;
mod transaction;

fn main() {
    println!("bitecoin");

    let mut blockchain = blockchain::Blockchain::new();

    let mut transa =
        transaction::Transaction::new(String::from("test1"), String::from("test2"), 999);

    transa.timestamp();
    transa.sign();

    println!("\nTransaction:");
    println!("  Signature: {}", transa.signature);
    println!("  Public key: {}", transa.pub_key_hex);
    println!("  Verified: {}", transa.verify());

    blockchain.add_transaction_pending(transa);

    for i in 0..1 {
        // adds blocks for testing
        let mut block = block::Block::new(
            i + 1,
            blockchain.get_newest_hash(),
            Vec::new(),
            blockchain.difficulty,
        );

        block.timestamp();

        blockchain.add_block(block);
    }

    blockchain.mine_pending();

    println!("\nBlockchain");
    println!("  Length: {}", blockchain.chain.len());

    for block in blockchain.chain.iter() {
        println!("\n  Block {}", block.id);
        println!("    Hash: {}", block.hash);
        println!("    Previous: {}", block.prev_hash);
        println!("    Transactions: {:?}", block.transactions);
    }

    println!("Blockchain valid: {}", blockchain.is_valid());
}
