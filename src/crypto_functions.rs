use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

pub fn hash(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    let hash = hasher.finalize();

    hex::encode(hash)
}

pub fn create_adress(pub_key_hex: &String) -> String {
    let pub_key = hex::decode(pub_key_hex)
        .expect("Invalid public key");

    let sha256 = Sha256::digest(&pub_key);
    let ripemd160 = Ripemd160::digest(sha256);

    let mut payload = vec![0x00];
    payload.extend_from_slice(&ripemd160);

    let checksum = Sha256::digest(Sha256::digest(&payload));
    payload.extend_from_slice(&checksum[..4]);

    bs58::encode(payload).into_string()
}

pub fn sign(priv_key_hex: &String, data: &String) -> String {
    let priv_key_bytes: [u8; 32] = hex::decode(priv_key_hex)
        .expect("Invalid private key hex")
        .try_into()
        .expect("Private key must be 32 bytes");
    let priv_key = SigningKey::from_bytes(&priv_key_bytes);

    let signature = priv_key.sign(data.as_bytes());

    hex::encode(signature.to_bytes())
}

pub fn generate_priv_key() -> String {
    let mut rng = rand::rng();
    let priv_key = SigningKey::generate(&mut rng);

    hex::encode(priv_key.to_bytes())
}

pub fn get_pub_key(priv_key_hex: &String) -> String {
    let priv_key_bytes: [u8; 32] = hex::decode(priv_key_hex)
        .expect("Invalid private key hex")
        .try_into()
        .expect("Private key must be 32 bytes");
    let priv_key = SigningKey::from_bytes(&priv_key_bytes);

    let pub_key = priv_key.verifying_key();

    hex::encode(pub_key.as_bytes())
}

pub fn verify_signature(pub_key_hex: &String, signature_hex: &String, data: &String) -> bool {
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
}
