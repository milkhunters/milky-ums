use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::user::UserId;

pub type AccessLogId = Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessLog {
    pub id: AccessLogId,
    pub user_id: UserId,
    pub is_success: bool,
    
    pub ip: String,
    pub client: String,
    pub os: String,
    pub device: String,
    
    pub created_at: DateTime<Utc>,
}

impl AccessLog {
    pub fn new(
        &self,
        user_id: UserId,
        is_success: bool,
        ip: String,
        client: String,
        os: String,
        device: String,
    ) -> AccessLog {
        AccessLog {
            id: AccessLogId::new_v4(),
            user_id,
            is_success,
            ip,
            client,
            os,
            device,
            created_at: Utc::now(),
        }
    }
}
