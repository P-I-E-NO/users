use axum::{routing::{get, post, put}, Router};
use crate::web::AppState;
pub mod root;

pub fn auth_routes(state: &AppState) -> Router {
    Router::new()
        .route("/auth", get(root::index))
        .route("/auth/register", post(root::register))
        .route("/auth/login", post(root::login))
        .route("/auth/fcm", put(root::add_fcm_token))
        .with_state(state.clone())
}