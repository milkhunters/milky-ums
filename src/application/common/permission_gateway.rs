use async_trait::async_trait;

use crate::domain::models::permission::{Permission as PermissionDomain, PermissionId, PermissionTextId};
use crate::domain::models::role::RoleId;
use crate::domain::models::service::ServiceId;
use crate::domain::models::user::UserId;

pub enum PermissionGatewayError {
    Critical(String)
}


#[async_trait]
pub trait PermissionReader {
    async fn get_permission_by_id(&self, permission_id: &PermissionId) -> Result<Option<PermissionDomain>, PermissionGatewayError>;
    async fn get_permissions_by_service_id(&self, service_id: &ServiceId) -> Result<Vec<PermissionDomain>, PermissionGatewayError>;
    async fn get_permissions_by_ids(&self, permission_ids: &Vec<PermissionId>) -> Result<Option<Vec<PermissionDomain>>, PermissionGatewayError>;
    async fn get_permissions_by_text_ids(&self, permission_text_ids: &Vec<PermissionTextId>) -> Result<Option<Vec<PermissionDomain>>, PermissionGatewayError>;
    async fn get_permissions_list(&self, limit: &u64, offset: &u64) -> Result<Vec<PermissionDomain>, PermissionGatewayError>;
    async fn get_role_permissions(&self, role_id: &RoleId) -> Result<Vec<PermissionDomain>, PermissionGatewayError>;
    async fn get_user_permissions(&self, user_id: &UserId) -> Result<Vec<PermissionDomain>, PermissionGatewayError>;
}

#[async_trait]
pub trait PermissionWriter {
    async fn save_permission(&self, data: &PermissionDomain) -> Result<(), PermissionGatewayError>;
    async fn save_permissions(&self, data: &Vec<PermissionDomain>) -> Result<(), PermissionGatewayError>;
}

#[async_trait]
pub trait PermissionRemover {
    async fn remove_permission(&self, permission_id: PermissionId) -> Result<(), PermissionGatewayError>;
}

#[async_trait]
pub trait PermissionLinker {
    async fn is_permission_linked_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) -> Result<bool, PermissionGatewayError>;
    async fn link_permission_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) -> Result<(), PermissionGatewayError>;
    async fn link_permissions_to_role(&self, role_id: &RoleId, permission_ids: &Vec<PermissionId>) -> Result<(), PermissionGatewayError>;
    async fn unlink_permission_from_role(&self, role_id: &RoleId, permission_id: &PermissionId) -> Result<(), PermissionGatewayError>;
}


pub trait PermissionGateway: PermissionReader + PermissionWriter + PermissionLinker + Send + Sync {}