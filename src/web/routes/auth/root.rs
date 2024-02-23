use axum::{debug_handler, extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use crate::web::{dto::{auth::{logged_user_response::LoggedUserResponse, login_request::{LoginRequest, LoginResponse}, put_fcm_token_request::PutFcmTokenRequest, register_request::{RegisterRequest, RegisterResponse}}, user_claims::UserClaims, Claim}, errors::HttpError, extractors::{token::Token, validate_body::ValidatedJson}, models::users::{User, UserModel}, util::{hash_password, verify_password}, AppState};

#[utoipa::path(
    get,
    path="/auth/",
    responses(
        (status = 200, description = "Gets logged user", body = LoggedUserResponse),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn index(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>
) -> Result<Json<LoggedUserResponse>, HttpError> {

    let mut conn = s.pool.acquire().await?;
    if let Some(user) = User::from_id(&mut *conn, &user.data().user_id).await? {

        Ok(Json(
            LoggedUserResponse {
                success: true,
                user: UserModel {
                    email: user.email,
                    name: user.name,
                    surname: user.surname,
                    id: user.id,
                    propic_url: user.propic_url,
                }
            }
        ))

    }else{
        Err(HttpError::Simple(StatusCode::NOT_FOUND, "user_not_found".to_string()))
    }

}

#[utoipa::path(
    post,
    path="/auth/login",
    responses(
        (status = 200, description = "Login successful. Outputs a token the user must use to make authenticated requests.", body = LoginResponse),
        (status = 401, description = "Invalid credentials: either the email and/or the password is invalid."),
    ),
    params(
        ("email" = String, Path, description = "User email"),
        ("password" = String, Path, description = "User password"),
    ),
)]
pub async fn login(
    State(s): State<AppState>,
    ValidatedJson(body): ValidatedJson<LoginRequest>
) -> Result<Json<LoginResponse>, HttpError> {

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
                surname: user.surname,
                propic_url: user.propic_url
            })
        ).await?;

        Ok(
            Json(
                LoginResponse {
                    success: true,
                    token: token
                } 
            )
        )
    }else{
        Err(HttpError::Simple(StatusCode::UNAUTHORIZED, "invalid_credentials".to_string()))
    }
    

}

#[utoipa::path(
    post,
    path="/auth/register",
    responses(
        (status = 200, description = "Registration successful. Outputs a token the user must use to make authenticated requests.", body = RegisterResponse),
        (status = 409, description = "The email is already registered."),
    ),
    params(
        ("email" = String, Path, description = "User email"),
        ("name" = String, Path, description = "User name"),
        ("surname" = String, Path, description = "User surname"),
        ("password" = String, Path, description = "User password"),
    ),
)]
pub async fn register(
    State(s): State<AppState>,
    ValidatedJson(body): ValidatedJson<RegisterRequest>
) -> Result<Json<RegisterResponse>, HttpError> {

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
            surname: body.surname,
            propic_url: None
        })
    ).await?;

    tx.commit().await?;

    Ok(Json(
            RegisterResponse{
                success: true,
                token: token
            }
        )
    )
    
}

#[utoipa::path(
    put,
    path="/auth/fcm",
    responses(
        (status = 200, description = "Token successfully inserted."),
        (status = 400, description = "The account you made the request with is no longer available")
    ),
    params(
        ("token" = String, Path, description = "The Firebase Cloud Messaging token"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[debug_handler]
pub async fn add_fcm_token(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>,
    ValidatedJson(body): ValidatedJson<PutFcmTokenRequest>,
) -> Result<Json<Value>, HttpError> {
    
    let mut tx = s.pool.begin().await?;
    if let Some(user) = User::from_id(&mut *tx, &user.data().user_id).await? {
        let result = user.add_fcm_token(&mut *tx, &body.token).await;
        tx.commit().await?;
        
        if let Err(err) = result {       
            // if the token is already inside the db we want to return 200 OK anyways      
            match err {
                sqlx_core::Error::Database(db_err) => {
                    if let Some(code) = db_err.code() {
                        if code == "23505" {
                            return Ok(
                                Json(
                                    json!({"success": true, "message": "fcm already in db"})
                                )
                            )
                        } else {
                            return Err(HttpError::DbError(sqlx::Error::Database(db_err)))
                        }
                    } else {
                       return Err(HttpError::DbError(sqlx::Error::Database(db_err)))
                    }
                }
                e => return Err(HttpError::DbError(e))
            }
        }
        Ok(
            Json(
                json!({"success": true})
            )
        )
    }else{
        // somebody has forged the token (zamn...)
        // or maybe somebody is trying to make a request with a token that belongs to a deleted account
        Err(HttpError::Simple(StatusCode::BAD_REQUEST, "account_unavailable".to_string()))
    }
}