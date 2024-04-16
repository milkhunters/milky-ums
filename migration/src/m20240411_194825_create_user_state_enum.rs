use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use crate::sea_orm::DbBackend;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(UserState::Enum)
                            .values([
                                UserState::Active,
                                UserState::Inactive,
                                UserState::Banned,
                                UserState::Deleted,
                            ])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(UserState::Enum).to_owned())
                    .await?;
            }
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_state")]
pub enum UserState {
    #[sea_orm(iden = "user_state")]
    Enum,

    #[sea_orm(string_value = "Active")]
    Active,

    #[sea_orm(string_value = "Inactive")]
    Inactive,

    #[sea_orm(string_value = "Banned")]
    Banned,

    #[sea_orm(string_value = "Deleted")]
    Deleted,
}

