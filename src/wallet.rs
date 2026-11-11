use crate::helpful_stuff;
use crate::transaction;
use crate::transaction::Transaction;
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

        let mut wallet = Wallet {
            pub_key: String::new(),
            priv_key,
            address: String::new(),
        };

        let priv_key_hex = wallet.priv_key.clone();
        wallet.set_pub_key(priv_key_hex.as_str());

        let pub_key_hex = wallet.pub_key.clone();
        wallet.set_address(pub_key_hex.as_str());

        wallet
    }

    // this gets a public key from a private key hex and sets pub_key to it
    pub fn set_pub_key(&mut self, priv_key_hex: &str) {
        let priv_key_bytes: [u8; 32] = hex::decode(priv_key_hex)
            .expect("Invalid private key hex")
            .try_into()
            .expect("Private key must be 32 bytes");
        let priv_key = SigningKey::from_bytes(&priv_key_bytes);

        let pub_key = priv_key.verifying_key();

        self.pub_key = hex::encode(pub_key.as_bytes());
    }

    // this gets an address from a public key hex and sets address to it
    pub fn set_address(&mut self, pub_key_hex: &str) {
        const PUB_KEY_HASH_LENGTH: usize = 32;
        const CHECKSUM_LENGTH: usize = 16;

        let full_hash = helpful_stuff::hash(pub_key_hex);

        let first_half = String::from("bite")
            + helpful_stuff::get_version().as_str()
            + &full_hash[0..PUB_KEY_HASH_LENGTH];

        let checksum_full = helpful_stuff::hash(first_half.as_str());
        let checksum_half = &checksum_full[0..CHECKSUM_LENGTH];

        self.address = first_half + checksum_half;
    }

    // this returns a signature of some data with wallets own private key
    pub fn get_signature(&self, data: &str) -> String {
        helpful_stuff::sign(self.priv_key.as_str(), data)
    }

    pub fn send_money(&self, receiver_addr: &str, amount: u128) -> Result<Transaction, String> {
        //TODO: add real uxto

        let spendable: Vec<(transaction::OutPoint, transaction::TxOutput)> = Vec::from([
            (
                transaction::OutPoint {
                    tx_id: String::from("ab12..."),
                    index: 0,
                },
                transaction::TxOutput {
                    amount: 10,
                    recipient: String::from("bite...alice"),
                },
            ),
            (
                transaction::OutPoint {
                    tx_id: String::from("cd34..."),
                    index: 1,
                },
                transaction::TxOutput {
                    amount: 4,
                    recipient: String::from("bite...alice"),
                },
            ),
        ]);

        let tx = Transaction::new(self, receiver_addr, amount, spendable)?;

        Ok(tx)
    }
}
