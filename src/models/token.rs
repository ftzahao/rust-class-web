use crate::state::{ARGON2_SALT, TOKEN_EXPIRE_TIME};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    errors::Result as JwtResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(sub: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(TOKEN_EXPIRE_TIME))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ARGON2_SALT),
    )
    .unwrap()
}

pub fn verify_token(token: &str) -> JwtResult<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(ARGON2_SALT),
        &Validation::default(),
    )
}
