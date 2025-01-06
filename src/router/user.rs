use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, post, put};
use crate::handlers;
use crate::handlers::user::UserState;

pub fn user(user_state: Arc<UserState>) -> Router {
    Router::new()
        .route("/", get(handlers::user::get_users))
        .route("/{user_id}", get(handlers::user::get_user_by_id))
        .route("/", post(handlers::user::created_user))
        .route("/", put(handlers::user::updated_user))
        .route("/{user_id}", delete(handlers::user::deleted_user_by_id))
        .with_state(user_state)
}