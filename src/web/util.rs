use std::env;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use axum::http::StatusCode;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use super::{dto::user_claims::UserClaims, errors::HttpError, extractors::token::Token};

pub async fn hash_password(password: String) -> Result<String, HttpError> {
    Ok(tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default(); // default settings, we can tweak later
        match argon2.hash_password(&password.as_bytes(), &salt){
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(HttpError::Simple(StatusCode::INTERNAL_SERVER_ERROR, "cannot_hash".to_string()))
        }
    }).await??)
}