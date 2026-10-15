mod block;
mod blockchain;
mod crypto_functions;
mod transaction;

fn main() {
    println!("bitecoin");

    let mut blockchain = blockchain::Blockchain::new();

    let mut transa = transaction::Transaction {
        sender: String::from("vassy"),
        receiver: String::from("maboj"),
        signature: String::new(),
        amount: 1000,
        pub_key_hex: String::new(),
        timestamp: 0,
    };

    transa.timestamp();
    transa.sign();

    println!("\nTransaction:");
    println!("  Signature: {}", transa.signature);
    println!("  Public key: {}", transa.pub_key_hex);
    println!("  Verified: {}", transa.verify());

    for _ in 0..10 { // prida 10 blocku na testovani
        let block = block::Block {
            hash: String::new(),
            prev_hash: String::new(),
            nonce: 0,
            transactions: Vec::new(),
            timestamp: 0,
            difficulty: 4,
        };

        blockchain.add_block(block);
    }

    println!("\nBlockchain");
    println!("  Length: {}", blockchain.chain.len());

    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("\n  Block {}", i);
        println!("    Hash: {}", block.hash);
        println!("    Previous: {}", block.prev_hash);
    }

    println!("Blockchain valid: {}", blockchain.is_valid());
}