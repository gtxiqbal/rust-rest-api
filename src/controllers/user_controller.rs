use std::sync::Arc;
use crate::models::dto::user::{UserReq, UserRes};
use crate::services::user_service::UserService;
use crate::utils::api_response::ApiResponse;

pub struct UserController {
    user_service: Arc<UserService>
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> Arc<Self> {
        Arc::new(Self {user_service})
    }

    pub async fn get_users(&self) -> ApiResponse<Vec<UserRes>> {
        self.user_service.get_users().await
    }

    pub async fn get_by_user_id(&self, user_id: String) -> ApiResponse<UserRes> {
        self.user_service.get_by_user_id(user_id).await
    }

    pub async fn create(&self, req: UserReq) -> ApiResponse<()> {
        self.user_service.create(req).await
    }

    pub async fn update(&self, req: UserReq) -> ApiResponse<()> {
        self.user_service.update(req).await
    }

    pub async fn delete_by_user_id(&self, user_id: String) -> ApiResponse<()> {
        self.user_service.delete_by_userid(user_id).await
    }
}