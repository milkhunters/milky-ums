pub use sea_orm_migration::prelude::*;

mod m20240411_194825_create_user_state_enum;
mod m20240412_063317_create_user;
mod m20240530_130156_create_role;
mod m20240530_133643_create_permission;
mod m20240530_132130_create_service;
mod m20240530_134552_create_m2m_role_permission;
mod m20240530_141709_create_m2m_role_user;
mod m20240601_100718_create_session;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240411_194825_create_user_state_enum::Migration),
            Box::new(m20240412_063317_create_user::Migration),
            Box::new(m20240530_130156_create_role::Migration),
            Box::new(m20240530_132130_create_service::Migration),
            Box::new(m20240530_133643_create_permission::Migration),
            Box::new(m20240530_134552_create_m2m_role_permission::Migration),
            Box::new(m20240530_141709_create_m2m_role_user::Migration),
            Box::new(m20240601_100718_create_session::Migration),
        ]
    }
}
