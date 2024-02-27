use axum::{routing::get, Router};

use crate::web::AppState;

pub mod root;

pub fn me_routes(state: &AppState) -> Router {
    Router::new()
        .route("/me/notifications", get(root::get_me_notifications))
        .with_state(state.clone())
}