use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetUserRangeDTO {
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct UserItemResult{
    id: Uuid,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub type GetUserRangeResultDTO = Vec<UserItemResult>;


pub struct GetUserRange<'a> {
    pub user_reader: &'a dyn UserReader,
    pub id_provider: &'a dyn IdProvider,
    pub access_service: &'a AccessService,
}

impl Interactor<GetUserRangeDTO, GetUserRangeResultDTO> for GetUserRange<'_> {
    async fn execute(&self, data: GetUserRangeDTO) -> Result<GetUserRangeResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_user_range(
            &self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(error.to_string())
                )
            )
        };

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        if data.page == 0 {
            validator_err_map.insert("page".to_string(), "Страница должна быть больше 0".to_string());
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
        
        let users = self.user_reader.get_users_list(
            &data.per_page,
            &(data.page * data.per_page)
        ).await;
        
        Ok(
            users.into_iter().map(|u| UserItemResult {
                id: u.id,
                username: u.username,
                first_name: u.first_name,
                last_name: u.last_name,
            }).collect()
        )
    }
}