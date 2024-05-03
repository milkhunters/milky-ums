use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::user::User as UserDomain;

#[async_trait]
pub trait UserReader {
    async fn get_user_by_id(&self, user_id: Uuid) -> Option<UserDomain>;
    async fn get_users_by_ids(&self, user_ids: Vec<Uuid>) -> Option<Vec<UserDomain>>;
    async fn get_users_list(&self, limit: u64, offset: u64) -> Vec<UserDomain>;
    async fn save_user(&self, data: &UserDomain);
    async fn get_user_by_username_not_sensitive(&self, username: String) -> Option<UserDomain>;
    async fn get_user_by_email_not_sensitive(&self, email: String) -> Option<UserDomain>;
}