mod block;
mod helpful_stuff;
mod transaction;
mod wallet;

// just some testing stuff for now
fn main() {
    println!("bitecoin");

    let wallet = wallet::Wallet::new();

    println!("{}", helpful_stuff::verify_address(wallet.address.as_str()));
    println!("{}", wallet.address);

    match wallet.send_money("bite01efb570a0be06479ba67842d3a2e223ac69536870951b99ea", 11) {
        Ok(tx) => {
            println!("{:?}\n{:?}\n{}", tx.inputs, tx.outputs, tx.timestamp);
            let block = block::Block::new(0, "0".repeat(32), vec![tx], 4);
            println!("\n{}\n{:?}", block.tx_hash, block.tx);
        }
        Err(e) => println!("error when creating transaction: {e}"),
    }
}
