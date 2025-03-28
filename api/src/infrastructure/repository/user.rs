use async_trait::async_trait;
use entity::entities::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Insert, InsertResult};

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

    async fn create(
        &self,
        active_model: User::ActiveModel,
    ) -> Result<InsertResult<User::ActiveModel>, DbErr> {
        User::Entity::insert(active_model)
            .exec(&self.database.to_owned())
            .await
    }

    fn custom_create(&self, active_model: User::ActiveModel) -> Insert<User::ActiveModel> {
        // no exec or await here since it needs to be usable with other types of executions
        // e.g: transactions
        User::Entity::insert(active_model)
    }
}

impl UserRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database: database }
    }
}
