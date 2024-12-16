use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    email: String,
    username: String,
}

const SECRET: &[u8] = b"secret";

pub fn generate_jwt(user: &User) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hour expiration

    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration as usize,
        email: user.email.clone(),
        username: user.username.clone(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn validate_jwt(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default()).is_ok()
}

pub fn refresh_jwt(token: &str) -> Option<String> {
    if let Ok(token_data) = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default()) {
        let new_expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour expiration

        let new_claims = Claims {
            sub: token_data.claims.sub,
            exp: new_expiration as usize,
            email: token_data.claims.email,
            username: token_data.claims.username,
        };

        Some(encode(&Header::default(), &new_claims, &EncodingKey::from_secret(SECRET)).unwrap())
    } else {
        None
    }
}
