use std::collections::HashMap;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::application::common::id_provider::IdProvider;
use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::service::ServiceTextId;
use crate::domain::models::session::SessionId;
use crate::domain::models::user::{UserId, UserState};

#[derive(Debug, Deserialize, Serialize)]
struct HeaderPayload {
    pub session_id: SessionId,
    pub user_id: UserId,
    pub user_state: UserState,  
    pub permissions: HashMap<ServiceTextId, Vec<PermissionTextId>>
}

pub struct IdHeaderProvider {
    user_id: Option<UserId>,
    session_id: Option<SessionId>,
    user_state: Option<UserState>,
    permissions: Vec<PermissionTextId>,
    user_agent: String,
    ip: String,
    is_auth: bool
}


impl IdHeaderProvider {
    pub fn new(
        service_name: &str,
        payload_raw: Option<String>,
        user_agent: &str,
        ip: &str
    ) -> Self {
        let payload: Option<HeaderPayload> = match payload_raw {
            Some(payload_raw) => serde_json::from_str(&payload_raw).ok(),
            None => None
        };

        
        match payload {
            Some(payload) => Self {
                user_id: Some(payload.user_id),
                session_id: Some(payload.session_id),
                user_state: Some(payload.user_state),
                permissions: match payload.permissions.get(service_name) {
                    Some(permissions) => permissions.to_owned(),
                    None => {
                        warn!(
                            "Permissions not found for service: {}, user_id: {}",
                            payload.user_id,
                            service_name
                        );
                        vec![]
                    }
                },
                user_agent: user_agent.to_string(),
                ip: ip.to_string(),
                is_auth: true
            },
            None => Self {
                user_id: None,
                session_id: None,
                user_state: None,
                permissions: vec![
                    "CreateUser".parse().unwrap(), 
                    "CreateSession".parse().unwrap()
                ],
                user_agent: user_agent.to_string(),
                ip: ip.to_string(),
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

    fn user_id(&self) -> Option<&UserId> {
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
