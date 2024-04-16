pub use sea_orm_migration::prelude::*;

mod m20240411_194825_create_user_state_enum;
mod m20240412_063317_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240411_194825_create_user_state_enum::Migration),
            Box::new(m20240412_063317_create_user::Migration),
        ]
    }
}
