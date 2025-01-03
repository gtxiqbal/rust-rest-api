use std::env;
use crate::config::setting;
use axum::http::Error;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};
use log::info;
use tower::ServiceBuilder;

mod config;
mod controllers;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let result = setting::Setting::new();
    if let Err(err) = result {
        panic!("panic load setting: {}", err.to_string())
    }

    let resources_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
    log4rs::init_file(format!("{resources_path}/log4rs.yaml"), Default::default()).unwrap();

    let setting = result.unwrap();

    let result = config::pg_conn::conn(&setting).await;
    if let Err(err) = result {
        panic!("panic database: {}", err.to_string())
    }

    utils::messages::init_message().await?;

    let conn = result.unwrap();
    let user_repo = repositories::db::user_repo_db::UserRepoDb::new(conn.clone());
    let user_service = services::user_service::UserService::new(user_repo.clone());
    let user_controller = controllers::user_controller::UserController::new(user_service.clone());

    let app = Router::new()
        .route("/api/v1/users", get(routes::api::get_users))
        .route("/api/v1/users/{user_id}", get(routes::api::get_user_by_id))
        .route("/api/v1/users", post(routes::api::created_user))
        .route("/api/v1/users", put(routes::api::updated_user))
        .route("/api/v1/users/{user_id}", delete(routes::api::deleted_user_by_id))
        .layer(ServiceBuilder::new()
            .layer(middleware::from_fn(middlewares::language::language_middleware))
            .layer(middleware::from_fn(middlewares::auth::auth_middleware))
            .layer(middleware::from_fn(middlewares::log_std::log_std_middleware))
        )
        .with_state(user_controller);

    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("start server on port {}", port);
    axum::serve(listener, app).await
}