use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(1))
        .connect(database_url)
        .await
}
