use async_trait::async_trait;

use crate::domain::models::user::{User as UserDomain, UserId};

pub enum UserGatewayError {
    Critical(String)
}

#[async_trait]
pub trait UserReader {
    async fn get_user_by_id(&self, user_id: &UserId) -> Result<Option<UserDomain>, UserGatewayError>;
    async fn get_users_by_ids(&self, user_ids: &Vec<UserId>) -> Result<Option<Vec<UserDomain>>, UserGatewayError>;
    async fn get_users_list(&self, limit: u8, offset: u32) -> Result<Vec<UserDomain>, UserGatewayError>;
    async fn get_user_by_username_not_sensitive(&self, username: &String) -> Result<Option<UserDomain>, UserGatewayError>;
    async fn get_user_by_email_not_sensitive(&self, email: &String) -> Result<Option<UserDomain>, UserGatewayError>;

}

#[async_trait]
pub trait UserWriter {
    async fn save(&self, data: &UserDomain) -> Result<(), UserGatewayError>;
}

pub trait UserGateway: UserReader + UserWriter + Send + Sync {}