mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;
pub mod dto;
pub mod models;
mod util;

use std::{env, time::Duration};

use axum::{extract::FromRef, http::HeaderMap, response::IntoResponse, routing::get, Router};
use log::info;
use sqlx::postgres::{PgPoolOptions, Postgres};
use utoipa::{openapi::security::{Http, HttpAuthScheme, SecurityScheme}, Modify, OpenApi};

use crate::web::{dto::{auth::{logged_user_response::LoggedUserResponse, login_request::{LoginRequest, LoginResponse}, register_request::{RegisterRequest, RegisterResponse}}, me::notifications::NotificationResponse, user_claims::UserClaims}, routes::{auth::auth_routes, me::me_routes}};

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
        info(description = "Users endpoints"),
        paths(
            routes::main::root::index,
            routes::auth::root::login,
            routes::auth::root::index,
            routes::auth::root::register,
            routes::auth::root::add_fcm_token,
            routes::me::root::get_me_notifications,
        ), 
        modifiers(&SecurityAddon),
        components(
        schemas(
                LoginRequest,
                LoginResponse,
                RegisterRequest,
                RegisterResponse,
                LoggedUserResponse,
                UserClaims,
                NotificationResponse
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

    async fn build_json_schema() -> impl IntoResponse {

        let str_docs = ApiDoc::openapi().to_pretty_json().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        (headers, str_docs)
    
    }

    let app = Router::new()
        .with_state(state.clone())
        .route("/", get(routes::main::root::index))
        .route("/json-schema", get(build_json_schema))
        .merge(auth_routes(&state))
        .merge(me_routes(&state));
    app
}