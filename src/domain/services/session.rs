use rand::random;
use uuid::Uuid;

use crate::domain::models::session::{Session, SessionToken, SessionTokenHash};

pub struct SessionService {
    session_expire: u32,
}

impl SessionService {
    
    pub fn new(session_expire: u32) -> SessionService {
        SessionService {
            session_expire,
        }
    }
    
    pub fn is_session_expired(&self, session: &Session) -> bool {
        let session_age = chrono::Utc::now() - session.updated_at.unwrap_or(session.created_at);
        session_age > chrono::Duration::seconds(self.session_expire as i64)
    }
    
    pub fn create_session_token(&self) -> SessionToken {
        (0..64).map(|_| format!("{:02x}", random::<u8>())).collect::<Vec<_>>().join("")
    }

    pub fn create_session(
        &self,
        token_hash: SessionTokenHash,
        user_id: Uuid,
        ip: String,
        user_agent: String,
    ) -> Session {
        Session {
            id: Uuid::new_v4(),
            token_hash,
            user_id,
            ip,
            user_agent,
            created_at: chrono::Utc::now(),
            updated_at: None,
        }
    }

    pub fn verify_session(
        &self,
        session: &Session,
        user_agent: String,
    ) -> bool {
        session.user_agent == user_agent
    }

    pub fn update_session(
        &self,
        session: Session,
        new_ip: String,
        new_user_agent: String,
    ) -> Session {
        Session {
            ip: new_ip,
            user_agent: new_user_agent,
            updated_at: Some(chrono::Utc::now()),
            ..session
        }
    }
}
