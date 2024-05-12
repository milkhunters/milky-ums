use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::session::Session;

pub struct SessionService { }

impl SessionService {

    pub fn create_session(
        &self,
        ip: String,
        user_agent: String,
    ) -> Result<Session, ApplicationError> {
        Ok(Session {
            id: Uuid::new_v4().to_string(),
            ip,
            user_agent,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
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
            updated_at: chrono::Utc::now(),
            ..session
        })
    }
}
