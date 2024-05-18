use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;

#[derive(Debug, Deserialize)]
pub struct GetUserSelfDTO {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UserSelfResultDTO{
    id: Uuid,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct GetUserSelf<'a> {
    pub user_gateway: &'a dyn UserReader,
}

impl Interactor<GetUserSelfDTO, UserSelfResultDTO> for GetUserSelf<'_> {
    async fn execute(&self, data: GetUserSelfDTO) -> Result<UserSelfResultDTO, ApplicationError> {
        let user = match self.user_gateway.get_user_by_id(data.id).await {
            Some(u) => u,
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("Пользователь не найден".to_string())
                )),
        };

        Ok(UserByIdResultDTO {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}