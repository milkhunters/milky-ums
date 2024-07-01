use sea_orm_migration::prelude::*;

use crate::m20240412_063317_create_user::Users;
use crate::m20240530_132130_create_service::Services;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccessLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccessLogs::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AccessLogs::IsSuccess)
                        .boolean()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(AccessLogs::Client)
                            .string_len(128)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(AccessLogs::Os)
                            .string_len(64)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(AccessLogs::Device)
                            .string_len(32)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(AccessLogs::Ip)
                            .string_len(15)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(AccessLogs::UserId)
                            .uuid()
                            .null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AccessLogs::Table, AccessLogs::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(
                        ColumnDef::new(AccessLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccessLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccessLogs {
    Table,
    Id,
    IsSuccess,
    Client,
    Os,
    Device,
    Ip,
    UserId,
    CreatedAt,
}
