// src/auth.rs
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng; // Random number generator

pub fn hash_password(password: &str) -> String {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Configure Argon2 with the default settings (Argon2id variant)
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    password_hash
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    // Parse the stored password hash
    let parsed_hash = PasswordHash::new(hash).expect("Failed to parse password hash");

    // Verify the password against the stored hash
    let argon2 = Argon2::default();
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
