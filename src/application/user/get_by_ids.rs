use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetUsersByIdsDTO {
    pub ids: Vec<Uuid>,
}


#[derive(Debug, Serialize)]
pub struct UserItemResult{
    id: Uuid,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub type UsersByIdsResultDTO = Vec<UserItemResult>;


pub struct GetUsersByIds<'a> {
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<GetUsersByIdsDTO, UsersByIdsResultDTO> for GetUsersByIds<'_> {
    async fn execute(&self, data: GetUsersByIdsDTO) -> Result<UsersByIdsResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_users(
            self.id_provider.is_auth(),
            self.id_provider.user_id(),
            &data.ids,
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(e) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(e.to_string())
                )
            )
        };
        
        let users = match self.user_reader.get_users_by_ids(&data.ids).await {
            Some(u) => u,
            None => return Err(ApplicationError::NotFound(
                ErrorContent::Message("Запрашиваемые пользователи не найдены".to_string())
            )),

        };
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