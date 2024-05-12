use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::id_provider::IdProvider;
use crate::domain::models::user::UserState;

#[derive(Debug, Deserialize, Serialize)]
struct HeaderPayload {
    pub user_id: Uuid,
    pub user_state: UserState,
    pub permissions: Vec<String>,
    pub user_agent: String,
    pub ip: String,
}


struct IdHeaderProvider {
    payload: Option<HeaderPayload>,
    is_auth: bool
}


impl IdHeaderProvider {
    pub fn new(
        payload_raw: Option<String>,
    ) -> Self {
        let payload: Option<HeaderPayload> = match payload_raw {
            Some(payload_raw) => match serde_json::from_str(&payload_raw) {
                Ok(payload) => Some(payload),
                Err(_) => None
            },
            None => None
        };

        IdHeaderProvider {
            is_auth: payload.is_some(),
            payload
        }
    }
}

impl IdProvider for IdHeaderProvider {

    fn user_id(&self) -> Uuid {
        self.payload.as_ref().unwrap().user_id
    }
    
    fn user_state(&self) -> UserState {
        self.payload.as_ref().unwrap().user_state.clone()
    }

    fn permissions(&self) -> Vec<String> {
        self.payload.as_ref().unwrap().permissions.clone()
    }

    fn user_agent(&self) -> String {
        self.payload.as_ref().unwrap().user_agent.clone()
    }

    fn ip(&self) -> String {
        self.payload.as_ref().unwrap().ip.clone()
    }

    fn is_auth(&self) -> bool {
        self.is_auth
    }
}
