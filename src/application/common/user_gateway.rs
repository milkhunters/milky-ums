use uuid::Uuid;
use crate::domain::models::user;

pub trait UserReader {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<user::Model, String>;
    async fn get_user_by_username(&self, username: String) -> Result<user::Model, String>;
    async fn get_user_by_email(&self, email: String) -> Result<user::Model, String>;
}