use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub user_id: String,
    pub name: String,
    pub surname: String,   
    pub propic_url: Option<String>
}