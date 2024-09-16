use sqlx::any::AnyPoolOptions;
use sqlx::{AnyPool, Error};
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> Result<AnyPool, Error> {
  AnyPoolOptions::new()
    .acquire_timeout(Duration::from_secs(1))
    .connect(database_url)
    .await
}
