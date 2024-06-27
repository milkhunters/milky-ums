use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionReader;
use crate::domain::exceptions::DomainError;
use crate::domain::models::session::SessionId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Serialize)]
pub struct SessionSelfResultDTO{
    id: SessionId,
    ip: String,
    client: String,
    os: String,
    device: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub struct GetSessionSelf<'a> {
    pub session_reader: &'a dyn SessionReader,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<(), SessionSelfResultDTO> for GetSessionSelf<'_> {
    async fn execute(&self, _data: ()) -> Result<SessionSelfResultDTO, ApplicationError> {
        
        match self.access_service.ensure_can_get_session_self(
            self.id_provider.is_auth(),
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

        let session = self.session_reader.get_session(
            &self.id_provider.session_id().unwrap()
        ).await.unwrap();

        Ok(SessionSelfResultDTO {
            id: session.id,
            ip: session.ip,
            client: session.client,
            os: session.os,
            device: session.device,
            created_at: session.created_at,
            updated_at: session.updated_at,
        })
    }
}