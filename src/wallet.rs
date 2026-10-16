use std::io::{self, Write};
use crate::crypto_functions;
use inquire::{Password, PasswordDisplayMode};

pub fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn ask_wallet_details() -> (String, String, String) {
    let location = prompt("Enter a path to the folder you want your wallet to be stored in: ");
    let name = prompt("Choose your wallet name: ");
    let password = Password::new("Choose your password used for encryption of your wallet: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_custom_confirmation_message("Confirm your password: ")
        .with_custom_confirmation_error_message("Passwords do not match, try again.")
        .prompt()
        .unwrap();

    (location, name, password)
}

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