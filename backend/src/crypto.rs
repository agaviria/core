use argon2rs::verifier::Encoded;
use base64::encode;
use rand::{OsRng, Rng};

/// Returns a random 128bit salt or error
pub fn generate_salt() -> Result<Vec<u8>> {
    const SALT_SIZE: usize = 16;

    let mut salt = [0u8; SALT_SIZE];

    match OsRng::new() {
        Ok(mut rng) => {
            rng.fill_bytes(&mut salt);
        }
        Err(_) => bail!("rng error"),
    };
    Ok(salt.iter().cloned().collect::<Vec<u8>>())
}

/// Hash a password with Argon2
pub fn hash(pwd: &str) -> Result<Vec<u8>> {
    let salt = generate_salt()?;

    if salt.len() != 16 || (string.len() == 0 || string.len() > 72) {
        bail!("Salt or string generation error.");
    }

    let encoded_hash = Encoded::default2i(pwd.as_bytes(), &salt, &[], &[]).to_u8();
    Ok(encoded_hash.iter().cloned().collect::<Vec<u8>>())
}

/// Verifies password with Argon2
pub fn verify_password(encoded_hash: &[u8], plaintext_password: &str) -> Result<bool> {
    // TODO: error handling
    let encoded = Encoded::from_u8(encoded_hash);
    Ok(encoded.verify(plaintext_password.as_bytes()))
}
