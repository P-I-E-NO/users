use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct PutFcmTokenRequest {
    pub token: String
}