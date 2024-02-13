use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
    user_id: String
}