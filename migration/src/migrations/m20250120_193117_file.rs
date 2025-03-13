use sea_orm_migration::{ prelude::*, schema::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(File::Table)
                .if_not_exists()
                .col(integer(File::Id).primary_key().unique_key().auto_increment())
                .col(integer(File::UserId).not_null())
                .col(string(File::Name).not_null().unique_key())
                .col(string(File::OriginalName).not_null())
                .col(string(File::Hash).not_null())
                .col(integer(File::FileSize).not_null())
                .col(string(File::FileType).not_null())
                .col(date(File::UploadedAt).not_null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(File::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum File {
    Table,
    Id,
    UserId,
    Name,
    OriginalName,
    Hash,
    FileSize,
    FileType,
    UploadedAt,
}
