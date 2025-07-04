use serde::Serialize;
use crate::application::common::error::AppError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::{UserId, UserState};
use crate::domain::services::access::ensure_can_get_user_self;

#[derive(Debug, Serialize)]
pub struct UserSelfResultDTO{
    id: UserId,
    username: String,
    email: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct GetUserSelf<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'interactor dyn UserReader
}

impl Interactor<(), UserSelfResultDTO> for GetUserSelf<'_> {
    async fn execute(&self, _data: ()) -> Result<UserSelfResultDTO, AppError> {
        ensure_can_get_user_self(
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        )?;
        
        let user = self.user_reader.get_user_by_id(self.id_provider.user_id()).await?
            .ok_or_else(|| AppError::Critical(format!(
                "self user not found: get_user_by_id: id:{}",
                self.id_provider.user_id()
            )))?;

        Ok(UserSelfResultDTO {
            id: user.id,
            username: user.username,
            email: user.email,
            state: user.state,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}