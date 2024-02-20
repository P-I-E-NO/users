use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UserClaims {
    pub user_id: String,
    pub name: String,
    pub surname: String,   
    pub propic_url: Option<String>
}