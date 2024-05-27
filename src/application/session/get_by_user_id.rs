use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::session::{Session, SessionId};
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct GetSessionByUserIdDTO {
    id: Uuid,
}

#[derive(Debug, Serialize)]
struct SessionItemResult{
    id: SessionId,
    ip: String,
    user_agent: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub type SessionsByUserIdResultDTO = Vec<SessionItemResult>;


pub struct GetSessionByUserId<'a> {
    pub session_reader: &'a dyn SessionReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService
}

impl Interactor<GetSessionByUserIdDTO, SessionsByUserIdResultDTO> for GetSessionByUserId<'_> {
    async fn execute(&self, data: GetSessionByUserIdDTO) -> Result<SessionsByUserIdResultDTO, ApplicationError> {
        
        
        match self.access_service.ensure_can_get_sessions(
            self.id_provider.is_auth(),
            self.id_provider.user_id(),
            &data.id,
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
        
        let sessions: Vec<Session> = self.session_reader.get_sessions(&data.id).await;
        
        Ok(
            sessions.iter().map(|session| SessionItemResult {
                id: session.id.clone(),
                ip: session.ip.clone(),
                user_agent: session.user_agent.clone(),
                created_at: session.created_at.clone(),
                updated_at: session.updated_at.clone()
            }).collect()
        )
    }
}