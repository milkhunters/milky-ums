use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::service_gateway::ServiceGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::service::{ServiceId, ServiceTextId};
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::external::ExternalService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct UpdateServiceDTO {
    pub id: UserId,
    pub title: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize)]
pub struct UpdateServiceResultDTO{
    pub id: ServiceId,
    pub text_id: ServiceTextId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}


pub struct UpdateService<'a> {
    pub service_gateway: &'a dyn ServiceGateway,
    pub external_service: &'a ExternalService,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService
}

impl Interactor<UpdateServiceDTO, UpdateServiceResultDTO> for UpdateService<'_> {
    async fn execute(&self, data: UpdateServiceDTO) -> Result<UpdateServiceResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_service(
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

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_service_title(&data.title).unwrap_or_else(|e| {
            validator_err_map.insert("title".to_string(), e.to_string());
        });
        
        if let Some(description) = &data.description {
            self.validator.validate_service_description(description).unwrap_or_else(|e| {
                validator_err_map.insert("description".to_string(), e.to_string());
            });
        }
        
        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let service = self.service_gateway.get_service_by_id(
            &data.id
        ).await.ok_or(
            ApplicationError::InvalidData(
                ErrorContent::Message("Указанный идентификатор сервиса не найден".to_string())
            )
        )?;
        
        let new_service = self.external_service.update_service(
            service,
            data.title,
            data.description
        );
        
        self.service_gateway.save_service(&new_service).await;
        
        Ok(
            UpdateServiceResultDTO {
                id: new_service.id,
                text_id: new_service.text_id,
                title: new_service.title,
                description: new_service.description,
                created_at: new_service.created_at,
                updated_at: new_service.updated_at,
            }
        )
    }
}