use async_trait::async_trait;
use uuid::Uuid;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::user;

#[async_trait]
pub trait UserReader {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<user::Model, ApplicationError>;
    async fn get_user_by_username(&self, username: String) -> Result<user::Model, ApplicationError>;
    async fn get_user_by_email(&self, email: String) -> Result<user::Model, ApplicationError>;
    async fn get_list(&self) -> Result<Vec<user::Model>, ApplicationError>;
}