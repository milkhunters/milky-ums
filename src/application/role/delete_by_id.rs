use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::{SessionReader, SessionWriter};
use crate::application::common::user_gateway::UserReader;
use crate::domain::services::access::AccessService;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

pub trait SessionGateway: SessionReader + SessionWriter {}

#[derive(Debug, Deserialize)]
pub struct DeleteSessionDTO {
    id: Uuid,
}

pub struct DeleteSession<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub user_gateway: &'a dyn UserReader,
    pub user_service: &'a UserService,
    pub id_provider: &'a dyn IdProvider,
    pub access_service: &'a AccessService,
}

impl Interactor<DeleteSessionDTO, None> for DeleteSession<'_> {
    async fn execute(&self, data: DeleteSessionDTO) -> Result<None, ApplicationError> {

        self.access_service.

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

        Ok(None)
    }
}
