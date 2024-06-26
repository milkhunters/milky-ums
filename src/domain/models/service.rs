use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ServiceId = Uuid;

pub type ServiceTextId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: ServiceId,
    pub text_id: ServiceTextId,
    pub title: String,
    pub description: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
