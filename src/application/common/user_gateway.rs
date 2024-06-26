use async_trait::async_trait;

use crate::domain::models::user::{User as UserDomain, UserId};

#[async_trait]
pub trait UserReader {
    async fn get_user_by_id(&self, user_id: &UserId) -> Option<UserDomain>;
    async fn get_users_by_ids(&self, user_ids: &Vec<UserId>) -> Option<Vec<UserDomain>>;
    async fn get_users_list(&self, limit: &u64, offset: &u64) -> Vec<UserDomain>;
    async fn get_user_by_username_not_sensitive(&self, username: &String) -> Option<UserDomain>;
    async fn get_user_by_email_not_sensitive(&self, email: &String) -> Option<UserDomain>;

}

#[async_trait]
pub trait UserWriter {
    async fn save_user(&self, data: &UserDomain);
}

pub trait UserGateway: UserReader + UserWriter {}