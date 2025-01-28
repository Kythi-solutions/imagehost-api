use async_trait::async_trait;
use sea_orm::{DbErr, EntityTrait, InsertResult};

pub mod user;

#[async_trait]
pub trait Repository<T: EntityTrait> {
    async fn by_id(&self, id: i32) -> Result<Option<T::Model>, DbErr>;
    async fn create(&self, model: T::Model) -> Result<InsertResult<T::ActiveModel>, DbErr>;
}
