//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::UserState;
use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: UserState,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_user::Entity")]
    RoleUser,
    #[sea_orm(has_many = "super::sessions::Entity")]
    Sessions,
}

impl Related<super::role_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleUser.def()
    }
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_user::Relation::Roles.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_user::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}