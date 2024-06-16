use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionGateway;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::role::RoleId;
use crate::domain::models::session::SessionToken;
use crate::domain::models::user::UserState;
use crate::domain::services::session::SessionService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct EPSessionDTO {
    pub(crate) session_token: Option<SessionToken>,
}

#[derive(Debug, Serialize)]
pub struct EPSessionResultDTO{
    user_id: Uuid,
    user_state: UserState,
    roles: Vec<(RoleId, Vec<PermissionTextId>)>
}

pub struct EPSession<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub user_gateway: &'a dyn UserReader,
    pub session_service: &'a SessionService,
    pub session_hasher: &'a dyn Hasher,
    pub id_provider: Box<dyn IdProvider>,
    pub validator_service: &'a ValidatorService,
}

impl Interactor<EPSessionDTO, Option<EPSessionResultDTO>> for EPSession<'_> {
    async fn execute(&self, data: EPSessionDTO) -> Result<Option<EPSessionResultDTO>, ApplicationError> {
        let session_token_hash = match data.session_token.clone() {
            Some(session_token) => {
                match self.validator_service.validate_session_token(&session_token) {
                    Ok(_) => self.session_hasher.hash(session_token.as_str()).await,
                    Err(error) => return Err(
                        ApplicationError::InvalidData(
                            ErrorContent::Message(error.to_string())
                        )
                    )
                }
            },
            None => return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("Токен не установлен".to_string())
            ))
        };
        
        let mut need_update = false;

        let (
            mut session, 
            user_state, 
            roles
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
                    ErrorContent::Message("Токен не найден".to_string())
                ))
            }
        };
        
        if self.session_service.is_session_expired(&session) {
            need_update = true;
        }

        if !self.session_service.verify_session(
            &session,
            self.id_provider.user_agent().to_string()
        ) {
            log::warn!("Сессия {} не прошла проверку по отпечатку", session.id);
            return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("Отпечаток сессии не совпадает с клиентским".to_string())
            ))
        }
        
        if need_update {
            session = self.session_service.update_session(
                session,
                self.id_provider.ip().to_string(),
                self.id_provider.user_agent().to_string()
            );
            self.session_gateway.save_session(&session).await;
            self.session_gateway.save_session_to_cache(
                &session,
                &user_state,
                &roles
            ).await;
        }
        
        Ok(Some(EPSessionResultDTO{
            user_id: session.user_id,
            user_state,
            roles
        }))
    }
}
