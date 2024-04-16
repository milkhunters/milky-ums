use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().null())
                    .col(ColumnDef::new(User::LastName).string().null())
                    .col(
                        ColumnDef::new(User::State)
                            .custom(UserState::Enum)
                    )

                    .col(ColumnDef::new(User::HashedPassword).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
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

