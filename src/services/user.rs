use std::sync::Arc;
use crate::get_message;
use crate::models::dto::user::{UserReq, UserRes};
use crate::models::entity::user_master::UserMaster;
use crate::repositories::user::UserRepo;
use crate::utils::api_response::ApiResponse;
use crate::utils::error::ErrorApp;

pub struct UserService<U> {
    user_repo : Arc<U>,
}

impl <U> UserService<U>
where
    U: UserRepo,
{
    pub fn new(user_repo: Arc<U>) -> Self {
        Self {
            user_repo,
        }
    }

    pub async fn get_users(&self) -> ApiResponse<Vec<UserRes>> {
        let result = self.user_repo.find_all().await;
        if let Err(err) = result {
            return match err {
                _ => ApiResponse::failed_internal(err.to_string()),
            };
        }

        let user_res= result.unwrap().into_iter().map(UserMaster::to_user_res).collect();
        ApiResponse::success(get_message!("user.get.retrieve.success"), user_res)
    }

    pub async fn get_by_user_id(&self, user_id: String) -> ApiResponse<UserRes> {
        let result = self.user_repo.find_by_user_id(user_id).await;
        if let Err(err) = result {
            return match err {
                ErrorApp::RowNotFound => ApiResponse::failed_not_found(get_message!("user.get.not.found")),
                _ => ApiResponse::failed_internal(err.to_string()),
            };
        }

        let user_res = result.map(UserMaster::to_user_res).unwrap();
        ApiResponse::success(get_message!("user.get.retrieve.success"), user_res)
    }

    pub async fn create(&self, req: UserReq) -> ApiResponse<()> {
        let result = self.user_repo.create(&mut UserMaster::from_user_req(req)).await;
        match result {
            Ok(_) => ApiResponse::success(get_message!("user.create.success"), ()),
            Err(err) => match err {
                ErrorApp::DuplicateKey => ApiResponse::failed_with_code("99".to_string(), get_message!("user.get.already.exists")),
                _ => ApiResponse::failed_internal(err.to_string())
            }
        }
    }

    pub async fn update(&self, req: UserReq) -> ApiResponse<()> {
        let result = self.user_repo.update(&mut UserMaster::from_user_req(req)).await;
        match result {
            Ok(_) => ApiResponse::success(get_message!("user.update.success"), ()),
            Err(err) => {
                match err {
                    ErrorApp::RowNotFound => ApiResponse::failed_not_found(get_message!("user.get.not.found")),
                    _ => ApiResponse::failed_internal(err.to_string())
                }
            },
        }
    }

    pub async fn delete_by_userid(&self, user_id: String) -> ApiResponse<()> {
        let result = self.user_repo.delete(user_id).await;
        match result {
            Ok(_) => ApiResponse::success(get_message!("user.delete.success"), ()),
            Err(err) => {
                match err {
                    ErrorApp::RowNotFound => ApiResponse::failed_not_found(get_message!("user.get.not.found")),
                    _ => ApiResponse::failed_internal(err.to_string())
                }
            },
        }
    }
}