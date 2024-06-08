use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use crate::application::common::hasher::Hasher;
use crate::application::common::init_state_gateway::InitStateGateway;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::application::common::role_gateway::RoleGateway;

use crate::application::common::service_gateway::ServiceGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::models::permission::{Permission, PermissionId, PermissionTextId};
use crate::domain::models::service::{Service, ServiceTextId};
use crate::domain::models::ums_permission::UMSPermission;
use crate::domain::services::role::RoleService;
use crate::domain::services::user::UserService;

pub async fn service_permissions(
    service_gateway: &dyn ServiceGateway,
    permission_gateway: &dyn PermissionGateway,
    service_name: &ServiceTextId,
) {
    let service = match service_gateway.get_services_by_text_id(
        &service_name
    ).await {
        Some(service) => service,
        None => {
            let service = Service::new(
                service_name.clone(),
                service_name.clone(),
                None,
            );
            service_gateway.save_service(&service).await;
            service
        }
    };

    let permissions_from_app = UMSPermission::iter().map(|permission| {
        permission.to_string()
    }).collect::<Vec<PermissionTextId>>();
    
    let permissions_from_repo = permission_gateway.get_permissions_by_service_id(
        &service.id
    ).await.unwrap_or_default().iter().map(|permission| {
        permission.text_id.clone()
    }).collect::<Vec<PermissionTextId>>();
    
    let permissions_to_add = permissions_from_app.iter().filter(|permission| {
        !permissions_from_repo.contains(permission)
    }).map(|permission| {
        Permission::new(
            permission.clone(),
            service.id,
            permission.clone(),
            None,
        )
    }).collect::<Vec<Permission>>();
    
    permission_gateway.save_permissions(&permissions_to_add).await;

}

pub async fn control_account(
    role_gateway: &dyn RoleGateway,
    role_service: &RoleService,
    permission_gateway: &dyn PermissionGateway,
    user_gateway: &dyn UserGateway,
    service_gateway: &dyn ServiceGateway,
    user_service: &UserService,
    password_hasher: &dyn Hasher,
    init_state_gateway: &dyn InitStateGateway,
    service_name: &ServiceTextId,
) {
    let service_id = match service_gateway.get_services_by_text_id(
        &service_name
    ).await {
        Some(service) => service.id,
        None => panic!("Service not found in control_account init"),
    };
    
    match init_state_gateway.get_state().await {
        Some(_) => return,
        None => {
            let role = role_service.create_role(
                "Control".to_string(),
                Some("Временная роль для инициализации системы".to_string()),
            );
            
            let permissions = vec![
                UMSPermission::GetUserSelf,
                
                UMSPermission::GetUser,
                UMSPermission::CreateUser,
                UMSPermission::UpdateUser,
                UMSPermission::DeleteUser,
                
                UMSPermission::DeleteUser,
                UMSPermission::CreateRole,
                UMSPermission::GetRole,
                UMSPermission::UpdateRole,
                UMSPermission::DeleteRole,
                
                UMSPermission::SetDefaultRole,
                UMSPermission::GetDefaultRole,
                
                UMSPermission::CreatePermission,
                UMSPermission::GetPermission,
                UMSPermission::UpdatePermission,
                UMSPermission::DeletePermission,
                
                UMSPermission::LinkRolePermission,
                UMSPermission::UnlinkRolePermission,
                
                UMSPermission::GetService,
                
                UMSPermission::DeleteSessionSelf,
            ].iter().map(|permission| {
                Permission::new(
                    permission.to_string(),
                    service_id,
                    permission.to_string(),
                    None,
                )
            }).collect::<Vec<Permission>>();
            
            permission_gateway.save_permissions(&permissions).await;
            role_gateway.save_role(&role).await;
            
            permission_gateway.link_permissions_to_role(
                &role.id,
                &permissions.iter().map(
                    |permission| permission.id.clone()
                ).collect::<Vec<PermissionId>>()
            ).await;
            
            // Create control user
            
            let password: String = {
                let mut rng = rand::thread_rng();
                
                let numeric: String = (0..2)
                    .map(|_| rng.gen_range('0'..'9'))
                    .collect();
                
                let alphabetic: String = (0..2)
                    .map(|_| rng.gen_range('A'..'Z'))
                    .collect();

                let alphanumeric: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();

                rng = rand::thread_rng();
                let mut password: Vec<char> = format!(
                    "{}{}{}", numeric, alphabetic, alphanumeric
                ).chars().collect();
                password.shuffle(&mut rng);

                password.into_iter().collect()
            };
            
            let password_hash = password_hasher.hash(password.as_str()).await;
            
            let user = user_service.create_user(
                "control".to_string(),
                "control@milkhunters.ru".to_string(),
                password_hash,
                None,
                None
            );
            
            user_gateway.save_user(&user).await;
            
            role_gateway.link_role_to_user(
                &role.id,
                &user.id
            ).await;
            
            init_state_gateway.set_state(&Utc::now()).await;
            
            log::info!("Control account created");
            log::info!("Login: control");
            log::info!("Password: {}", password);
        }
    }
}