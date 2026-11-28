mod block;
mod helpful_stuff;
mod transaction;
mod wallet;

// just some testing stuff for now
fn main() {
    println!("bitecoin\n");

    let wallet = wallet::Wallet::new();

    println!("wallet private key: {}", wallet.priv_key);
    println!("wallet public key: {}", wallet.pub_key);
    println!("wallet address: {}\n", wallet.address);

    let spendable: Vec<(transaction::OutPoint, transaction::OutputTx)> = Vec::from([
        (
            transaction::OutPoint {
                tx_id: String::from("ab12..."),
                index: 0,
            },
            transaction::OutputTx {
                amount: helpful_stuff::to_satoshi(10).expect("amount too large"),
                recipient: String::from("bite...alice"),
            },
        ),
        (
            transaction::OutPoint {
                tx_id: String::from("cd34..."),
                index: 1,
            },
            transaction::OutputTx {
                amount: helpful_stuff::to_satoshi(4).expect("amount too large"),
                recipient: String::from("bite...alice"),
            },
        ),
    ]);

    match transaction::Transaction::new(
        &wallet,
        "bite0122b086f0b8031378fed567f91ad5ac078e1d190734a6bc63",
        helpful_stuff::to_satoshi(10).expect("amount too large"),
        helpful_stuff::to_satoshi(1).expect("amount too large"),
        spendable,
    ) {
        Ok(tx) => {
            println!("{:?}", tx);
            println!("\nfull transaction payload: {}", tx.full_payload());
        }
        Err(e) => println!("error while creating transaction: {e}"),
    }

    let coinbase = transaction::Transaction::new_coinbase(
        "sajkfsbabafs",
        helpful_stuff::to_satoshi(10).expect("amount too large"),
        0,
    );

    println!("\n{:?}", coinbase);

    let block = block::Block::new(0, String::from("prev_hash"), vec![coinbase], 2);

    println!("\n{:?}", block);
    println!("block hashing payload: {}", block.get_hashing_payload());
    println!("block reward: {}", block.get_block_reward());
}
