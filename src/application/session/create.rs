use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::{SessionReader, SessionWriter};
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::session::SessionId;
use crate::domain::services::access::AccessService;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

trait SessionGateway: SessionReader + SessionWriter {}

#[derive(Debug, Deserialize)]
pub struct CreateSessionDTO {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
struct CreateSessionResultDTO{
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
    pub access_service: &'a AccessService
}

impl Interactor<CreateSessionDTO, (CreateSessionResultDTO, SessionId)> for CreateSession<'_> {
    async fn execute(&self, data: CreateSessionDTO) -> Result<(CreateSessionResultDTO, SessionId), ApplicationError> {

        match self.access_service.ensure_can_create_session(
            self.id_provider.is_auth(),
            self.id_provider.user_state(),
            self.id_provider.permissions()
            
        ) {
            Ok(_) => (),
            Err(e) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(e.to_string())
                )
            )
        };

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
        
        let user = match self.user_gateway.get_user_by_username_not_sensitive(&data.username).await {
            Some(user) => user,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Неверная пара имя пользователя и пароль".to_string())
                )
            )
        };

        match self.password_hasher.verify(
            &data.password,
            &user.hashed_password
        ).await {
            true => true,
            false => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Неверная пара имя пользователя и пароль".to_string())
                )
            )
        };

        let session = self.session_service.create_session(
            user.id,
            self.id_provider.ip().to_string(),
            self.id_provider.user_agent().to_string()
        )?;

        self.session_gateway.save_session(&session).await?;

        Ok((
            CreateSessionResultDTO {
                id: user.id,
                username: user.username,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
            },
            session.id
        ))
    }
}
