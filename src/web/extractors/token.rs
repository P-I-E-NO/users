use std::env;

use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::{header::AUTHORIZATION, request::Parts, StatusCode}, response::IntoResponse};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::web::{dto::user_claims::UserClaims, errors::HttpError};

pub struct Token(pub UserClaims);

impl Token {
    pub async fn generate(user: &UserClaims) -> Result<String, HttpError> {
        let key = env::var("JWT_SECRET").unwrap();

        Ok(
            encode(
                &Header::default(),
                &user, 
                &EncodingKey::from_secret(&key.as_bytes())
            )?
        )
    }
    pub async fn from(token: String) -> Result<Token, HttpError> {
        let key = env::var("JWT_SECRET").unwrap();
        let validation = Validation::new(Algorithm::HS256);

        tokio::task::spawn_blocking(move || {
            Ok(
                Token(
                    jsonwebtoken::decode::<UserClaims>(
                        &token, 
                        &DecodingKey::from_secret(&key.as_bytes()),
                        &validation
                    )?.claims
                )
            )
        }).await?
        
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(str_header) = auth_header
                .to_str() {
                    let pieces: Vec<&str> = str_header.split("Bearer ").collect();
                    if pieces.len() < 2 {
                        return Err(HttpError::Simple(StatusCode::BAD_REQUEST, "no_bearer_specified".to_string()))
                    }
                    Ok(
                        Token::from(pieces[1].to_string()).await?
                    )
                } else {
                    Err(HttpError::Simple(StatusCode::BAD_REQUEST, "invalid_auth_header".to_string()))
                }
        } else {
            Err(HttpError::Simple(StatusCode::BAD_REQUEST, "no_auth_header".to_string()))
        }
    }
}