use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::web::{dto::register_request::RegisterRequest, errors::HttpError, extractors::validate_body::ValidatedJson, models::users::User, util::hash_password, AppState};

pub async fn index() -> Result<Json<Value>, HttpError> {

    Ok(Json(
        json!({
            "success": true,
            "message": "user microservice /auth"
        })
    ))

}


pub async fn register(
    State(s): State<AppState>,
    ValidatedJson(body): ValidatedJson<RegisterRequest>
) -> Result<Json<Value>, HttpError> {

    let mut tx = s.pool.begin().await?;  
    
    let hashed_password = hash_password(body.password).await?;
    User::register(
        &mut tx, 
        body.email, 
        body.name, 
        body.surname,
        hashed_password
    ).await?;
    tx.commit().await?;

    Ok(Json(json!({
        "success": true,
        "token": "pog"
    })))
    
}