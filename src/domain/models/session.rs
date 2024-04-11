use chrono::{DateTime, Utc};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Session {
    pub id: String,
    pub ip: String,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
