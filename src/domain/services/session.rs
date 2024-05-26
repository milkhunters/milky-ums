use uuid::Uuid;
use rand::random;

use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::session::{Session, SessionId};

pub struct SessionService { }

impl SessionService {

    pub fn create_session(
        &self,
        user_id: Uuid,
        ip: String,
        user_agent: String,
    ) -> Result<Session, ApplicationError> {
        let id: SessionId = SessionId::from(
            (0..128).map(|_| format!("{:02x}", random::<u8>())).collect::<Vec<_>>().join("")
        );
        
        Ok(Session {
            id,
            user_id,
            ip,
            user_agent,
            created_at: chrono::Utc::now(),
            updated_at: None,
        })
    }

    pub fn verify_session(
        &self,
        session: Session,
        user_agent: String,
    ) -> Result<bool, ApplicationError> {
        Ok(session.user_agent == user_agent)
    }

    pub fn update_session(
        &self,
        session: Session,
        new_ip: String,
        new_user_agent: String,
    ) -> Result<Session, ApplicationError> {
        Ok(Session {
            ip: new_ip,
            user_agent: new_user_agent,
            updated_at: Some(chrono::Utc::now()),
            ..session
        })
    }
}
