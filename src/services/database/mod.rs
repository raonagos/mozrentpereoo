#[cfg(feature = "server")]
mod sqlite;
mod entities;

pub use entities::*;

use crate::AppResult;
#[cfg(feature = "server")]
use axum::async_trait;

#[cfg(feature = "server")]
#[async_trait]
pub trait Database: std::fmt::Debug + Send + Sync {
    // cars
    async fn get_cars(&self) -> AppResult<Vec<Car>>;
}
