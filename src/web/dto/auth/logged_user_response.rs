use serde::Serialize;
use utoipa::ToSchema;

use crate::web::models::users::UserModel;

#[derive(Serialize, ToSchema)]
pub struct LoggedUserResponse {
    pub success: bool,
    pub user: UserModel
}