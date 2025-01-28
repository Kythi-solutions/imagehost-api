use async_trait::async_trait;
use entity::entities::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, InsertResult, IntoActiveModel};

use super::Repository;

#[derive(Clone)]
pub struct UserRepository {
    pub database: DatabaseConnection,
}

#[async_trait]
impl Repository<User::Entity> for UserRepository {
    async fn by_id(&self, id: i32) -> Result<Option<User::Model>, DbErr> {
        User::Entity::find_by_id(id)
            .one(&self.database.to_owned())
            .await
    }

    async fn create(&self, model: User::Model) -> Result<InsertResult<User::ActiveModel>, DbErr> {
        User::Entity::insert(model.into_active_model())
            .exec(&self.database.to_owned())
            .await
    }
}

impl UserRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database: database }
    }
}
