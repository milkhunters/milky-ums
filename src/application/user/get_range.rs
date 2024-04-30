use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;

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
    pub user_gateway: &'a dyn UserReader,
}

impl Interactor<GetUserRangeDTO, GetUserRangeResultDTO> for GetUserRange<'_> {
    async fn execute(&self, data: GetUserRangeDTO) -> Result<GetUserRangeResultDTO, ApplicationError> {
        let users = self.user_gateway.get_list(
            data.per_page,
            data.page * data.per_page
        ).await?;

        let mut users_list = Vec::new();
        for user in users {
            users_list.push(UserItemResult {
                id: user.id,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
            });
        }
        Ok(users_list)
    }
}