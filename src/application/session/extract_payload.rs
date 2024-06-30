use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionGateway;
use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::service::ServiceTextId;
use crate::domain::models::session::{SessionId, SessionToken};
use crate::domain::models::user::{UserId, UserState};
use crate::domain::services::session::SessionService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Serialize)]
pub struct EPSessionResultDTO{
    pub session_id: SessionId,
    pub user_id: UserId,
    pub user_state: UserState,
    pub permissions: HashMap<ServiceTextId, Vec<PermissionTextId>>
}

pub struct EPSession<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub session_service: &'a SessionService,
    pub session_hasher: &'a dyn Hasher,
    pub id_provider: Box<dyn IdProvider>,
    pub validator_service: &'a ValidatorService,
}

impl Interactor<SessionToken, EPSessionResultDTO> for EPSession<'_> {
    async fn execute(&self, data: SessionToken) -> Result<EPSessionResultDTO, ApplicationError> {
        let session_token_hash = match self.validator_service.validate_session_token(&data) {
            Ok(_) => self.session_hasher.hash(data.as_str()).await,
            Err(error) => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message(error.to_string())
                )
            )
        };
        
        let mut need_update = false;

        let (
            mut session, 
            user_state, 
            permissions
        ) = match self.session_gateway.get_session_by_token_hash_from_cache(
            &session_token_hash
        ).await {
            Some(data) => data,
            None => match self.session_gateway.get_session_by_token_hash(&session_token_hash).await {
                Some(data) => {
                    need_update = true;
                    data
                },
                None => return Err(ApplicationError::Unauthorized(
                    ErrorContent::Message("Токен не существует".to_string())
                ))
            }
        };
        
        if self.session_service.is_session_expired(&session) {
            need_update = true;
        }

        if !self.session_service.verify_session(
            &session,
            self.id_provider.client(),
            self.id_provider.os(),
            self.id_provider.device()
        ) {
            log::warn!("Сессия {} не прошла проверку по отпечатку! IP: {}", session.id, self.id_provider.ip());
            return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("Отпечаток сессии не совпадает с клиентским".to_string())
            ))
        }
        
        if need_update {
            session = self.session_service.update_session(
                session,
                self.id_provider.ip().to_string(),
            );
            self.session_gateway.save_session(&session).await;
            self.session_gateway.save_session_to_cache(
                &session,
                &user_state,
                &permissions
            ).await;
        }
        
        Ok(EPSessionResultDTO{
            session_id: session.id,
            user_id: session.user_id,
            user_state,
            permissions
        })
    }
}
