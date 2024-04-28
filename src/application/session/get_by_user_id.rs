use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionReader;

#[derive(Debug, Deserialize)]
pub struct GetSessionByIdDTO {
    id: Uuid,
}

#[derive(Debug, Serialize)]
struct SessionItemResult{
    id: Uuid,
    ip: String,
    user_agent: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub type SessionsByIdResultDTO = Vec<SessionItemResult>;


pub struct GetSessionByUserId<'a> {
    pub session_gateway: &'a dyn SessionReader,
}

impl Interactor<GetSessionByIdDTO, SessionsByIdResultDTO> for GetSessionByUserId<'_> {
    async fn execute(&self, data: GetSessionByIdDTO) -> Result<SessionsByIdResultDTO, ApplicationError> {
        let session = match self.session_gateway.get_sessions_by_user_id(data.id).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        let mut sessions = Vec::new();
        for s in session {
            sessions.push(SessionItemResult {
                id: s.id.parse().unwrap(),
                ip: s.ip,
                user_agent: s.user_agent,
                created_at: s.created_at,
                updated_at: s.updated_at,
            });
        }
        Ok(sessions)
    }
}