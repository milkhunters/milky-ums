use async_trait::async_trait;

use crate::domain::models::role::{Role as RoleDomain, RoleId};
use crate::domain::models::user::UserId;

pub enum RoleGatewayError {
    Critical(String)
}


#[async_trait]
pub trait RoleReader {
    async fn get_role(&self, role_id: &RoleId) -> Result<Option<RoleDomain>, RoleGatewayError>;
    async fn get_roles_by_ids(&self, role_ids: &Vec<RoleId>) -> Result<Option<Vec<RoleDomain>>, RoleGatewayError>;
    async fn get_roles_range(
        &self, 
        limit: u64, 
        offset: u64
    ) -> Result<Vec<RoleDomain>, RoleGatewayError>;

    async fn get_user_roles(&self, user_id: &UserId) -> Result<Vec<RoleDomain>, RoleGatewayError>;
    
    async fn get_role_by_title_not_sensitive(&self, title: &String) -> Result<Option<RoleDomain>, RoleGatewayError>;
    async fn get_default_role(&self) -> Result<RoleDomain, RoleGatewayError>;
}

#[async_trait]
pub trait RoleWriter {
    async fn save(&self, data: &RoleDomain) -> Result<(), RoleGatewayError>;
    async fn set_default_role(&self, role_id: &RoleId) -> Result<(), RoleGatewayError>;
}

#[async_trait]
pub trait RoleLinker {
    async fn link_role_to_user(&self, role_id: &RoleId, user_id: &UserId) -> Result<(), RoleGatewayError>;
    async fn unlink_role_from_user(&self, role_id: &RoleId, user_id: &UserId) -> Result<(), RoleGatewayError>;
    async fn is_role_linked_to_user(&self, role_id: &RoleId, user_id: &UserId) -> Result<bool, RoleGatewayError>;
}

#[async_trait]
pub trait RoleRemover {
    async fn remove(&self, role_id: &RoleId) -> Result<(), RoleGatewayError>;
}

pub trait RoleGateway: RoleReader + RoleWriter + RoleRemover + RoleLinker + Sync + Send {}
