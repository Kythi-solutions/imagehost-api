use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccessToken::Table)
                    .if_not_exists()
                    .col(
                        integer(AccessToken::Id)
                            .unique_key()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(integer(AccessToken::UserId))
                    .col(string(AccessToken::Name).not_null())
                    .col(string(AccessToken::Description))
                    .col(string(AccessToken::Token).unique_key().not_null())
                    .col(string(AccessToken::Access).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccessToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AccessToken {
    Table,
    Id,
    UserId,
    Name,
    Description,
    Token,
    Access,
}
