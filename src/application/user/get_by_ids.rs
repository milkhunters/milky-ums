use serde::{Deserialize, Serialize};

use crate::application::common::error::AppError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;
use crate::domain::services::access::ensure_can_get_users;

#[derive(Debug, Deserialize)]
pub struct GetUsersByIdsDTO {
    pub ids: Vec<UserId>,
}


#[derive(Debug, Serialize)]
pub struct UserItemResult{
    id: UserId,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub type UsersByIdsResultDTO = Vec<UserItemResult>;


pub struct GetUsersByIds<'interactor> {
    pub user_reader: &'interactor dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<GetUsersByIdsDTO, UsersByIdsResultDTO> for GetUsersByIds<'_> {
    async fn execute(&self, data: GetUsersByIdsDTO) -> Result<UsersByIdsResultDTO, AppError> {
        ensure_can_get_users(
            self.id_provider.user_id(),
            &data.ids,
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        )?;
        
        let users = self.user_reader.get_users_by_ids(&data.ids).await?
            .ok_or_else(|| AppError::NotFound("ids".into()))?;
        
        Ok(users.into_iter().map(|u| UserItemResult {
            id: u.id,
            username: u.username,
            first_name: u.first_name,
            last_name: u.last_name,
        }).collect())
    }
}
