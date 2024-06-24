use sea_orm_migration::prelude::*;
use crate::m20240412_063317_create_user::Users;
use crate::m20240530_130156_create_role::Roles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleUser::UserId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(RoleUser::RoleId)
                            .uuid()
                            .not_null()
                    )
                    .primary_key(
                        Index::create()
                            .table(RoleUser::Table)
                            .col(RoleUser::RoleId)
                            .col(RoleUser::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RoleUser::Table, RoleUser::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RoleUser::Table, RoleUser::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleUser {
    Table,
    UserId,
    RoleId
}
