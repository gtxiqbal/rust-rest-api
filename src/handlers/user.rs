use crate::models::dto::user::UserReq;
use crate::repositories::db::user::UserRepoDb;
use crate::services::user::UserService;
use crate::utils::api_response::ApiResponse;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

#[derive(Clone, Debug)]
pub struct UserState {
    pub user_service: UserService<UserRepoDb>,
}

pub async fn get_users(State(user_state): State<UserState>) -> impl IntoResponse {
    let result = user_state.user_service.get_users().await;
    ApiResponse::response_from(result).into_response()
}

pub async fn get_user_by_id(
    State(user_state): State<UserState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let result = user_state
        .user_service
        .get_by_user_id(user_id)
        .await;
    ApiResponse::response_from(result).into_response()
}

pub async fn created_user(
    State(user_state): State<UserState>,
    Json(req): Json<UserReq>,
) -> impl IntoResponse {
    let result = user_state.user_service.create(req).await;
    ApiResponse::response_from(result).into_response()
}

pub async fn updated_user(
    State(user_state): State<UserState>,
    Json(req): Json<UserReq>,
) -> impl IntoResponse {
    let result = user_state.user_service.update(req).await;
    ApiResponse::response_from(result).into_response()
}

pub async fn deleted_user_by_id(
    State(user_state): State<UserState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let result = user_state
        .user_service
        .delete_by_userid(user_id)
        .await;
    ApiResponse::response_from(result).into_response()
}
