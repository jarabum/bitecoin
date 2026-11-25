use crate::helpful_stuff;
use crate::helpful_stuff::ADDR_VERSION;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

// wallet struct
pub struct Wallet {
    pub pub_key: String,
    pub priv_key: String,
    pub address: String,
}

impl Wallet {
    // use this to create a new wallet
    pub fn new() -> Self {
        let priv_key = helpful_stuff::generate_priv_key();
        let pub_key = helpful_stuff::get_pub_key(priv_key.as_str());

        let mut wallet = Wallet {
            pub_key,
            priv_key,
            address: String::new(),
        };

        wallet.set_address();

        wallet
    }

    // this gets an address from a public key hex and sets address to it
    pub fn set_address(&mut self) {
        const PUB_KEY_HASH_LENGTH: usize = 32;
        const CHECKSUM_LENGTH: usize = 16;

        let full_hash = helpful_stuff::hash(self.pub_key.as_str());

        let first_half = String::from("bite") + ADDR_VERSION + &full_hash[0..PUB_KEY_HASH_LENGTH];

        let checksum_full = helpful_stuff::hash(first_half.as_str());
        let checksum_half = &checksum_full[0..CHECKSUM_LENGTH];

        self.address = first_half + checksum_half;
    }

    // this signs some data and returns it
    pub fn sign(&self, data: &str) -> String {
        helpful_stuff::sign(self.priv_key.as_str(), data)
    }
}
