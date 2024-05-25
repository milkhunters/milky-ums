use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::{UserReader, UserWriter};
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::UserState;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;

pub trait UserGateway: UserReader + UserWriter {}

#[derive(Debug, Deserialize)]
pub struct UpdateUserByIdDTO {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub state: UserState,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserByIdResultDTO{
    id: Uuid,
    email: String,
    username: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct UpdateUserById<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub id_provider: &'a dyn IdProvider,
    pub access_service: AccessService,
}

impl Interactor<UpdateUserByIdDTO, UserByIdResultDTO> for UpdateUserById<'_> {
    async fn execute(&self, data: UpdateUserByIdDTO) -> Result<UserByIdResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_user(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            &self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return match error {
                DomainError::AccessDenied => Err(
                    ApplicationError::Forbidden(
                        ErrorContent::Message(error.to_string())
                    )
                ),
                DomainError::AuthorizationRequired => Err(
                    ApplicationError::Unauthorized(
                        ErrorContent::Message(error.to_string())
                    )
                )
            }
        }

        let user = match self.user_gateway.get_user_by_id(&data.id).await {
            Some(user) => user,
            None => {
                return Err(ApplicationError::NotFound(
                    ErrorContent::Message("Пользователь не найден".to_string()))
                );
            }
        };

        self.user_gateway.save_user(&user).await;

        Ok(UserByIdResultDTO {
            id: user.id,
            username: user.username,
            email: user.email,
            state: user.state,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}