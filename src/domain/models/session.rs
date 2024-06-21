use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::user::UserId;

pub type SessionId = Uuid;
pub type SessionToken = String;
pub type SessionTokenHash = String;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub token_hash: SessionTokenHash,
    pub user_id: UserId,
    
    pub ip: String,
    pub user_agent: String,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
