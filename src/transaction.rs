use crate::crypto_functions;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub signature: String,
    pub pub_key_hex: String,
    pub timestamp: u64,
}

impl Transaction {
    pub fn sign(&mut self) {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);

        let priv_key_hex = crypto_functions::generate_priv_key();
        let pub_key_hex = crypto_functions::get_pub_key(&priv_key_hex);

        self.pub_key_hex = pub_key_hex;

        self.signature = crypto_functions::sign(&priv_key_hex, &data);
    }

    pub fn verify(&self) -> bool {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);

        crypto_functions::verify_signature(&self.pub_key_hex, &self.signature, &data)
    }

    pub fn timestamp(&mut self) {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => self.timestamp = n.as_secs(),
            Err(_) => panic!("Invalid Systemtime"),
        }
    }
}
