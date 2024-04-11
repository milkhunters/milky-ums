use sea_orm::entity::prelude::*;

use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum UserState {
    #[sea_orm(string_value = "Active")]
    Active,

    #[sea_orm(string_value = "Inactive")]
    Inactive,

    #[sea_orm(string_value = "Banned")]
    Banned,

    #[sea_orm(string_value = "Deleted")]
    Deleted,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: UserState,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}