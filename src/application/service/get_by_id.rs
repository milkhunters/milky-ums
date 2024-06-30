use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::service_gateway::ServiceReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::service::{ServiceId, ServiceTextId};
use crate::domain::services::access::AccessService;

#[derive(Debug, Serialize)]
pub struct ServiceResultDTO{
    pub id: ServiceId,
    pub text_id: ServiceTextId,
    pub title: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct GetService<'a> {
    pub service_reader: &'a dyn ServiceReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<ServiceId, ServiceResultDTO> for GetService<'_> {
    async fn execute(&self, data: ServiceId) -> Result<ServiceResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_service(
            self.id_provider.is_auth(),
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
        
        let service = self.service_reader.get_service_by_id(
            &data
        ).await.ok_or(
            ApplicationError::InvalidData(
                ErrorContent::Message("Сервис не найден".to_string())
            )
        )?;
        
        Ok(ServiceResultDTO {
            id: service.id,
            text_id: service.text_id,
            title: service.title,
            description: service.description,
            created_at: service.created_at,
            updated_at: service.updated_at,
        })
    }
}