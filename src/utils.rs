use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

pub fn get_secret_key() -> String {
    dotenv().ok();
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
}
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret_key().as_ref()),
    )
    .unwrap()
}
