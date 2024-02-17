use axum::{extract::State, http::StatusCode, Json};

use crate::web::{dto::{auth::{logged_user_response::LoggedUserResponse, login_request::{LoginRequest, LoginResponse}, register_request::{RegisterRequest, RegisterResponse}}, user_claims::UserClaims, Claim}, errors::HttpError, extractors::{token::Token, validate_body::ValidatedJson}, models::users::User, util::{hash_password, verify_password}, AppState};

#[utoipa::path(
    get,
    path="/auth",
    responses(
        (status = 200, description = "Gets logged user", body = LoggedUserResponse),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn index(
    Token(user): Token<Claim<UserClaims>>
) -> Result<Json<LoggedUserResponse>, HttpError> {

    Ok(Json(
        LoggedUserResponse {
            success: true,
            user: user.data().to_owned()
        }
    ))

}

#[utoipa::path(
    get,
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
    get,
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