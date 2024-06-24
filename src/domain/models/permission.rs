use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::service::ServiceId;

pub type PermissionId = Uuid;

pub type PermissionTextId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: PermissionId,
    pub text_id: PermissionTextId,
    
    pub service_id: ServiceId,
    pub title: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Permission {
    pub fn new(
        text_id: PermissionTextId,
        service_id: ServiceId,
        title: String,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            text_id,
            service_id,
            title,
            description,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
