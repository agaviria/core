use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};
use base64::encode;
use rand::{OsRng, Rng};

/// Returns a random 128bit salt or error
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];

    match OsRng::new() {
        Ok(mut rng) => {
            rng.fill_bytes(&mut salt);
        }
        Err(_) => println!("rng error"),
    };
    return salt;
}

/// Hash a password with Argon2
pub fn hash(pwd: &str, salt: &[u8]) -> String {
    if salt.len() != 16 || (pwd.len() == 0 || pwd.len() > 72) {
        println!("Salt or string generation error.");
    }
    let argon2i = Argon2::new(10, 1, 4096, Variant::Argon2i).unwrap();
    let encoded_hash = Encoded::new(pwd.as_bytes(), &salt, &[], &[]).to_u8();
    String::from_utf8(encoded_hash).unwrap()
}

/// Verifies password with Argon2
pub fn verify_password(encoded_hash: &[u8], plaintext_password: &str) -> bool {
    // TODO: error handling
    let encoded = Encoded::from_u8(encoded_hash);
    Ok(encoded.verify(plaintext_password.as_bytes()))
}
