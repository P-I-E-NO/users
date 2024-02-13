mod log_util;
pub mod web;
use log::info;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let router = web::build_app().await;
    info!("users microservice built and running");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
