use async_trait::async_trait;
use sea_orm::{ DbErr, EntityTrait, Insert, InsertResult };

pub mod access_token;
pub mod credential;
pub mod user;

#[async_trait]
pub trait Repository<T: EntityTrait> {
    async fn by_id(&self, id: i32) -> Result<Option<T::Model>, DbErr>;
    async fn create(&self, model: T::ActiveModel) -> Result<InsertResult<T::ActiveModel>, DbErr>;
    fn custom_create(&self, model: T::ActiveModel) -> Insert<T::ActiveModel>;
}
