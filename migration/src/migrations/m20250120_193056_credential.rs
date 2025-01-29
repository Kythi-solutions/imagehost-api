use extension::postgres::Type;
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};
use serde::{Deserialize, Serialize};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ProviderEnum)
                    .values(AuthProvider::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Credential::Table)
                    .if_not_exists()
                    .col(
                        integer(Credential::Id)
                            .primary_key()
                            .unique_key()
                            .auto_increment(),
                    )
                    .col(integer(Credential::UserId).not_null())
                    .col(string(Credential::Secret).not_null())
                    .col(
                        ColumnDef::new(Credential::Provider)
                            .enumeration(ProviderEnum, AuthProvider::iter())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credential::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Credential {
    Table,
    Id,
    UserId,
    Provider,
    Secret,
}

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Copy,
    DeriveIden,
)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum AuthProvider {
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "discord")]
    Discord,
    #[sea_orm(string_value = "github")]
    Github,
    #[sea_orm(string_value = "password")]
    Password,
}

#[derive(DeriveIden)]
struct ProviderEnum;
