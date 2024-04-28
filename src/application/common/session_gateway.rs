use uuid::Uuid;
use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::session::Session;

#[async_trait]
pub trait SessionReader {
    async fn get_session_by_id(&self, session_id: Uuid) -> Result<Session, ApplicationError>;
    async fn get_sessions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, ApplicationError>;

}
#[async_trait]
pub trait SessionWriter {
    async fn create_session(&self, session: Session) -> Result<(), ApplicationError>;
    async fn delete_session(&self, session_id: Uuid) -> Result<(), ApplicationError>;
}
