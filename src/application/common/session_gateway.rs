use async_trait::async_trait;
use redis::geo::Unit;
use uuid::Uuid;

use crate::domain::models::session::Session;

#[async_trait]
pub trait SessionReader {
    async fn get_session_by_id(&self, session_id: Uuid) -> Option<Session>;
    async fn get_sessions_by_user_id(&self, user_id: Uuid) -> Vec<Session>;
}
#[async_trait]
pub trait SessionWriter {
    async fn save_session(&self, session: &Session) -> Unit;
    async fn delete_session(&self, session_id: Uuid) -> Unit;
}
