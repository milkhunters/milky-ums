use sea_orm_migration::prelude::*;
use crate::m20240530_132130_create_service::Services;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permissions::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permissions::TextId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Permissions::ServiceId)
                        .uuid()
                        .not_null()
                    )
                     .foreign_key(
                         ForeignKey::create()
                             .from(Permissions::Table, Permissions::ServiceId)
                             .to(Services::Table, Services::Id)
                             .on_delete(ForeignKeyAction::Cascade)
                     )
                    .col(ColumnDef::new(Permissions::Title).string_len(64).unique_key().not_null())
                    .col(ColumnDef::new(Permissions::Description).string_len(255).null())
                    .col(
                        ColumnDef::new(Permissions::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Permissions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Permissions {
    Table,
    Id,
    TextId,
    ServiceId,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
}
