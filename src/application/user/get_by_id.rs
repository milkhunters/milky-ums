use serde::{Deserialize, Serialize};

use crate::application::common::error::AppError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;
use crate::domain::services::access::ensure_can_get_user;

#[derive(Debug, Deserialize)]
pub struct GetUserByIdDTO {
    pub id: UserId,
}

#[derive(Debug, Serialize)]
pub struct UserByIdResultDTO{
    id: UserId,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct GetUserById<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'interactor dyn UserReader,
}

impl Interactor<GetUserByIdDTO, UserByIdResultDTO> for GetUserById<'_> {
    async fn execute(&self, data: GetUserByIdDTO) -> Result<UserByIdResultDTO, AppError> {
        ensure_can_get_user(
            self.id_provider.user_id(),
            &data.id,
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        )?;
        
        let user = self.user_reader.get_user_by_id(&data.id).await?
            .ok_or_else(|| AppError::NotFound("id".into()))?;

        Ok(UserByIdResultDTO {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
