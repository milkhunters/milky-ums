use sea_orm::entity::prelude::*;

use chrono::{DateTime, Utc};
use crate::domain::models::service::{ServiceId, ServiceTextId};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "services")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, default = "gen_random_uuid()")]
    pub id: ServiceId,
    #[sea_orm(primary_key)]
    pub text_id: ServiceTextId,
    
    pub title: String,
    pub description: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}