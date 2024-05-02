use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserState;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResultDTO{
    id: Uuid,
    username: String,
    email: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub struct CreateUser<'a> {
    pub user_gateway: &'a dyn UserReader,
    pub user_service: &'a UserService,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService
}

impl Interactor<CreateUserDTO, CreateUserResultDTO> for CreateUser<'_> {
    async fn execute(&self, data: CreateUserDTO) -> Result<CreateUserResultDTO, ApplicationError> {

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        self.validator.validate_password(&data.password).unwrap_or_else(|e| {
            validator_err_map.insert("password".to_string(), e.to_string());
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
        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(data.username.clone()).await;
        let user_by_email = self.user_gateway.get_user_by_email_not_sensitive(data.email.clone()).await;

        if user_by_username.is_some() {
            validator_err_map.insert("username".to_string(), "Имя пользователя занято".to_string());
        }


        if user_by_email.is_some() {
            validator_err_map.insert("email".to_string(), "Пользователь с таким Email уже существует".to_string());
        }


        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }


        let hashed_password = self.password_hasher.hash(&data.password).await;


        let user = self.user_service.create_user(
            data.username,
            data.email,
            hashed_password,
            data.first_name,
            data.last_name,
        )?;

        self.user_gateway.save_user(&user).await;

        Ok(CreateUserResultDTO {
            id: user.id,
            username: user.username,
            email: user.email,
            state: user.state,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
