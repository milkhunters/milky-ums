use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;

#[derive(Debug, Deserialize)]
pub struct GetUserByIdDTO {
    id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UserByIdResultDTO{
    id: Uuid,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct GetUserById<'a> {
    pub user_gateway: &'a dyn UserReader,
}

impl Interactor<GetUserByIdDTO, UserByIdResultDTO> for GetUserById<'_> {
    async fn execute(&self, data: GetUserByIdDTO) -> Result<UserByIdResultDTO, ApplicationError> {
        let user = self.user_gateway.get_user_by_id(data.id).await?;
        Ok(UserByIdResultDTO {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}