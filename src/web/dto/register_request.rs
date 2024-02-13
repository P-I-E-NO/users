use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    pub name: String,
    pub surname: String,
    #[validate(length(min=8))]
    pub password: String
}