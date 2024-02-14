use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::StatusCode;

use super::errors::HttpError;

pub async fn hash_password(password: &str) -> Result<String, HttpError> {
    let password = password.to_string();
    Ok(tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default(); // default settings, we can tweak later
        match argon2.hash_password(&password.as_bytes(), &salt){
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(HttpError::Simple(StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error".to_string()))
        }
    }).await??)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, HttpError> {
    let password = password.to_string();
    let hash = hash.to_string();
    Ok(tokio::task::spawn_blocking(move || {
        let argon2 = Argon2::default(); // default settings, we can tweak later
        let hash = PasswordHash::new(&hash)?;

        Ok::<bool, argon2::password_hash::Error>(argon2.verify_password(password.as_bytes(), &hash).is_ok())
        
    }).await??)
}