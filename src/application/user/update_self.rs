use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;

use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::user::{UserId, UserState};
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;


#[derive(Debug, Deserialize)]
pub struct UpdateSelfDTO {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

#[derive(Debug, Serialize)]
pub struct UpdateSelfResultDTO{
    id: UserId,
    email: String,
    username: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct UpdateUserSelf<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService
}

impl Interactor<UpdateSelfDTO, UpdateSelfResultDTO> for UpdateUserSelf<'_> {
    async fn execute(&self, data: UpdateSelfDTO) -> Result<UpdateSelfResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_update_user_self(
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
        };

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        if let Some(first_name) = &data.first_name {
            self.validator.validate_first_name(first_name).unwrap_or_else(|e| {
                validator_err_map.insert("first_name".to_string(), e.to_string());
            });
        }

        if let Some(last_name) = &data.last_name {
            self.validator.validate_last_name(last_name).unwrap_or_else(|e| {
                validator_err_map.insert("last_name".to_string(), e.to_string());
            });
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }

        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(&data.username).await;
        if let Some(user) = user_by_username {
            if &user.id != self.id_provider.user_id().unwrap() {
                validator_err_map.insert(
                    "username".to_string(), 
                    "Имя пользователя занято".to_string()
                );
                return Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Map(validator_err_map)
                    )
                )
            }
        }
        

        let user = self.user_gateway.get_user_by_id(self.id_provider.user_id().unwrap()).await.unwrap();

        let updated_user = self.user_service.update_user_self(
            user,
            data.username,
            data.first_name,
            data.last_name,
        )?;

        Ok(UpdateSelfResultDTO {
            id: updated_user.id,
            email: updated_user.email,
            username: updated_user.username,
            state: updated_user.state,
            first_name: updated_user.first_name,
            last_name: updated_user.last_name,
        })
    }
}