use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserState {
    Active,
    Inactive,
    Banned,
    Deleted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: UserState,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}