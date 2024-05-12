use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::{UserReader, UserWriter};
use crate::domain::models::user::UserState;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;


pub trait UserGateway: UserReader + UserWriter {}

#[derive(Debug, Deserialize)]
pub struct UpdateSelfByIdDTO {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

#[derive(Debug, Serialize)]
pub struct SelfByIdResultDTO{
    id: Uuid,
    email: String,
    username: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct UpdateSelfById<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub id_provider: &'a dyn IdProvider,
    pub access_service: AccessService,
}

impl Interactor<UpdateSelfByIdDTO, SelfByIdResultDTO> for UpdateSelfById<'_> {
    async fn execute(&self, data: UpdateSelfByIdDTO) -> Result<SelfByIdResultDTO, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Forbidden( // todo: change to Unauthorized ??
                ErrorContent::Message("У вас нет доступа к этому ресурсу".to_string()))
            );
        }

        let current_user_id = self.id_provider.user_id();
        let user = match self.user_gateway.get_user_by_id(current_user_id).await {
            Some(user) => user,
            None => {
                return Err(ApplicationError::NotFound(
                    ErrorContent::Message("Пользователь не найден".to_string()))
                );
            }
        };

        let updated_user = self.user_service.update_user(
            user,
            data.username,
            data.first_name,
            data.last_name,
        )?;

        Ok(SelfByIdResultDTO {
            id: updated_user.id,
            email: updated_user.email,
            username: updated_user.username,
            state: updated_user.state,
            first_name: updated_user.first_name,
            last_name: updated_user.last_name,
        })
    }
}