use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn validate_hash(
    password: String,
    hash: PasswordHash,
) -> Result<(), argon2::password_hash::Error> {
    Argon2::default().verify_password(password.as_bytes(), &hash)
}
