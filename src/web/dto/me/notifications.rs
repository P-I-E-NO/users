use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct NotificationResponse {
    pub success: bool,
    pub notifications: Vec<Notification>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema, Debug)]
pub struct Notification {
    id: String,
    to_user: String,
    created_at: String,
    data: Json<NotificationData>,
}

#[derive(sqlx::Decode, Serialize, Deserialize, ToSchema, Debug)]
pub struct NotificationData {
    owner: String,
    car_name: String,
    tank_size: String,
    consumption: String,
}
