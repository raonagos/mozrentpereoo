use super::*;
use crate::AppResult;
use axum::async_trait;
use sqlx::{Database as SqlxDatabase, Pool};

#[async_trait]
impl<DB: SqlxDatabase> Database for Pool<DB> {
    async fn get_cars(&self) -> AppResult<Vec<Car>> {
        _ = self.acquire().await.expect("");
        Ok(vec![Car::default()])
    }
}
