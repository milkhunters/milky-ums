use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::services::access::AccessService;
use crate::domain::services::role::RoleService;


#[derive(Debug, Deserialize)]
pub struct UpdateRoleDTO {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleResultDTO{
    id: Uuid,
    title: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub struct UpdateRole<'a> {
    pub role_gateway: &'a dyn RoleGateway,
    pub role_service: &'a RoleService,
    pub id_provider: &'a dyn IdProvider,
    pub access_service: &'a AccessService,
}

impl Interactor<UpdateRoleDTO, RoleResultDTO> for UpdateRole<'_> {
    async fn execute(&self, data: UpdateRoleDTO) -> Result<RoleResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_role(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return match error {
                DomainError::AccessDenied => Err(
                    ApplicationError::Forbidden(
                        ErrorContent::Message(error.to_string())
                    )
                ),
                DomainError::AuthorizationRequired => Err(
                    ApplicationError::Unauthorized(
                        ErrorContent::Message(error.to_string())
                    )
                )
            }
        };
        
        let old_role = match self.role_gateway.get_role(&data.id).await {
            Some(role) => role,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Роль не найдена".to_string())
                )
            )
        };

        let new_role = match self.role_service.update_role(
            old_role,
            data.title,
            data.description
        ) {
            Ok(role) => role,
            Err(error) => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message(error.to_string())
                )
            )
        };
        
        self.role_gateway.save_role(&new_role).await;
        
        Ok(RoleResultDTO{
            id: new_role.id,
            title: new_role.title,
            description: new_role.description,
            created_at: new_role.created_at,
            updated_at: new_role.updated_at,
        })
    }
}
