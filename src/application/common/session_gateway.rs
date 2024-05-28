use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::session::{Session, SessionId};

#[async_trait]
pub trait SessionReader {
    async fn get_session(&self, session_id: &SessionId) -> Option<Session>;
    async fn get_sessions(&self, user_id: &Uuid) -> Vec<Session>;
}

#[async_trait]
pub trait SessionWriter {
    async fn save_session(&self, data: &Session);
    async fn delete_session(&self, session_id: &SessionId, user_id: &Uuid);
}

pub trait SessionGateway: SessionReader + SessionWriter {}