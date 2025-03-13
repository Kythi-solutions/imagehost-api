use async_trait::async_trait;
use entity::entities::prelude::*;
use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    Insert,
    InsertResult,
    QueryFilter,
};

use super::Repository;

#[derive(Clone)]
pub struct AccessTokenRepository {
    pub database: DatabaseConnection,
}

#[async_trait]
impl Repository<AccessToken::Entity> for AccessTokenRepository {
    async fn by_id(&self, id: i32) -> Result<Option<AccessToken::Model>, DbErr> {
        AccessToken::Entity::find_by_id(id).one(&self.database.to_owned()).await
    }

    async fn create(
        &self,
        active_model: AccessToken::ActiveModel
    ) -> Result<InsertResult<AccessToken::ActiveModel>, DbErr> {
        AccessToken::Entity::insert(active_model).exec(&self.database.to_owned()).await
    }

    fn custom_create(
        &self,
        active_model: AccessToken::ActiveModel
    ) -> Insert<AccessToken::ActiveModel> {
        // no exec or await here since it needs to be usable with other types of executions
        // e.g: transactions
        AccessToken::Entity::insert(active_model)
    }
}

impl AccessTokenRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database: database }
    }

    pub async fn find_by_token(&self, token: String) -> Result<Option<AccessToken::Model>, DbErr> {
        AccessToken::Entity::find()
            .filter(AccessToken::Column::Token.eq(token))
            .one(&self.database.to_owned()).await
    }
}
