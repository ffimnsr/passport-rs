use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{Database, Error, Pool};
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> Result<Pool<impl Database>, Error> {

  SqlitePoolOptions::new()
    .acquire_timeout(Duration::from_secs(1))
    .connect(database_url)
    .await
}
