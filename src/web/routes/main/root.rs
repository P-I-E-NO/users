use std::env;

use axum::{http::StatusCode, Json};
use ::futures::future::{try_join_all};
use serde_json::{json, Value};
use tokio::sync::futures;

use crate::web::errors::HttpError;

#[utoipa::path(
    get,
    path="/",
    responses(
        (status = 200, description = "Welcome endpoint")
    ),
)]
pub async fn index() -> Result<Json<Value>, HttpError> {

    Ok(Json(
        json!({
            "success": true,
            "message": "users microservice"
        })
    ))

}