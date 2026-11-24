/*
mod block;
mod transaction;
mod wallet;
*/
mod helpful_stuff;

// just some testing stuff for now
fn main() {
    println!("bitecoin");

    println!("'bitecoin' hash: {}", helpful_stuff::hash("bitecoin"));
    println!("current unix time: {}", helpful_stuff::get_time());

    let priv_key = helpful_stuff::generate_priv_key();

    println!("generated private key: {priv_key}");

    println!(
        "generated public key: {}",
        helpful_stuff::get_pub_key(priv_key.as_str())
    );
}
