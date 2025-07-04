use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::common::error::AppError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::error::ValidationError;
use crate::domain::models::user::UserId;
use crate::domain::services::access::ensure_can_get_user_range;
use crate::domain::services::validator::{validate_page, validate_per_page};

#[derive(Debug, Deserialize)]
pub struct GetUserRangeDTO {
    pub page: u32,
    pub per_page: u8,
}

#[derive(Debug, Serialize)]
pub struct UserItemResult{
    id: UserId,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub type GetUserRangeResultDTO = Vec<UserItemResult>;


pub struct GetUserRange<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'interactor dyn UserReader
}

impl Interactor<GetUserRangeDTO, GetUserRangeResultDTO> for GetUserRange<'_> {
    async fn execute(&self, data: GetUserRangeDTO) -> Result<GetUserRangeResultDTO, AppError> {
        ensure_can_get_user_range(self.id_provider.permissions())?;

        let mut validator_err_map: HashMap<String, ValidationError> = HashMap::new();
        validate_page(data.page).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });
        
        validate_per_page(data.per_page).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });
        
        if !validator_err_map.is_empty() {
            return Err(AppError::Validation(validator_err_map));
        }
        
        let users = self.user_reader.get_users_list(
            data.per_page,
            (data.page * data.per_page as u32).into()
        ).await?;
        
        Ok(users.into_iter().map(|u| UserItemResult {
            id: u.id,
            username: u.username,
            first_name: u.first_name,
            last_name: u.last_name,
        }).collect())
    }
}
