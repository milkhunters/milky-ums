use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string_len(32).unique_key().not_null())
                    .col(ColumnDef::new(Users::Email).string_len(255).unique_key().not_null())
                    .col(ColumnDef::new(Users::FirstName).string_len(64).null())
                    .col(ColumnDef::new(Users::LastName).string_len(64).null())
                    .col(
                        ColumnDef::new(Users::State)
                            .custom(UserState::Enum)
                    )

                    .col(ColumnDef::new(Users::HashedPassword).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    FirstName,
    LastName,
    State,
    HashedPassword,
    CreatedAt,
    UpdatedAt,
}


#[derive(DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_state")]
pub enum UserState {
    #[sea_orm(iden = "user_state")]
    Enum,
}

