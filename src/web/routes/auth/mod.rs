use axum::{routing::get, Router};

use crate::web::AppState;

pub mod root;
pub fn auth_routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(root::index))
        .with_state(state.clone())
}