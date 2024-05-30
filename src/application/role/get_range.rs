use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::role::RoleId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetRoleRangeDTO {
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct RoleItemResult{
    id: RoleId,
    title: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub type GetRoleRangeResultDTO = Vec<RoleItemResult>;


pub struct GetRoleRange<'a> {
    pub role_gateway: &'a dyn RoleReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<GetRoleRangeDTO, GetRoleRangeResultDTO> for GetRoleRange<'_> {
    async fn execute(&self, data: GetRoleRangeDTO) -> Result<GetRoleRangeResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_role(
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
        }
        
        let roles = self.role_gateway.get_roles_list(
            &data.per_page,
            &(data.page * data.per_page)
        ).await;

        
        Ok(
            roles.iter().map(|role| RoleItemResult{
                id: role.id,
                title: role.title.clone(),
                description: role.description.clone(),
                created_at: role.created_at.to_string(),
                updated_at: role.updated_at.map(|date| date.to_string())
            }).collect()
        )
        
    }
}