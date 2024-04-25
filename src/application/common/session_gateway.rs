use uuid::Uuid;
use async_trait::async_trait;
use crate::domain::models::session::Session;

#[async_trait]
pub trait SessionReader {
    async fn get_session_by_id(&self, session_id: Uuid) -> Result<Session, String>;
    async fn get_sessions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, String>;

}
#[async_trait]
pub trait SessionWriter {
    async fn create_session(&self, session: Session) -> Result<(), String>;
    async fn delete_session(&self, session_id: Uuid) -> Result<(), String>;
}
