use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::access_log_gateway::AccessLogWriter;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionGateway;
use crate::application::common::user_gateway::UserReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::session::SessionTokenHash;
use crate::domain::models::user::UserState;
use crate::domain::services::access::AccessService;
use crate::domain::services::access_log::AccessLogService;
use crate::domain::services::session::SessionService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateSessionDTO {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResultDTO{
    id: Uuid,
    username: String,
    email: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub struct CreateSession<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub user_gateway: &'a dyn UserReader,
    pub access_log_writer: &'a dyn AccessLogWriter,
    pub access_log_service: &'a AccessLogService,
    pub session_service: &'a SessionService,
    pub session_hasher: &'a dyn Hasher,
    pub id_provider: Box<dyn IdProvider>,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService
}

impl Interactor<CreateSessionDTO, (CreateSessionResultDTO, SessionTokenHash)> for CreateSession<'_> {
    async fn execute(
        &self, 
        data: CreateSessionDTO
    ) -> Result<(CreateSessionResultDTO, SessionTokenHash), ApplicationError> {

        match self.access_service.ensure_can_create_session(
            self.id_provider.is_auth(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(error) => return match error {
                DomainError::AccessDenied => Err(
                    ApplicationError::Forbidden(
                        ErrorContent::Message(error.to_string())
                    )
                ),
                _ => panic!("Unexpected error")
            }
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

        let mut access_log = self.access_log_service.create_log(
            user.id,
            false,
            self.id_provider.ip().to_string(),
            self.id_provider.client().to_string(),
            self.id_provider.os().to_string(),
            self.id_provider.device().to_string(),
        );

        match self.password_hasher.verify(
            &data.password,
            &user.hashed_password
        ).await {
            true => true,
            false => {
                self.access_log_writer.save_rec(&access_log).await;
                return Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Message("Неверная пара имя пользователя и пароль".to_string())
                    )
                )
            }
        };
        
        if user.state == UserState::Inactive {
            return {
                self.access_log_writer.save_rec(&access_log).await;
                Err(
                    ApplicationError::InvalidData(
                        ErrorContent::Message("Сначала подтвердите свой email".to_string())
                    )
                )
            }
        }
        
        
        let session_token = self.session_service.create_session_token();
        let session_token_hash = self.session_hasher.hash(&session_token).await;

        let session = self.session_service.create_session(
            session_token_hash,
            user.id,
            self.id_provider.ip().to_string(),
            self.id_provider.client().to_string(),
            self.id_provider.os().to_string(),
            self.id_provider.device().to_string(),
        );
        
        self.session_gateway.save_session(&session).await;
        
        access_log.is_success = true;
        self.access_log_writer.save_rec(&access_log).await;

        Ok((
            CreateSessionResultDTO {
                id: user.id,
                username: user.username,
                email: user.email,
                state: user.state,
                first_name: user.first_name,
                last_name: user.last_name,
            },
            session_token
        ))
    }
}
