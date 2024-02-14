use axum::{debug_handler, extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::web::{dto::{auth::{login_request::LoginRequest, register_request::RegisterRequest}, user_claims::UserClaims, Claim}, errors::HttpError, extractors::{token::Token, validate_body::ValidatedJson}, models::users::User, util::{hash_password, verify_password}, AppState};

#[debug_handler]
pub async fn index(
    Token(user): Token<Claim<UserClaims>>
) -> Result<Json<Value>, HttpError> {

    Ok(Json(
        json!({
            "success": true,
            "user": user.data()
        })
    ))

}

pub async fn login(
    State(s): State<AppState>,
    ValidatedJson(body): ValidatedJson<LoginRequest>
) -> Result<Json<Value>, HttpError> {

    let mut conn = s.pool.acquire().await?;

    if let Some(user) = User::from_email(&mut *conn, &body.email).await? {

        let verify = verify_password(&body.password, &user.password).await?;
        if !verify{
            return Err(
                HttpError::Simple(StatusCode::UNAUTHORIZED, "invalid_credentials".to_string())
            )
        }

        let token = Token::<Claim<UserClaims>>::generate(
            Claim::from(UserClaims {
                user_id: user.id,
                name: user.name,
                surname: user.surname
            })
        ).await?;

        Ok(
            Json(
                json!({
                    "success": true,
                    "token": token
                })
            )
        )
    }else{
        Err(HttpError::Simple(StatusCode::UNAUTHORIZED, "invalid_credentials".to_string()))
    }
    

}


pub async fn register(
    State(s): State<AppState>,
    ValidatedJson(body): ValidatedJson<RegisterRequest>
) -> Result<Json<Value>, HttpError> {

    let mut tx = s.pool.begin().await?;  

    let hashed_password = hash_password(&body.password).await?;
    let user_id = User::register(
        &mut *tx, 
        &body.email, 
        &body.name, 
        &body.surname,
        &hashed_password
    ).await?;

    let token = Token::<Claim<UserClaims>>::generate(
        Claim::from(UserClaims { 
            user_id: user_id,
            name: body.name,
            surname: body.surname
        })
    ).await?;

    tx.commit().await?;

    Ok(Json(json!({
        "success": true,
        "token": token
    })))
    
}