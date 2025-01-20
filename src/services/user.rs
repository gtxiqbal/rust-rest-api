use crate::get_message;
use crate::models::dto::user::{UserReq, UserRes};
use crate::models::entity::user_master::UserMaster;
use crate::repositories::user::UserRepo;
use crate::utils::api_response::ApiResponse;
use crate::utils::error::ErrorApp;
use rust_rest_api::transaction;

#[derive(Clone, Debug)]
pub struct UserService<U: UserRepo> {
    user_repo: U,
}

impl<U> UserService<U>
where
    U: UserRepo,
{
    pub fn new(user_repo: U) -> Self {
        Self { user_repo }
    }

    pub async fn get_users(&self) -> Result<ApiResponse<Vec<UserRes>>, ErrorApp> {
        let result = self.user_repo.find_all().await;
        if let Err(err) = result {
            return Err(match err {
                _ => ErrorApp::OtherErr(err.to_string()),
            });
        }

        let user_res = result
            .unwrap()
            .into_iter()
            .map(UserMaster::to_user_res)
            .collect();
        Ok(ApiResponse::success(get_message!("user.get.retrieve.success"), user_res))
    }

    pub async fn get_by_user_id(&self, user_id: String) -> Result<ApiResponse<UserRes>, ErrorApp> {
        let result = self.user_repo.find_by_user_id(user_id).await;

        if let Err(err) = result {
            return Err(match err {
                ErrorApp::RowNotFound => ErrorApp::WithCode("99".to_string(), get_message!("user.get.not.found")),
                _ => ErrorApp::OtherErr(err.to_string()),
            });
        }

        let user_res = result.map(UserMaster::to_user_res).unwrap();
        Ok(ApiResponse::success(get_message!("user.get.retrieve.success"), user_res))
    }

    #[transaction]
    pub async fn create(&self, req: UserReq) -> Result<ApiResponse<()>, ErrorApp> {
        let result = self
            .user_repo
            .create(&mut UserMaster::from_user_req(req))
            .await;
        
        if let Err(err) = result { 
            return Err(match err {
                ErrorApp::DuplicateKey => ErrorApp::WithCode("99".to_string(), get_message!("user.get.already.exists")),
                _ => ErrorApp::OtherErr(err.to_string()),
            });
        }

        Ok(ApiResponse::success(get_message!("user.create.success"), ()))
    }

    #[transaction]
    pub async fn update(&self, req: UserReq) -> Result<ApiResponse<()>, ErrorApp> {
        let result = self
            .user_repo
            .update(&mut UserMaster::from_user_req(req))
            .await;
        
        if let Err(err) = result {
            return Err(match err {
                ErrorApp::RowNotFound => ErrorApp::WithCode("99".to_string(), get_message!("user.get.not.found")),
                _ => ErrorApp::OtherErr(err.to_string()),
            });
        }
        
        Ok(ApiResponse::success(get_message!("user.update.success"), ()))
    }

    #[transaction]
    pub async fn delete_by_userid(&self, user_id: String) -> Result<ApiResponse<()>, ErrorApp> {
        let result = self.user_repo.delete(user_id).await;

        if let Err(err) = result {
            return Err(match err {
                ErrorApp::RowNotFound => ErrorApp::WithCode("99".to_string(), get_message!("user.get.not.found")),
                _ => ErrorApp::OtherErr(err.to_string()),
            });
        }

        Ok(ApiResponse::success(get_message!("user.delete.success"), ()))
    }
}
