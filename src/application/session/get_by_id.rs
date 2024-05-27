use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::session::SessionId;
use crate::domain::services::access::AccessService;


#[derive(Debug, Deserialize)]
pub struct GetSessionByIdDTO {
    id: SessionId,
}

#[derive(Debug, Serialize)]
pub struct SessionByIdResultDTO{
    id: SessionId,
    ip: String,
    user_agent: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub struct GetSessionById<'a> {
    pub session_reader: &'a dyn SessionReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}SessionReader

impl Interactor<GetSessionByIdDTO, SessionByIdResultDTO> for GetSessionById<'_> {
    async fn execute(&self, data: GetSessionByIdDTO) -> Result<SessionByIdResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_session(
            self.id_provider.is_auth(),
            self.id_provider.session_id(),
            &data.id,
            self.id_provider.user_state(),
            self.id_provider.permissions()
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
        
        let session = match self.session_reader.get_session(&data.id).await {
            Some(session) => session,
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("Запрашиваемая сессия не найдена".to_string())
                )
            ),
        };
        
        Ok(SessionByIdResultDTO {
            id: session.id,
            ip: session.ip,
            user_agent: session.user_agent,
            created_at: session.created_at,
            updated_at: session.updated_at,
        })
    }
}