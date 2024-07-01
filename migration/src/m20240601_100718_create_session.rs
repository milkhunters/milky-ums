use sea_orm_migration::prelude::*;

use crate::m20240412_063317_create_user::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Sessions::TokenHash)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Sessions::UserId)
                        .uuid()
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Sessions::Table, Sessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Sessions::Ip).string_len(15).not_null())
                    .col(
                        ColumnDef::new(Sessions::Client)
                            .string_len(128)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Sessions::Os)
                            .string_len(64)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Sessions::Device)
                            .string_len(32)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Sessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Sessions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .to_owned(),
            )
            .await
        
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    TokenHash,
    UserId,
    Ip,
    Client,
    Os,
    Device,
    CreatedAt,
    UpdatedAt,
}
