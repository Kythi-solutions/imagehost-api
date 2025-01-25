use async_trait::async_trait;
use sea_orm::DbErr;

pub mod user;

#[async_trait]
pub trait Repository<T> {
    async fn by_id(&self, id: i32) -> Result<Option<T>, DbErr>;
}
