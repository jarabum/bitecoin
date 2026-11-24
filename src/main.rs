/*
mod block;
mod transaction;
*/
mod helpful_stuff;
mod wallet;

// just some testing stuff for now
fn main() {
    println!("bitecoin\n");

    println!("'bitecoin' hash: {}", helpful_stuff::hash("bitecoin"));
    println!("current unix time: {}", helpful_stuff::get_time());

    let wallet = wallet::Wallet::new();

    println!("\nwallet private key: {}", wallet.priv_key);
    println!("wallet public key: {}", wallet.pub_key);
    println!("wallet address: {}", wallet.address);

    println!(
        "\nis wallet address valid: {}",
        helpful_stuff::verify_address(wallet.address.as_str())
    );

    let mut signature = wallet.sign("bitecoin");

    println!("\n'bitecoin' signature with wallet: {}", signature);

    println!(
        "is signature valid: {}",
        helpful_stuff::verify_signature(wallet.pub_key.as_str(), signature.as_str(), "bitecoin")
    );
}
