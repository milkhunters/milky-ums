use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionRemover;
use crate::domain::exceptions::DomainError;
use crate::domain::services::access::AccessService;

pub struct DeleteSessionSelf<'a> {
    pub session_remover: &'a dyn SessionRemover,
    pub id_provider: Box<dyn IdProvider>,
    pub access_service: &'a AccessService,
}

impl Interactor<(), ()> for DeleteSessionSelf<'_> {
    async fn execute(&self, _data: ()) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_delete_session_self(
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
        
        self.session_remover.remove_session(
            &self.id_provider.session_id().unwrap(),
        ).await;

        Ok(())
    }
}
