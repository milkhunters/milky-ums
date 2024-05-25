use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type SessionId = String;

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub id: SessionId,
    pub user_id: Uuid,
    pub ip: String,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
