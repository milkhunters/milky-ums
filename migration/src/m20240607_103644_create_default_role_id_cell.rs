use sea_orm_migration::prelude::*;
use crate::m20240530_130156_create_role::Roles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DefaultRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DefaultRole::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_id")
                            .from(DefaultRole::Table, DefaultRole::Id)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DefaultRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DefaultRole {
    Table,
    Id,
}
