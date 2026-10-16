use crate::crypto_functions;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub hash: String,
    pub receiver: String,
    pub amount: u128,
    pub signature: String,
    pub pub_key_hex: String,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u128) -> Self {
        Transaction {
            sender,
            hash: String::new(),
            receiver,
            amount,
            signature: String::new(),
            pub_key_hex: String::new(),
            timestamp: 0,
        }
    }

    pub fn sign(&mut self) {
        let hash = crypto_functions::hash(format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.timestamp
        )); //TODO: add pub_key_hex to hash

        let priv_key_hex = crypto_functions::generate_priv_key();
        let pub_key_hex = crypto_functions::get_pub_key(&priv_key_hex);

        self.pub_key_hex = pub_key_hex;

        self.signature = crypto_functions::sign(&priv_key_hex, &hash);
        self.hash = hash;
    }

    pub fn verify(&self) -> bool {
        crypto_functions::verify_signature(&self.pub_key_hex, &self.signature, &self.hash)
    }

    pub fn timestamp(&mut self) {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => self.timestamp = n.as_secs(),
            Err(_) => panic!("Invalid Systemtime"),
        }
    }
}
