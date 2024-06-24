use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::access_log_gateway::AccessLogReader;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::domain::exceptions::DomainError;
use crate::domain::models::access_log::AccessLogId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetAccessLogSelfDTO {
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


pub struct GetAccessLogSelf<'a> {
    pub access_log_reader: &'a dyn AccessLogReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService
}

impl Interactor<GetAccessLogSelfDTO, AccessLogResultDTO> for GetAccessLogSelf<'_> {
    async fn execute(&self, data: GetAccessLogSelfDTO) -> Result<AccessLogResultDTO, ApplicationError> {

        match self.access_service.ensure_can_get_access_log_self(
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
        if data.page == 0 {
            validator_err_map.insert("page".to_string(), "Номер страницы должен быть больше 0".to_string());
        }

        if data.per_page == 0 {
            validator_err_map.insert("per_page".to_string(), "Количество элементов на странице должно быть больше 0".to_string());
        } else if data.per_page > 100 {
            validator_err_map.insert("per_page".to_string(), "Количество элементов на странице должно быть не больше 100".to_string());
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let records = self.access_log_reader.get_user_records(
            self.id_provider.user_id().unwrap(),
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