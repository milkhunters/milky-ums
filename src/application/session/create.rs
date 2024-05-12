use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::{SessionReader, SessionWriter};
use crate::application::common::user_gateway::UserReader;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

pub trait SessionGateway: SessionReader + SessionWriter {}

#[derive(Debug, Deserialize)]
pub struct CreateSessionDTO {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
struct CreateSessionResult{
    id: Uuid,
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub struct CreateSession<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub user_gateway: &'a dyn UserReader,
    pub user_service: &'a UserService,
    pub session_service: &'a SessionService,
    pub id_provider: &'a dyn IdProvider,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
}

impl Interactor<CreateSessionDTO, CreateSessionResult> for CreateSession<'_> {
    async fn execute(&self, data: CreateSessionDTO) -> Result<CreateSessionResult, ApplicationError> {

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        self.validator.validate_password(&data.password).unwrap_or_else(|e| {
            validator_err_map.insert("password".to_string(), e.to_string());
        });


        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }

        let user = match self.user_gateway.get_user_by_username_not_sensitive(
            data.username.clone()
        ).await {
            Some(user) => user,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Неверная пара имя пользователя и пароль".to_string())
                )
            )
        };

        match self.password_hasher.verify(
            &data.password.as_str(),
            &user.hashed_password.as_str()
        ).await {
            true => true,
            false => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Неверная пара имя пользователя и пароль".to_string())
                )
            )
        };

        let session = self.session_service.create_session(
            self.id_provider.ip(),
            self.id_provider.user_agent()
        )?;

        self.session_gateway.save_session(&session).await?;

        Ok(CreateSessionResult {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
