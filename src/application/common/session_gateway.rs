use uuid::Uuid;
use crate::domain::models::session::Session;

pub trait SessionReader {
    async fn get_session_by_id(&self, session_id: Uuid) -> Result<Session, String>;

}