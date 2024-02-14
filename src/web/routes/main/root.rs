use axum::Json;
use serde_json::{json, Value};

use crate::web::errors::HttpError;

pub async fn index() -> Result<Json<Value>, HttpError> {

    Ok(Json(
        json!({
            "success": true,
            "message": "users microservice"
        })
    ))

}