use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Services::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Services::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Services::TextId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Services::Title).string_len(64).unique_key().not_null())
                    .col(ColumnDef::new(Services::Description).string_len(255).null())
                    .col(
                        ColumnDef::new(Services::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Services::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Services::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Services {
    Table,
    Id,
    TextId,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
}
