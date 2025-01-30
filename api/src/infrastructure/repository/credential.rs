use async_trait::async_trait;
use entity::entities::prelude::*;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Insert, InsertResult, QueryFilter,
};

use super::Repository;

#[derive(Clone)]
pub struct CredentialRepository {
    pub database: DatabaseConnection,
}

#[async_trait]
impl Repository<Credential::Entity> for CredentialRepository {
    async fn by_id(&self, id: i32) -> Result<Option<Credential::Model>, DbErr> {
        Credential::Entity::find_by_id(id)
            .one(&self.database.to_owned())
            .await
    }

    async fn create(
        &self,
        active_model: Credential::ActiveModel,
    ) -> Result<InsertResult<Credential::ActiveModel>, DbErr> {
        Credential::Entity::insert(active_model)
            .exec(&self.database.to_owned())
            .await
    }

    fn custom_create(
        &self,
        active_model: Credential::ActiveModel,
    ) -> Insert<Credential::ActiveModel> {
        // no exec or await here since it needs to be usable with other types of executions
        // e.g: transactions
        Credential::Entity::insert(active_model)
    }
}

impl CredentialRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database: database }
    }

    pub async fn by_user_id(&self, user_id: i32) -> Result<Vec<Credential::Model>, DbErr> {
        Credential::Entity::find()
            .filter(Credential::Column::UserId.eq(user_id))
            .all(&self.database.to_owned())
            .await
    }
}
