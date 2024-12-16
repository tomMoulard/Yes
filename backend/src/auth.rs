use crate::models::User;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{instrument, info, error};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    email: String,
    username: String,
}

const SECRET: &[u8] = b"secret";

#[instrument(skip(user))]
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
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap();

    info!("Generated JWT for user: {}", user.email);
    token
}

#[instrument(skip(token))]
pub fn validate_jwt(token: &str) -> bool {
    let is_valid = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .is_ok();

    if is_valid {
        info!("JWT is valid");
    } else {
        error!("JWT is invalid");
    }

    is_valid
}

#[instrument(skip(token))]
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
            };

            let new_token = encode(
                &Header::default(),
                &new_claims,
                &EncodingKey::from_secret(SECRET),
            )
            .unwrap();

            info!("Refreshed JWT for user: {}", token_data.claims.email);
            return Some(new_token);
        }
    }
    None
}
