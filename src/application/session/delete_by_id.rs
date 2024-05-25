use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::{SessionReader, SessionWriter};
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::session::SessionId;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;

trait SessionGateway: SessionReader + SessionWriter {}

#[derive(Debug, Deserialize)]
pub struct DeleteSessionDTO {
    id: SessionId,
}

pub struct DeleteSessionById<'a> {
    pub session_gateway: &'a dyn SessionGateway,
    pub user_gateway: &'a dyn UserReader,
    pub user_service: &'a UserService,
    pub id_provider: &'a dyn IdProvider,
    pub access_service: &'a AccessService,
}

impl Interactor<DeleteSessionDTO, ()> for DeleteSessionById<'_> {
    async fn execute(&self, data: DeleteSessionDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_delete_session(
            self.id_provider.is_auth(),
            self.id_provider.session_id(),
            &data.id,
            self.id_provider.user_state(),
            self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(e) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(e.to_string())
                )
            )
        };

        match self.session_gateway.get_session(&data.id).await {
            Some(session) => session,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Сессия не найдена".to_string())
                )
            )
        };

        self.session_gateway.delete_session(&data.id).await?;

        Ok(())
    }
}
