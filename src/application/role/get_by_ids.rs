use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;

#[derive(Debug, Deserialize)]
pub struct GetUsersByIdsDTO {
    pub(crate) ids: Vec<Uuid>,
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
    pub user_gateway: &'a dyn UserReader,
}

impl Interactor<GetUsersByIdsDTO, UsersByIdsResultDTO> for GetUsersByIds<'_> {
    async fn execute(&self, data: GetUsersByIdsDTO) -> Result<UsersByIdsResultDTO, ApplicationError> {
        let users = match self.user_gateway.get_users_by_ids(data.ids).await {
            Some(u) => u,
            None => return Err(ApplicationError::NotFound(
                ErrorContent::Message("Запрашиваемые пользователи не найдены".to_string())
            )),

        };
        let mut users_list = Vec::new();
        for u in users {
            users_list.push(UserItemResult {
                id: u.id,
                username: u.username,
                first_name: u.first_name,
                last_name: u.last_name,
            });
        }
        Ok(users_list)
    }
}