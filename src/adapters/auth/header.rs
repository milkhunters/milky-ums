use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::id_provider::IdProvider;
use crate::domain::models::session::SessionId;
use crate::domain::models::user::UserState;

#[derive(Debug, Deserialize, Serialize)]
struct HeaderPayload {
    pub user_id: Uuid,
    pub user_state: UserState,  
    pub permissions: Vec<String>,
}

pub struct IdHeaderProvider {
    user_id: Option<Uuid>,
    session_id: Option<SessionId>,
    user_state: Option<UserState>,
    permissions: Vec<String>,
    user_agent: String,
    ip: String,
    is_auth: bool
}


impl IdHeaderProvider {
    pub fn new(
        session_id: Option<SessionId>,
        payload_raw: Option<String>,
        user_agent: String,
        ip: String,
    ) -> Self {
        let payload: Option<HeaderPayload> = match payload_raw {
            Some(payload_raw) => serde_json::from_str(&payload_raw).unwrap_or_else(|_| None),
            None => None
        };
        
        match payload {
            Some(payload) => Self {
                user_id: Some(payload.user_id),
                session_id,
                user_state: Some(payload.user_state),
                permissions: payload.permissions,
                user_agent,
                ip,
                is_auth: true
            },
            None => Self {
                user_id: None,
                session_id: None,
                user_state: None,
                permissions: vec!["CreateUser".parse().unwrap()],
                user_agent,
                ip,
                is_auth: false
            }
        }
    }
}

impl IdProvider for IdHeaderProvider {
    fn session_id(&self) -> Option<&SessionId> {
        match &self.session_id {
            Some(session_id) => Some(session_id),
            None => None
        }
    }

    fn user_id(&self) -> Option<&Uuid> {
        match &self.user_id {
            Some(user_id) => Some(user_id),
            None => None
        }
    }
    
    fn user_state(&self) -> Option<&UserState> {
        match &self.user_state {
            Some(user_state) => Some(user_state),
            None => None
        }
    }

    fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    fn user_agent(&self) -> &str {
        &self.user_agent
    }

    fn ip(&self) -> &str {
        &self.ip
    }

    fn is_auth(&self) -> &bool {
        &self.is_auth
    }
}
