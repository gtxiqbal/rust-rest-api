use crate::models::dto::user::UserReq;
use crate::repositories::db::user::UserRepoDb;
use crate::services::user::UserService;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UserState {
    pub user_service: Arc<UserService<UserRepoDb>>,
}

pub async fn get_users(State(user_state): State<Arc<UserState>>) -> impl IntoResponse {
    user_state.user_service.get_users().await.into_response()
}

pub async fn get_user_by_id(
    State(user_state): State<Arc<UserState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    user_state
        .user_service
        .get_by_user_id(user_id)
        .await
        .into_response()
}

pub async fn created_user(
    State(user_state): State<Arc<UserState>>,
    Json(req): Json<UserReq>,
) -> impl IntoResponse {
    user_state.user_service.create(req).await.into_response()
}

pub async fn updated_user(
    State(user_state): State<Arc<UserState>>,
    Json(req): Json<UserReq>,
) -> impl IntoResponse {
    user_state.user_service.update(req).await.into_response()
}

pub async fn deleted_user_by_id(
    State(user_state): State<Arc<UserState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    user_state
        .user_service
        .delete_by_userid(user_id)
        .await
        .into_response()
}
