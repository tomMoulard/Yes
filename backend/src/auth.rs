use crate::models::User;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    email: String,
    username: String,
    points: i64,
}

const SECRET: &[u8] = b"secret";

pub fn generate_jwt(user: &User) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 hour expiration

    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration as usize,
        email: user.email.clone(),
        username: user.username.clone(),
        points: user.points,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}

pub fn validate_jwt(token: &str) -> bool {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .is_ok()
}

pub fn refresh_jwt(token: &str) -> Option<String> {
    if validate_jwt(token) {
        if let Ok(token_data) = decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        ) {
            let new_expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600; // 1 hour expiration

            let new_claims = Claims {
                sub: token_data.claims.sub,
                exp: new_expiration as usize,
                email: token_data.claims.email,
                username: token_data.claims.username,
                points: token_data.claims.points,
            };

            return Some(
                encode(
                    &Header::default(),
                    &new_claims,
                    &EncodingKey::from_secret(SECRET),
                )
                .unwrap(),
            );
        }
    }
    None
}

pub fn token_to_user(token: &str) -> Option<User> {
    if let Ok(token_data) = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    ) {
        Some(User {
            email: token_data.claims.email,
            username: token_data.claims.username,
            password: "".to_string(),
            points: token_data.claims.points,
        })
    } else {
        None
    }
}

pub fn extract_email_from_jwt(token: &str) -> Option<String> {
    token_to_user(token).map(|u| u.email)
}
