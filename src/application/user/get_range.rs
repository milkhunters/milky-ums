use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;

#[derive(Debug, Deserialize)]
pub struct GetUserByIdDTO {
    page: u32,
    per_page: u32,
}

#[derive(Debug, Serialize)]
struct UserItemResult{
    id: Uuid,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub type GetUserRangeResultDTO = Vec<UserItemResult>;


pub struct GetUserRange<'a> {
    pub user_gateway: &'a dyn UserReader,
}

impl Interactor<GetUserByIdDTO, GetUserRangeResultDTO> for GetUserRange<'_> {
    async fn execute(&self, data: GetUserByIdDTO) -> Result<GetUserRangeResultDTO, String> {
        let users = match self.user_gateway.get_list().await {
            Ok(user) => user,
            Err(e) => return Err(e),
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