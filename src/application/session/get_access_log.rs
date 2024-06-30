use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::access_log_gateway::AccessLogReader;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::domain::exceptions::DomainError;
use crate::domain::models::access_log::AccessLogId;
use crate::domain::models::user::UserId;
use crate::domain::services::access::AccessService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct GetAccessLogDTO {
    pub user_id: UserId,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct AccessLogItemResult{
    pub id: AccessLogId,
    pub is_success: bool,

    pub ip: String,
    pub client: String,
    pub os: String,
    pub device: String,

    pub created_at: DateTime<Utc>,
}

pub type AccessLogResultDTO = Vec<AccessLogItemResult>;


pub struct GetAccessLog<'a> {
    pub access_log_reader: &'a dyn AccessLogReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService,
}

impl Interactor<GetAccessLogDTO, AccessLogResultDTO> for GetAccessLog<'_> {
    async fn execute(&self, data: GetAccessLogDTO) -> Result<AccessLogResultDTO, ApplicationError> {

        match self.access_service.ensure_can_get_access_log(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            &self.id_provider.permissions()
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

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_page(&data.page).unwrap_or_else(|e| {
            validator_err_map.insert("page".to_string(), e.to_string());
        });

        self.validator.validate_per_page(&data.per_page).unwrap_or_else(|e| {
            validator_err_map.insert("per_page".to_string(), e.to_string());
        });

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        let records = self.access_log_reader.get_user_records(
            &data.user_id,
            &data.per_page,
            &(data.page * data.per_page)
        ).await;

        Ok(
            records.iter().map(|session| AccessLogItemResult {
                id: session.id.clone(),
                is_success: session.is_success.clone(),
                ip: session.ip.clone(),
                client: session.client.clone(),
                os: session.os.clone(),
                device: session.device.clone(),
                created_at: session.created_at.clone(),
            }).collect()
        )
    }
}