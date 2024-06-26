use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::permission_gateway::PermissionGateway;
use crate::application::common::service_gateway::ServiceGateway;
use crate::domain::models::permission::{Permission, PermissionTextId};
use crate::domain::models::service::ServiceTextId;
use crate::domain::services::external::ExternalService;
use crate::domain::services::permission::PermissionService;

pub struct ServiceSyncDTO {
    pub service_text_id: ServiceTextId,
    pub permission_text_ids: Vec<PermissionTextId>,
}


pub struct ServiceSync<'a> {
    pub service_gateway: &'a dyn ServiceGateway,
    pub permission_gateway: &'a dyn PermissionGateway,
    pub permission_service: &'a PermissionService,
    pub external_service: &'a ExternalService
}

impl Interactor<ServiceSyncDTO, ()> for ServiceSync<'_> {
    async fn execute(&self, data: ServiceSyncDTO) -> Result<(), ApplicationError> {
        let service = match self.service_gateway.get_services_by_text_id(
            &data.service_text_id
        ).await {
            Some(service) => service,
            None => {
                let service = self.external_service.create_service(
                    data.service_text_id.clone(),
                    data.service_text_id.clone(),
                    None,
                );
                self.service_gateway.save_service(&service).await;
                service
            }
        };
        let permission_text_ids_from_repo = self.permission_gateway.get_permissions_by_service_id(
            &service.id
        ).await.iter().map(|permission| {
            permission.text_id.clone()
        }).collect::<Vec<PermissionTextId>>();

        let permissions_to_add = data.permission_text_ids.iter().filter(
            |permission_text_id| {
                !permission_text_ids_from_repo.contains(permission_text_id)
            }
        ).map(|permission| {
            self.permission_service.create_permission(
                permission.clone(),
                service.id,
                permission.clone(),
                None,
            )
        }).collect::<Vec<Permission>>();

        self.permission_gateway.save_permissions(&permissions_to_add).await;
        Ok(())
    }
}
