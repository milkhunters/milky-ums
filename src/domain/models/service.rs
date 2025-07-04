use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type ServiceId = Uuid;
pub type ServiceTextId = String;
pub const SERVICE_TITLE_LENGTH_RANGE: (usize, usize) = (1, 64);
pub const SERVICE_DESCRIPTION_LENGTH_RANGE: (usize, usize) = (1, 255);


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: ServiceId,
    pub text_id: ServiceTextId,
    pub title: String,
    pub description: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}


impl Service {

    pub fn new(
        &self,
        text_id: ServiceTextId,
        title: String,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            text_id,
            title,
            description,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update(&mut self, title: String, description: Option<String>, ) {
        self.title = title;
        self.description = description;
        self.updated_at = Some(Utc::now());
    }
}
