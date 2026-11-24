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
}
