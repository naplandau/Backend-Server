use crate::config::config::CONFIG;
use crate::core::models::Claims;
// use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn generate_jwt(claims: Claims) -> String {
    let _var = &CONFIG.secret_key;
    let key = _var.as_bytes();
    encode(&Header::default(), &claims, &EncodingKey::from_secret(key)).unwrap()
}

pub fn validate_jwt(jwt_token: &str) -> Option<Claims> {
    let _var = &CONFIG.secret_key;
    let key = _var.as_bytes();
    let _decode = decode::<Claims>(
        jwt_token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    );
    match _decode {
        Ok(decoded) => Some(decoded.claims),
        Err(_) => None,
    }
}
