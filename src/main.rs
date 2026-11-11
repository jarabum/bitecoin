mod block;
mod helpful_stuff;
mod transaction;
mod wallet;

// just some testing stuff for now
fn main() {
    println!("bitecoin");

    let wallet = wallet::Wallet::new();

    match wallet.send_money(
        "bite0001bc1949c7e140914b919339c1a34cacb12338ab95fdfe86b0",
        11,
    ) {
        Ok(tx) => println!("{:?}\n{:?}\n{}", tx.inputs, tx.outputs, tx.timestamp),
        Err(e) => println!("error when creating transaction: {e}"),
    }
}
