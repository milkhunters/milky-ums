use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};

use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionReader;

#[derive(Debug, Deserialize)]
pub struct GetSessionByIdDTO {
    id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SessionByIdResultDTO{
    id: Uuid,
    ip: String,
    user_agent: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


pub struct GetSessionById<'a> {
    pub session_gateway: &'a dyn SessionReader,
}

impl Interactor<GetSessionByIdDTO, SessionByIdResultDTO> for GetSessionById<'_> {
    async fn execute(&self, data: GetSessionByIdDTO) -> Result<SessionByIdResultDTO, ApplicationError> {
        let session = match self.session_gateway.get_session_by_id(data.id).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };
        Ok(SessionByIdResultDTO {
            id: session.id.parse().unwrap(),
            ip: session.ip,
            user_agent: session.user_agent,
            created_at: session.created_at,
            updated_at: session.updated_at,
        })
    }
}