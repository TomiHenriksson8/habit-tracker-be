
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Validation, Header};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::env;

// Load SECRET_KEY from .env
pub fn get_secret_key() -> String {
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
}

// 🛡️ Hash Password Using Argon2
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng); // Generate a random salt
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed")
        .to_string()
}

// 🔑 Verify Password
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password).expect("Invalid hash format");
    let argon2 = Argon2::default();

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// 📌 JWT Claims Structure
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// 🎫 Generate JWT Token
pub fn generate_jwt(user_email: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_email.to_owned(),
        exp: expiration,
    };

    let secret_key = get_secret_key();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Failed to generate JWT")
}

// 🔍 Decode JWT Token (Extract User Email)
pub fn decode_jwt(token: &str) -> Result<String, &'static str> {
    let secret_key = get_secret_key();
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());

    match decode::<Claims>(token, &decoding_key, &Validation::default()) {
        Ok(data) => Ok(data.claims.sub),
        Err(_) => Err("Invalid token"),
    }
}

