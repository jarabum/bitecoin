use crate::crypto_functions;

pub struct Wallet {
    pub pub_key: String,
    pub priv_key: String,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let priv_key = crypto_functions::generate_priv_key();
        let pub_key = crypto_functions::get_pub_key(&priv_key);
        let address = crypto_functions::create_adress(&pub_key);

        Self {
            priv_key,
            pub_key,
            address,
        }
    }
}