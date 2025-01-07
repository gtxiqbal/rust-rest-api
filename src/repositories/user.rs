use crate::models::entity::user_master::UserMaster;
use crate::utils::error::ErrorApp;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_all(&self) -> Result<Vec<UserMaster>, ErrorApp>;
    async fn find_by_user_id(&self, user_id: String) -> Result<UserMaster, ErrorApp>;
    async fn create(&self, user_master: &mut UserMaster) -> Result<(), ErrorApp>;
    async fn update(&self, user_master: &mut UserMaster) -> Result<(), ErrorApp>;
    async fn delete(&self, user_id: String) -> Result<(), ErrorApp>;
}
