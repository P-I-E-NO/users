use axum::{routing::{get, post}, Router};

use crate::web::AppState;

pub mod root;
pub fn auth_routes(state: &AppState) -> Router {
    Router::new()
        .route("/auth", get(root::index))
        .route("/auth/register", post(root::register))
        .route("/auth/login", post(root::login))
        .with_state(state.clone())
}