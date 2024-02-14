mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;
pub mod dto;
pub mod models;
mod util;

use std::{env, time::Duration};

use axum::{extract::FromRef, routing::{get, post}, Router};
use log::info;
use sqlx::postgres::{PgPoolOptions, Postgres};

use crate::web::routes::auth::auth_routes;

#[derive(Clone, FromRef)]
pub struct AppState {
    pool: sqlx::Pool<Postgres>,
}
impl AppState {
    pub async fn new() -> Result<AppState, anyhow::Error> {
        info!("acquiring pool...");

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .max_connections(1)
            .connect(&env::var("CONN_URI").unwrap())
            .await?;

        Ok(AppState { pool })
    }
}

pub async fn build_app() -> Router {
    let state = AppState::new().await.unwrap();
    info!("state ok");
    let app = Router::new()
        .with_state(state.clone())
        .route("/", get(routes::main::root::index))
        .merge(auth_routes(&state));
    app
}
