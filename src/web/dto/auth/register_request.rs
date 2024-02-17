use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub surname: String,
    #[validate(length(min=8))]
    pub password: String
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub success: bool,
    pub token: String
}