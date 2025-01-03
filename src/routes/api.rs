use crate::controllers::user_controller::UserController;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use std::sync::Arc;
use axum::Json;
use crate::models::dto::user::UserReq;

pub async fn get_users(State(user_controller): State<Arc<UserController>>) -> impl IntoResponse {
    user_controller.get_users().await.into_response()
}

pub async fn get_user_by_id(State(user_controller): State<Arc<UserController>>, Path(user_id): Path<String>) -> impl IntoResponse {
    user_controller.get_by_user_id(user_id).await.into_response()
}

pub async fn created_user(State(user_controller): State<Arc<UserController>>, Json(req): Json<UserReq>) -> impl IntoResponse {
    user_controller.create(req).await.into_response()
}

pub async fn updated_user(State(user_controller): State<Arc<UserController>>, Json(req): Json<UserReq>) -> impl IntoResponse {
    user_controller.update(req).await.into_response()
}

pub async fn deleted_user_by_id(State(user_controller): State<Arc<UserController>>, Path(user_id): Path<String>) -> impl IntoResponse {
    user_controller.delete_by_user_id(user_id).await.into_response()
}