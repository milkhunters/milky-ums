use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::user::UserId;

pub type SessionId = Uuid;
pub type SessionToken = String;
pub type SessionTokenHash = String;

pub const SESSION_TOKEN_LENGTH: usize = 128;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub token_hash: SessionTokenHash,
    pub user_id: UserId,
    
    pub ip: String,
    pub client: String,
    pub os: String,
    pub device: String,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}


impl Session {
    pub fn new(
        &self,
        token_hash: SessionTokenHash,
        user_id: Uuid,
        ip: String,
        client: String,
        os: String,
        device: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            token_hash,
            user_id,
            ip,
            client,
            os,
            device,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update(&mut self, ip: String) {
        self.ip = ip;
        self.updated_at = Some(Utc::now());
    }
}