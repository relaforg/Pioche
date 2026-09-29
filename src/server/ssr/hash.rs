use std::sync::LazyLock;

use argon2::{password_hash, Argon2, PasswordHasher, PasswordVerifier};

pub static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| hash("dummy".into()).unwrap());

pub fn hash(password: &str) -> Result<String, password_hash::Error> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    Ok(hash)
}

pub fn verify(password: &str, hash: &str) -> Result<bool, password_hash::Error> {
    match Argon2::default().verify_password(password.as_bytes(), hash) {
        Ok(_) => Ok(true),
        Err(password_hash::Error::PasswordInvalid) => Ok(false),
        Err(err) => Err(err),
    }
}
