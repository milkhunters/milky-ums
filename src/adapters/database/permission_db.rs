use core::option::Option;

use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait};

use crate::adapters::database::models::permissions;
use crate::application::common::permission_gateway::{
    PermissionGateway as PermissionGatewayTrait, 
    PermissionLinker, 
    PermissionReader, 
    PermissionRemover, 
    PermissionWriter
};
use crate::domain::models::permission::{Permission as PermissionDomain, PermissionId};
use crate::domain::models::permission::Permission;
use crate::domain::models::role::RoleId;
use crate::domain::models::service::ServiceId;

pub struct PermissionGateway{
    pub db: Box<DbConn>,
}

impl PermissionGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        PermissionGateway {
            db,
        }
    }
}

#[async_trait]
impl PermissionReader for PermissionGateway {
    async fn get_permission_by_id(&self, permission_id: &PermissionId) -> Option<Permission> {
        todo!()
    }

    async fn get_permissions_by_service_id(&self, service_id: &ServiceId) -> Option<Vec<Permission>> {
        todo!()
    }

    async fn get_permissions_by_ids(&self, permission_ids: &Vec<PermissionId>) -> Option<Vec<Permission>> {
        todo!()
    }

    async fn get_permissions_list(&self, limit: &u64, offset: &u64) -> Vec<Permission> {
        todo!()
    }

    async fn get_role_permissions(&self, user_id: &RoleId) -> Vec<Permission> {
        todo!()
    }
}

#[async_trait]
impl PermissionWriter for PermissionGateway {
    async fn save_permission(&self, data: &Permission) {
        todo!()
    }

    async fn save_permissions(&self, data: &Vec<Permission>) {
        todo!()
    }
}

#[async_trait]
impl PermissionRemover for PermissionGateway {
    async fn remove_permission(&self, permission_id: PermissionId) {
        permissions::Entity::delete_by_id(permission_id).exec(&*self.db).await.unwrap();
    }
}

#[async_trait]
impl PermissionLinker for PermissionGateway {
    async fn link_permission_to_role(&self, role_id: &RoleId, permission_id: &PermissionId) {
        todo!()
    }

    async fn link_permissions_to_role(&self, role_id: &RoleId, permission_ids: &Vec<PermissionId>) {
        todo!()
    }

    async fn unlink_permission_from_role(&self, role_id: &RoleId, permission_id: &PermissionId) {
        todo!()
    }
}

fn map_permission_model_to_domain(permission: permissions::Model) -> PermissionDomain {
    PermissionDomain {
        id: permission.id,
        text_id: permission.text_id,
        service_id: permission.service_id,
        title: permission.title,
        description: permission.description,
        created_at: permission.created_at,
        updated_at: permission.updated_at,
    }
}


impl PermissionGatewayTrait for PermissionGateway {}
