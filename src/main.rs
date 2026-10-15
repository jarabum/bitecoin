mod block;
mod crypto_functions;
mod transaction;

fn main() {
    println!("bitecoin");

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

    println!("{}, {}", transa.signature, transa.pub_key_hex);

    println!("{}", transa.verify());

    let mut block = block::Block {
        hash: String::new(),
        prev_hash: String::from("aslnhfasb"),
        nonce: 0,
        transactions: vec![transa],
        timestamp: 0,
        difficulty: 5,
    };

    block.timestamp();

    block.mine();
    println!("{}", block.hash);
}
