pub use sea_orm_migration::prelude::*;

mod migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations::m20250120_192657_user::Migration),
            Box::new(migrations::m20250120_193033_access_token::Migration),
            Box::new(migrations::m20250120_193056_credential::Migration),
            Box::new(migrations::m20250120_193117_file::Migration),
            Box::new(migrations::m20250313_205730_deletable::Migration)
        ]
    }
}
