mod block;
mod blockchain;
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
            //println!("\nfull transaction payload: {}", tx.full_payload());
        }
        Err(e) => println!("error while creating transaction: {e}"),
    }

    let coinbase = transaction::Transaction::new_coinbase(
        "sajkfsbabafs",
        helpful_stuff::to_satoshi(10).expect("amount too large"),
        1,
    );

    println!("\n{:?}", coinbase);

    let block = block::Block::new(
        1,
        String::from("213b20e88a3b3353f62b527b638c6962ef77e254d5e60a0195f4805ff15105c6"),
        vec![coinbase],
        0,
    );

    println!("\n{:?}", block);
    //println!("block hashing payload: {}", block.get_hashing_payload());

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.add_block(block);

    println!("\nlatest block: {:?}", blockchain.tip());
    println!("blockchain height: {}", blockchain.height());
    println!("blockchain block 0: {:?}", blockchain.get_block(0));
    println!("is blockchain valid: {}", blockchain.is_valid());
}
