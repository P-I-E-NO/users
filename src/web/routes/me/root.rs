use axum::{extract::State, http::StatusCode, Json};

use crate::web::{dto::{me::notifications::NotificationResponse, user_claims::UserClaims, Claim}, errors::HttpError, extractors::token::Token, models::users::User, AppState};

#[utoipa::path(
    get,
    path="/me/notifications",
    responses(
        (status = 200, description = "Notifications fetched correctly", body = NotificationResponse),
        (status = 401, description = "Invalid token sent"),
    ),
)]
pub async fn get_me_notifications(
    State(s): State<AppState>,
    Token(user): Token<Claim<UserClaims>>,
) -> Result<Json<NotificationResponse>, HttpError> {
    let mut conn = s.pool.acquire().await?;
    if let Some(user) = User::from_id(&mut *conn, &user.data().user_id).await? {
        let notifications = user.get_notifications(&mut *conn).await?;
        Ok(
            Json(NotificationResponse {
                success: true, 
                notifications 
            })
        )
    }else{
        Err(
            HttpError::Simple(StatusCode::UNAUTHORIZED, "invalid_credentials".to_string())
        )
    }
}