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
pub struct UpdateUserDTO {
    pub id: UserId,
    pub email: String,
    pub username: String,
    pub state: UserState,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserResultDTO{
    id: UserId,
    email: String,
    username: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}


pub struct UpdateUser<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
    pub validator: &'a ValidatorService
}

impl Interactor<UpdateUserDTO, UpdateUserResultDTO> for UpdateUser<'_> {
    async fn execute(&self, data: UpdateUserDTO) -> Result<UpdateUserResultDTO, ApplicationError> {
        
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

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });
        
        self.validator.validate_email(&data.email).unwrap_or_else(|e| {
            validator_err_map.insert("email".to_string(), e.to_string());
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

        // Todo: to gather
        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(&data.username).await;
        let user_by_email = self.user_gateway.get_user_by_email_not_sensitive(&data.email).await;

        // let (user_by_username, user_by_email) = match tokio::try_join!(
        //     tokio::spawn(async move { self.user_gateway.get_user_by_username_not_sensitive(&data.username).await }),
        //     tokio::spawn(async move { self.user_gateway.get_user_by_email_not_sensitive(&data.email).await })
        // ) {
        //     Ok((user_by_username, user_by_email)) => (user_by_username, user_by_email),
        //     Err(e) => panic!("Error: {:?}", e)
        // };
        
        if user_by_username.is_some() && user_by_username.unwrap().id != data.id {
            validator_err_map.insert("username".to_string(), "Имя пользователя занято".to_string());
        }
        
        if user_by_email.is_some() && user_by_email.unwrap().id != data.id{
            validator_err_map.insert("email".to_string(), "Email занят".to_string());
        }
        
        

        let user = match self.user_gateway.get_user_by_id(&data.id).await {
            Some(user) => user,
            None => {
                return Err(ApplicationError::NotFound(
                    ErrorContent::Message("Пользователь не найден".to_string()))
                );
            }
        };
        
        let new_user = self.user_service.update_user(
            user.clone(),
            data.email,
            data.username,
            data.state,
            data.first_name,
            data.last_name,
            user.hashed_password
        );

        self.user_gateway.save_user(&new_user).await;

        Ok(UpdateUserResultDTO {
            id: new_user.id,
            username: new_user.username,
            email: new_user.email,
            state: new_user.state,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
        })
    }
}