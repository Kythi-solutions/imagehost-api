use async_trait::async_trait;
use entity::entities::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use super::Repository;

pub struct UserRepository {
    database: DatabaseConnection,
}

#[async_trait]
impl Repository<User::Model> for UserRepository {
    async fn by_id(&self, id: i32) -> Result<Option<User::Model>, DbErr> {
        User::Entity::find_by_id(id)
            .one(&self.database.to_owned())
            .await
    }
}
