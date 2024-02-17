use serde::Serialize;
use utoipa::ToSchema;

use crate::web::dto::user_claims::UserClaims;

#[derive(Serialize, ToSchema)]
pub struct LoggedUserResponse {
    pub success: bool,
    pub user: UserClaims
}