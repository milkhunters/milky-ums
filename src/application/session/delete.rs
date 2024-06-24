use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionGateway;
use crate::domain::exceptions::DomainError;
use crate::domain::models::session::SessionId;
use crate::domain::services::access::AccessService;

#[derive(Debug, Deserialize)]
pub struct DeleteSessionDTO {
    id: SessionId,
}

pub struct DeleteSession<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<DeleteSessionDTO, ()> for DeleteSession<'_> {
    async fn execute(&self, data: DeleteSessionDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_delete_session(
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

        match self.session_gateway.get_session(&data.id).await {
            Some(session) => session,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Сессия не найдена".to_string())
                )
            )
        };

        self.session_gateway.remove_session(&data.id).await;
        
        Ok(())
    }
}
