use async_trait::async_trait;

use crate::domain::models::permission::Permission;
use crate::domain::models::role::{Role as RoleDomain, RoleId};
use crate::domain::models::user::UserId;

#[async_trait]
pub trait RoleReader {
    async fn get_role(&self, role_id: &RoleId) -> Option<RoleDomain>;
    async fn get_roles_by_ids(&self, role_ids: &Vec<RoleId>) -> Option<Vec<RoleDomain>>;
    async fn get_roles(&self) -> Vec<RoleDomain>;
    async fn get_roles_by_user_with_perms(
        &self, 
        user_id: &UserId,
    ) -> Vec<(RoleDomain, Vec<Permission>)>;
    async fn get_role_by_title_not_sensitive(&self, title: &String) -> Option<RoleDomain>;
    async fn get_default_role(&self) -> Option<RoleDomain>;
}

#[async_trait]
pub trait RoleWriter {
    async fn save_role(&self, data: &RoleDomain);
    async fn set_default_role(&self, role_id: &RoleId);
}

#[async_trait]
pub trait RoleLinker {
    async fn link_role_to_user(&self, role_id: &RoleId, user_id: &UserId);
    async fn unlink_role_from_user(&self, role_id: &RoleId, user_id: &UserId);
}

#[async_trait]
pub trait RoleRemover {
    async fn remove_role(&self, role_id: &RoleId);
}

pub trait RoleGateway: RoleReader + RoleWriter + RoleRemover + RoleLinker {}