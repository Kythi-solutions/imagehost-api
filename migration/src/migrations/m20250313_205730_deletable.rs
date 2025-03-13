use sea_orm_migration::prelude::*;

use super::m20250120_193117_file::File;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let alter_stmt = TableAlterStatement::new()
            .table(File::Table)
            .add_column(ColumnDef::new(Alias::new("deletable")).boolean().not_null().default(true))
            .to_owned();

        manager.alter_table(alter_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(File::Table).to_owned()).await
    }
}
