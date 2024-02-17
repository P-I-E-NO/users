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
use utoipa::{openapi::security::{Http, HttpAuthScheme, SecurityScheme}, Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::web::{dto::auth::{login_request::{LoginRequest, LoginResponse}, register_request::{RegisterRequest, RegisterResponse}}, routes::auth::auth_routes};

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

    #[derive(OpenApi)]
    #[openapi(
        info(description = "Cars endpoints"),
        paths(
            routes::main::root::index,
            routes::auth::root::login,
            routes::auth::root::register,
        ), 
        modifiers(&SecurityAddon),
        components(
        schemas(
                LoginRequest,
                LoginResponse,
                RegisterRequest,
                RegisterResponse,
            )
        )
    )]
    struct ApiDoc;
    struct SecurityAddon;
    
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }

    let app = Router::new()
        .with_state(state.clone())
        .route("/", get(routes::main::root::index))
        .merge(SwaggerUi::new("/swagger")
            .url("/json-docs", ApiDoc::openapi()))
        .merge(auth_routes(&state));
    app
}
