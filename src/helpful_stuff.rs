use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

// this hashes the input and returns the hex of the hash
pub fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    let hash = hasher.finalize();

    hex::encode(hash)
}

// this returns the unix time
pub fn get_time() -> u64 {
    let mut time = 0;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => time = n.as_secs(),
        Err(_) => panic!("Invalid Systemtime"),
    }

    time
}

// this generates and returns a new private key
pub fn generate_priv_key() -> String {
    let mut rng = rand::rng();
    let priv_key = SigningKey::generate(&mut rng);

    hex::encode(priv_key.to_bytes())
}

// this returns true if a wallet address is correct
pub fn verify_address(address: &str) -> bool {
    // "bite" (4) + version (4) + hash (32) + checksum (16) = 56
    if address.len() != 56 || !address.is_ascii() || !address.starts_with("bite") {
        return false;
    }

    let (body, checksum) = address.split_at(56 - 16);

    &hash(body)[0..16] == checksum
}

// this signs some data with a private key hex
pub fn sign(priv_key_hex: &str, data: &str) -> String {
    let priv_key_bytes: [u8; 32] = hex::decode(priv_key_hex)
        .expect("Invalid private key hex")
        .try_into()
        .expect("Private key must be 32 bytes");
    let priv_key = SigningKey::from_bytes(&priv_key_bytes);

    let signature = priv_key.sign(data.as_bytes());

    hex::encode(signature.to_bytes())
}

// this verifies a signature using a public key
pub fn verify_signature(pub_key_hex: &str, signature_hex: &str, data: &str) -> bool {
    /*
    let signature_bytes: [u8; 64] = hex::decode(signature_hex)
        .expect("Invalid hex signature")
        .try_into()
        .expect("Signature must be 64 bytes");
    let signature = &Signature::from_bytes(&signature_bytes);

    let pub_key_bytes: [u8; 32] = hex::decode(pub_key_hex)
        .expect("Invalid hex public key")
        .try_into()
        .expect("Public key must be 32 bytes");
    let pub_key = VerifyingKey::from_bytes(&pub_key_bytes).expect("Invalid public key");

    match pub_key.verify(data.as_bytes(), signature) {
        Ok(()) => true,
        Err(_) => false,
    }
    */

    let Ok(sig_bytes) = hex::decode(signature_hex) else {
        return false;
    };
    let Ok(sig_bytes): Result<[u8; 64], _> = sig_bytes.try_into() else {
        return false;
    };
    let Ok(pk_bytes) = hex::decode(pub_key_hex) else {
        return false;
    };
    let Ok(pk_bytes): Result<[u8; 32], _> = pk_bytes.try_into() else {
        return false;
    };
    let Ok(pub_key) = VerifyingKey::from_bytes(&pk_bytes) else {
        return false;
    };

    pub_key
        .verify_strict(data.as_bytes(), &Signature::from_bytes(&sig_bytes))
        .is_ok()
}

// this returns the current code version which is stored in ./version.txt
pub fn get_version() -> String {
    String::from(include_str!("./version.txt").trim())
}
