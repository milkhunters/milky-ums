use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type RoleId = Uuid;
pub const ROLE_TITLE_LENGTH_RANGE: (usize, usize) = (1, 64);
pub const ROLE_DESCRIPTION_LENGTH_RANGE: (usize, usize) = (1, 256);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Role {
    pub fn new(
        &self,
        title: String,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: Default::default(),
            updated_at: None,
        }
    }

    pub fn update(
        &mut self,
        title: String,
        description: Option<String>,
    ) {
        self.title = title;
        self.description = description;
        self.updated_at = Some(Utc::now());
    }
}
